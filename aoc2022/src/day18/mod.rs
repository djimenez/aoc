use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let coords = parse_input(input);
    let locations: HashSet<&Coord> = coords.iter().collect();

    let mut sides = 0;

    for Coord { x, y, z } in &coords {
        if !locations.contains(&Coord {
            x: *x - 1,
            y: *y,
            z: *z,
        }) {
            sides += 1;
        }
        if !locations.contains(&Coord {
            x: *x + 1,
            y: *y,
            z: *z,
        }) {
            sides += 1;
        }
        if !locations.contains(&Coord {
            x: *x,
            y: *y - 1,
            z: *z,
        }) {
            sides += 1;
        }
        if !locations.contains(&Coord {
            x: *x,
            y: *y + 1,
            z: *z,
        }) {
            sides += 1;
        }
        if !locations.contains(&Coord {
            x: *x,
            y: *y,
            z: *z - 1,
        }) {
            sides += 1;
        }
        if !locations.contains(&Coord {
            x: *x,
            y: *y,
            z: *z + 1,
        }) {
            sides += 1;
        }
    }

    sides
}

fn part2(input: &str) -> u64 {
    let coords = parse_input(input);

    let mut x_min: i64 = i64::MAX;
    let mut x_max: i64 = i64::MIN;
    let mut y_min: i64 = i64::MAX;
    let mut y_max: i64 = i64::MIN;
    let mut z_min: i64 = i64::MAX;
    let mut z_max: i64 = i64::MIN;

    for Coord { x, y, z } in &coords {
        if x - 1 < x_min {
            x_min = x - 1;
        }

        if x + 1 > x_max {
            x_max = x + 1;
        }

        if y - 1 < y_min {
            y_min = y - 1;
        }

        if y + 1 > y_max {
            y_max = y + 1;
        }

        if z - 1 < z_min {
            z_min = z - 1;
        }

        if z + 1 > z_max {
            z_max = z + 1;
        }
    }

    let lava: HashSet<Coord> = coords.into_iter().collect();
    let mut external_space = HashMap::new();

    // flood fill external space
    let mut fill_queue = VecDeque::new();
    fill_queue.push_back(Coord {
        x: x_min,
        y: y_min,
        z: z_min,
    });

    while fill_queue.len() > 0 {
        let coord = fill_queue.pop_front().unwrap();

        // if we've already assessed this coord, skip
        if external_space.contains_key(&coord) {
            continue;
        }

        // if coord is lava, mark as negatively assessed
        if lava.contains(&coord) {
            external_space.insert(coord, false);
        } else {
            external_space.insert(coord, true);

            // also push neighbors
            let Coord { x, y, z } = coord;

            if x > x_min {
                fill_queue.push_back(Coord { x: x - 1, y, z });
            }

            if x < x_max {
                fill_queue.push_back(Coord { x: x + 1, y, z });
            }

            if y > y_min {
                fill_queue.push_back(Coord { x, y: y - 1, z });
            }

            if y < y_max {
                fill_queue.push_back(Coord { x, y: y + 1, z });
            }

            if z > z_min {
                fill_queue.push_back(Coord { x, y, z: z - 1 });
            }

            if z < z_max {
                fill_queue.push_back(Coord { x, y, z: z + 1 });
            }
        }
    }

    let mut sides = 0;

    for Coord { x, y, z } in &lava {
        let left = Coord {
            x: *x - 1,
            y: *y,
            z: *z,
        };
        let right = Coord {
            x: *x + 1,
            y: *y,
            z: *z,
        };
        let above = Coord {
            x: *x,
            y: *y + 1,
            z: *z,
        };
        let below = Coord {
            x: *x,
            y: *y - 1,
            z: *z,
        };
        let before = Coord {
            x: *x,
            y: *y,
            z: *z - 1,
        };
        let after = Coord {
            x: *x,
            y: *y,
            z: *z + 1,
        };

        if !lava.contains(&left) && *external_space.get(&left).unwrap_or(&false) {
            sides += 1;
        }
        if !lava.contains(&right) && *external_space.get(&right).unwrap_or(&false) {
            sides += 1;
        }
        if !lava.contains(&above) && *external_space.get(&above).unwrap_or(&false) {
            sides += 1;
        }
        if !lava.contains(&below) && *external_space.get(&below).unwrap_or(&false) {
            sides += 1;
        }
        if !lava.contains(&before) && *external_space.get(&before).unwrap_or(&false) {
            sides += 1;
        }
        if !lava.contains(&after) && *external_space.get(&after).unwrap_or(&false) {
            sides += 1;
        }
    }

    sides
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(",");

            Coord {
                x: split.next().unwrap().parse().unwrap(),
                y: split.next().unwrap().parse().unwrap(),
                z: split.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 64)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 58)
    }
}
