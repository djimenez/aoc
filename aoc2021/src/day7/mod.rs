const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let positions = parse_input(input);

    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    let mut min_target = min_pos;
    let mut min_fuel = u64::MAX;

    for target in min_pos..=max_pos {
        let mut fuel = 0;

        for pos in &positions {
            fuel += if target < *pos {
                *pos - target
            } else {
                target - *pos
            };
        }

        if fuel < min_fuel {
            min_target = target;
            min_fuel = fuel;
        }
    }

    println!("min target: {} for {}", min_target, min_fuel);

    min_fuel
}

fn part2(input: &str) -> u64 {
    let positions = parse_input(input);

    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    let mut min_target = min_pos;
    let mut min_fuel = u64::MAX;

    for target in min_pos..=max_pos {
        let mut fuel = 0;

        for pos in &positions {
            let diff = if target < *pos {
                *pos - target
            } else {
                target - *pos
            };

            fuel += diff * (diff + 1) / 2;
        }

        if fuel < min_fuel {
            min_target = target;
            min_fuel = fuel;
        }
    }

    println!("min target: {} for {}", min_target, min_fuel);

    min_fuel
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 37)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 168)
    }

    #[test]
    fn test_parse_input() {
        let positions = parse_input(TEST_INPUT);

        assert_eq!(positions.len(), 10)
    }
}
