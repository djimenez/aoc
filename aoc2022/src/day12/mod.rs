use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);

    find_path(&map).unwrap()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    find_closest_start(&map).unwrap()
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    start: Coord,
    end: Coord,

    elevations: Vec<Vec<char>>,
}

fn parse_input(input: &str) -> Map {
    let mut elevations: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start: Coord = Coord { x: 0, y: 0 };
    let mut end: Coord = Coord { x: 0, y: 0 };

    for (y, row) in elevations.iter().enumerate() {
        for (x, chr) in row.iter().enumerate() {
            if *chr == 'S' {
                start = Coord { x, y };
            }

            if *chr == 'E' {
                end = Coord { x, y };
            }
        }
    }

    // convert S -> a, E -> z
    elevations[start.y][start.x] = 'a';
    elevations[end.y][end.x] = 'z';

    Map {
        start,
        end,

        elevations,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Candidate {
    coord: Coord,
    steps: usize,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.steps.cmp(&self.steps))
    }
}

fn find_path(map: &Map) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    //let mut distances = HashMap::new();
    let mut visited = HashSet::new();

    heap.push(Candidate {
        coord: map.start,
        steps: 0,
    });

    while let Some(candidate) = heap.pop() {
        if candidate.coord == map.end {
            return Some(candidate.steps);
        }

        // generate new candidates from this one
        let Coord { x, y } = candidate.coord;

        let current_elevation = map.elevations[y][x];

        if candidate.coord.y > 0 {
            let north_coord = Coord { x, y: y - 1 };

            if !visited.contains(&north_coord) {
                let north_elevation = map.elevations[y - 1][x];

                if current_elevation as u32 + 1 >= north_elevation as u32 {
                    visited.insert(north_coord);
                    heap.push(Candidate {
                        coord: north_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }

        if candidate.coord.y < map.elevations.len() - 1 {
            let south_coord = Coord { x, y: y + 1 };

            if !visited.contains(&south_coord) {
                let south_elevation = map.elevations[y + 1][x];

                if current_elevation as u32 + 1 >= south_elevation as u32 {
                    visited.insert(south_coord);
                    heap.push(Candidate {
                        coord: south_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }

        if candidate.coord.x > 0 {
            let west_coord = Coord { x: x - 1, y };

            if !visited.contains(&west_coord) {
                let west_elevation = map.elevations[y][x - 1];

                if current_elevation as u32 + 1 >= west_elevation as u32 {
                    visited.insert(west_coord);
                    heap.push(Candidate {
                        coord: west_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }

        if x < map.elevations[y].len() - 1 {
            let east_coord = Coord { x: x + 1, y };

            if !visited.contains(&east_coord) {
                let east_elevation = map.elevations[y][x + 1];

                if current_elevation as u32 + 1 >= east_elevation as u32 {
                    visited.insert(east_coord);
                    heap.push(Candidate {
                        coord: east_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }
    }

    // output debug if we didn't find anything
    for (y, row) in map.elevations.iter().enumerate() {
        let line: String = row
            .iter()
            .enumerate()
            .map(|(x, chr)| {
                let coord = Coord { x, y };

                if visited.contains(&coord) {
                    '#'
                } else {
                    *chr
                }
            })
            .collect();

        println!("{}", line);
    }

    None
}

fn find_closest_start(map: &Map) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    //let mut distances = HashMap::new();
    let mut visited = HashSet::new();

    heap.push(Candidate {
        coord: map.end,
        steps: 0,
    });

    while let Some(candidate) = heap.pop() {
        // generate new candidates from this one
        let Coord { x, y } = candidate.coord;
        let current_elevation = map.elevations[y][x];

        if current_elevation == 'a' {
            return Some(candidate.steps);
        }

        if candidate.coord.y > 0 {
            let north_coord = Coord { x, y: y - 1 };

            if !visited.contains(&north_coord) {
                let north_elevation = map.elevations[y - 1][x];

                if current_elevation as u32 <= 1 + north_elevation as u32 {
                    visited.insert(north_coord);
                    heap.push(Candidate {
                        coord: north_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }

        if candidate.coord.y < map.elevations.len() - 1 {
            let south_coord = Coord { x, y: y + 1 };

            if !visited.contains(&south_coord) {
                let south_elevation = map.elevations[y + 1][x];

                if current_elevation as u32 <= 1 + south_elevation as u32 {
                    visited.insert(south_coord);
                    heap.push(Candidate {
                        coord: south_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }

        if candidate.coord.x > 0 {
            let west_coord = Coord { x: x - 1, y };

            if !visited.contains(&west_coord) {
                let west_elevation = map.elevations[y][x - 1];

                if current_elevation as u32 <= 1 + west_elevation as u32 {
                    visited.insert(west_coord);
                    heap.push(Candidate {
                        coord: west_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }

        if x < map.elevations[y].len() - 1 {
            let east_coord = Coord { x: x + 1, y };

            if !visited.contains(&east_coord) {
                let east_elevation = map.elevations[y][x + 1];

                if current_elevation as u32 <= 1 + east_elevation as u32 {
                    visited.insert(east_coord);
                    heap.push(Candidate {
                        coord: east_coord,
                        steps: candidate.steps + 1,
                    });
                }
            }
        }
    }

    // output debug if we didn't find anything
    for (y, row) in map.elevations.iter().enumerate() {
        let line: String = row
            .iter()
            .enumerate()
            .map(|(x, chr)| {
                let coord = Coord { x, y };

                if visited.contains(&coord) {
                    '#'
                } else {
                    *chr
                }
            })
            .collect();

        println!("{}", line);
    }

    None
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29)
    }
}
