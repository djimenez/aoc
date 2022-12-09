use std::collections::HashSet;
use std::str::FromStr;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    let moves = parse_input(input);

    let mut locations = HashSet::new();
    let mut rope = Rope::new(2);

    for m in moves {
        for _ in 0..m.distance {
            rope.step(&m.direction);
            locations.insert(rope.tail());
        }
    }

    locations.len()
}

fn part2(input: &str) -> usize {
    let moves = parse_input(input);

    let mut locations = HashSet::new();
    let mut rope = Rope::new(10);

    for m in moves {
        for _ in 0..m.distance {
            rope.step(&m.direction);
            locations.insert(rope.tail());
        }
    }

    locations.len()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            knots: (0..size).map(|_| Point { x: 0, y: 0 }).collect(),
        }
    }

    fn tail(&self) -> Point {
        self.knots.last().unwrap().clone()
    }

    fn step(&mut self, direction: &Direction) {
        let hdx;
        let hdy;

        match direction {
            Direction::L => {
                hdx = -1;
                hdy = 0;
            }

            Direction::R => {
                hdx = 1;
                hdy = 0;
            }

            Direction::U => {
                hdx = 0;
                hdy = 1;
            }

            Direction::D => {
                hdx = 0;
                hdy = -1;
            }
        }

        // move the head by hdx, hdy
        self.knots[0].x += hdx;
        self.knots[0].y += hdy;

        // propagate the movement to all other knots
        for i in 0..self.knots.len() - 1 {
            let head = &self.knots[i];
            let tail = &self.knots[i + 1];

            let mut dx = head.x - tail.x;
            let mut dy = head.y - tail.y;

            if dx.abs() > 1 || dy.abs() > 1 {
                if dx < -1 {
                    dx = -1;
                } else if dx > 1 {
                    dx = 1;
                }

                if dy < -1 {
                    dy = -1;
                } else if dy > 1 {
                    dy = 1;
                }

                let mut tail = &mut self.knots[i + 1];

                tail.x += dx;
                tail.y += dy;
            }
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: usize,
}

#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        let mut split = input.split(" ");

        let direction = match split.next().ok_or("missing direction")? {
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),

            _ => Err("direction did not match R|L|U|D"),
        }?;

        let distance = split
            .next()
            .ok_or("missing distance")?
            .parse()
            .or(Err("error parsing distance"))?;

        Ok(Move {
            direction,
            distance,
        })
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");
    const TEST_INPUT_2: &str = include_str!("test2");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1);
        assert_eq!(part2(TEST_INPUT_2), 36);
    }
}
