use regex::Regex;
use std::collections::HashSet;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT, 2000000));
    println!("part2: {}", part2(DEFAULT_INPUT, 4000000));
}

fn part1(input: &str, y_target: i64) -> usize {
    let placements = parse_input(input);
    let mut ranges = Vec::new();
    let mut exclusions = HashSet::new();

    for (sensor, beacon, dist) in placements {
        let y_dist = (sensor.y - y_target).abs();

        // if the sensor area covers the y target, add its ranges
        if y_dist <= dist {
            let x_dist = dist - y_dist;
            ranges.push(sensor.x - x_dist..=sensor.x + x_dist);
        }

        // if the sensor or beacon is on the y target, count it as an exclusion
        if sensor.y == y_target {
            exclusions.insert(sensor);
        }

        if beacon.y == y_target {
            exclusions.insert(beacon);
        }
    }

    // sort the ranges - hilarious that Ord isn't implemented for RangeInclusive but i can do this with tuples
    ranges.sort_by(|a, b| (a.start(), a.end()).cmp(&(b.start(), b.end())));

    let (count, _) = ranges
        .iter()
        // because we sorted we know each next start is >= previous start
        .fold((0, &i64::MIN), |(count, last_end), range| {
            if range.start() > last_end {
                // this range if fully uncovered by last range
                (range.end() - range.start() + 1, range.end())
            } else if range.end() > last_end {
                // this range was partially contained in the last range
                (range.end() - last_end + count, range.end())
            } else {
                // this range was fully contained in the last range, ignore it
                (count, last_end)
            }
        });

    count as usize - exclusions.len()
}

fn part2(input: &str, search_area: i64) -> i64 {
    let mut placements = parse_input(input);

    // sort placements by x ascending so that as we iterate them we're skipping ahead correctly
    placements.sort_by(|(sensor_a, _, _), (sensor_b, _, _)| sensor_a.x.cmp(&sensor_b.x));

    // skipping ahead nicely, but still seems naive
    for y in 0..=search_area {
        let mut x = 0;

        for (sensor, _, dist) in &placements {
            // skip ahead to outside the sensor's area if we're in it
            if (sensor.x - x).abs() + (sensor.y - y).abs() <= *dist {
                x = sensor.x + *dist - (sensor.y - y).abs() + 1;
            }
        }

        if x <= search_area {
            return x * 4000000 + y;
        }
    }

    panic!("found no solution");
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_input(input: &str) -> Vec<(Coord, Coord, i64)> {
    let re =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            let sensor = Coord {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
            };

            let beacon = Coord {
                x: captures[3].parse().unwrap(),
                y: captures[4].parse().unwrap(),
            };

            let manhattan_dist = sensor.manhattan_dist(&beacon);

            (sensor, beacon, manhattan_dist)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 20), 56000011)
    }
}
