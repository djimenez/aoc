use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let lines = parse_input(input);
    let mut space = HashMap::new();
    let mut max_y = 0;

    // place the lines into the space
    for Line { x0, y0, x1, y1 } in lines {
        for x in x0..=x1 {
            for y in y0..=y1 {
                let coord = Coord { x, y };
                space.insert(coord, '#');

                max_y = i64::max(max_y, y);
            }
        }
    }

    let mut sand_grains = 0;

    loop {
        // simulate a grain of sand
        let mut sand = Coord { x: 500, y: 0 };

        loop {
            // try straight down
            sand.y += 1;

            if sand.y > max_y {
                return sand_grains;
            }

            if !space.contains_key(&sand) {
                continue;
            }

            // try down and left
            sand.x -= 1;

            if !space.contains_key(&sand) {
                continue;
            }

            // try down and right
            sand.x += 2;

            if !space.contains_key(&sand) {
                continue;
            }

            // reset sand and insert into space as rested
            sand.x -= 1;
            sand.y -= 1;

            space.insert(sand, 'o');
            sand_grains += 1;
            break;
        }
    }
}

fn part2(input: &str) -> u64 {
    let lines = parse_input(input);
    let mut space = HashMap::new();
    let mut max_y = 0;

    // place the lines into the space
    for Line { x0, y0, x1, y1 } in lines {
        for x in x0..=x1 {
            for y in y0..=y1 {
                let coord = Coord { x, y };
                space.insert(coord, '#');

                max_y = i64::max(max_y, y);
            }
        }
    }

    // floor is at max_y + 2
    max_y += 2;

    let mut sand_grains = 0;

    loop {
        // simulate a grain of sand
        let mut sand = Coord { x: 500, y: 0 };

        if space.contains_key(&sand) {
            return sand_grains;
        }

        loop {
            // try straight down
            sand.y += 1;

            if sand.y < max_y {
                if !space.contains_key(&sand) {
                    continue;
                }

                // try down and left
                sand.x -= 1;

                if !space.contains_key(&sand) {
                    continue;
                }

                // try down and right
                sand.x += 2;

                if !space.contains_key(&sand) {
                    continue;
                }

                sand.x -= 1;
            }

            // reset sand and insert into space as rested
            sand.y -= 1;

            space.insert(sand, 'o');
            sand_grains += 1;
            break;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i64,
    y: i64,
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split(',');

        Ok(Coord {
            x: split.next().unwrap().parse::<i64>()?,
            y: split.next().unwrap().parse::<i64>()?,
        })
    }
}

#[derive(Debug)]
struct Line {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .collect::<Vec<_>>()
                .windows(2)
                .map(|window| {
                    let left = window[0].parse::<Coord>().unwrap();
                    let right = window[1].parse::<Coord>().unwrap();

                    if left <= right {
                        Line {
                            x0: left.x,
                            y0: left.y,
                            x1: right.x,
                            y1: right.y,
                        }
                    } else {
                        Line {
                            x0: right.x,
                            y0: right.y,
                            x1: left.x,
                            y1: left.y,
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93)
    }
}
