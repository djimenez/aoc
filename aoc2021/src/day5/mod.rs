use std::cmp::max;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    let mut points: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        if line.square() {
            for point in line.points() {
                let count = points.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    debug_points(&points);

    points.into_values().filter(|count| *count > 1).count()
}

fn part2(input: &str) -> usize {
    let lines = parse_input(input);
    let mut points: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        for point in line.points() {
            let count = points.entry(point).or_insert(0);
            *count += 1;
        }
    }

    debug_points(&points);

    points.into_values().filter(|count| *count > 1).count()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect()
}

fn debug_points(points: &HashMap<Point, usize>) {
    println!("");

    for y in 0..10 {
        for x in 0..10 {
            let point = &Point { x, y };
            let default = &0;
            let count = points.get(point).unwrap_or(default);

            print!("{} ", count);
        }

        println!("");
    }
}

#[derive(Debug, Default)]
struct Line {
    x1: u64,
    x2: u64,
    y1: u64,
    y2: u64,
}

impl Line {
    fn square(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn points(&self) -> Vec<Point> {
        let x_diff = if self.x1 > self.x2 {
            self.x1 - self.x2
        } else {
            self.x2 - self.x1
        };
        let y_diff = if self.y1 > self.y2 {
            self.y1 - self.y2
        } else {
            self.y2 - self.y1
        };
        let steps = max(x_diff, y_diff) as usize;

        let mut points = Vec::with_capacity(steps);
        let mut x = self.x1;
        let mut y = self.y1;

        points.push(Point { x, y });

        for _ in 0..steps {
            if self.x1 < self.x2 {
                x += 1;
            } else if self.x1 > self.x2 {
                x -= 1;
            }

            if self.y1 < self.y2 {
                y += 1;
            } else if self.y1 > self.y2 {
                y -= 1;
            }

            points.push(Point { x, y });
        }

        points
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(str_value: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = str_value
            .trim()
            .split(" -> ")
            .flat_map(|point| point.split(','))
            .collect();

        let x1 = coords[0].parse()?;
        let y1 = coords[1].parse()?;
        let x2 = coords[2].parse()?;
        let y2 = coords[3].parse()?;

        Ok(Self { x1, x2, y1, y2 })
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 12)
    }

    #[test]
    fn test_parse_input() {
        let lines = parse_input(TEST_INPUT);
        let square_lines: Vec<&Line> = lines.iter().filter(|line| line.square()).collect();

        assert_eq!(lines.len(), 10);
        assert_eq!(square_lines.len(), 6);
    }
}
