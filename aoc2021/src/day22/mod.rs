use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    let operations = parse_input(input);

    // use hash set as a sparse array
    let mut state: HashSet<Point> = HashSet::new();
    let init_limit = Cuboid {
        xmin: -50,
        xmax: 50,
        ymin: -50,
        ymax: 50,
        zmin: -50,
        zmax: 50,
    };

    for operation in operations {
        if let Some(intersection) = operation.cuboid.intersection(&init_limit) {
            for point in &intersection {
                if operation.on {
                    state.insert(point);
                } else {
                    state.remove(&point);
                }
            }
        }
    }

    state.len()
}

fn part2(input: &str) -> usize {
    let operations = parse_input(input);

    // part 2 has too many points for a hashset, move to keeping list of on cuboids, fracture / combine cuboids for intersections
    let mut state: Vec<Cuboid> = Vec::new();

    for operation in operations {
        // check new cuboid for intersection with existing cuboids
        for existing in &state {
            if let Some(intersection) = existing.intersection(&operation.cuboid) {
                println!("intersection: {:?}", intersection);
            }
        }

        if operation.on {
            state.push(operation.cuboid);
        }
    }

    state.iter().map(|cube| cube.count() as usize).sum()
}

fn parse_input(input: &str) -> Vec<Operation> {
    let re = Regex::new(r"^(on|off) x=(\-?[0-9]+)..(\-?[0-9]+),y=(\-?[0-9]+)..(\-?[0-9]+),z=(\-?[0-9]+)..(\-?[0-9]+)").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            Operation {
                on: captures.get(1).unwrap().as_str() == "on",
                cuboid: Cuboid {
                    xmin: captures.get(2).unwrap().as_str().parse().unwrap(),
                    xmax: captures.get(3).unwrap().as_str().parse().unwrap(),
                    ymin: captures.get(4).unwrap().as_str().parse().unwrap(),
                    ymax: captures.get(5).unwrap().as_str().parse().unwrap(),
                    zmin: captures.get(6).unwrap().as_str().parse().unwrap(),
                    zmax: captures.get(7).unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .collect()
}

#[derive(Debug)]
struct Operation {
    on: bool,
    cuboid: Cuboid,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Cuboid {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

impl Cuboid {
    fn count(&self) -> u64 {
        let xlen = (self.xmax - self.xmin + 1) as u64;
        let ylen = (self.ymax - self.ymin + 1) as u64;
        let zlen = (self.zmax - self.zmin + 1) as u64;

        xlen * ylen * zlen
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let xmin = max(self.xmin, other.xmin);
        let xmax = min(self.xmax, other.xmax);
        let ymin = max(self.ymin, other.ymin);
        let ymax = min(self.ymax, other.ymax);
        let zmin = max(self.zmin, other.zmin);
        let zmax = min(self.zmax, other.zmax);

        if xmin <= xmax && ymin <= ymax && zmin <= zmax {
            return Some(Cuboid {
                xmin,
                xmax,
                ymin,
                ymax,
                zmin,
                zmax,
            });
        }

        None
    }
}

struct CuboidIterator {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,

    x: i64,
    y: i64,
    z: i64,
}

impl Iterator for CuboidIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.xmax {
            return None;
        }

        let point = Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };

        if self.z < self.zmax {
            self.z += 1;
        } else {
            self.z = self.zmin;

            if self.y < self.ymax {
                self.y += 1;
            } else {
                self.y = self.ymin;
                self.x += 1;
            }
        }

        return Some(point);
    }
}

impl IntoIterator for &Cuboid {
    type Item = Point;
    type IntoIter = CuboidIterator;

    fn into_iter(self) -> Self::IntoIter {
        CuboidIterator {
            xmin: self.xmin,
            xmax: self.xmax,
            ymin: self.ymin,
            ymax: self.ymax,
            zmin: self.zmin,
            zmax: self.zmax,

            x: self.xmin,
            y: self.ymin,
            z: self.zmin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");
    const TEST2_INPUT: &str = include_str!("test2");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 590784)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST2_INPUT), 2758514936282235)
    }
}
