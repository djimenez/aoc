const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let values = parse_input(input);

    values.iter().sum()
}

fn part2(input: &str) -> u64 {
    let values = parse_input(input);

    values.iter().sum()
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.trim_end())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0)
    }
}
