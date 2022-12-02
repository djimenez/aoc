const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let inventory = parse_input(input);

    // max of inventory group sums
    inventory
        .into_iter()
        .map(|group| group.iter().sum())
        .max()
        .unwrap_or_default()
}

fn part2(input: &str) -> u64 {
    let inventory = parse_input(input);

    let mut totals: Vec<u64> = inventory.iter().map(|group| group.iter().sum()).collect();

    // sort descending
    totals.sort_by(|a, b| b.cmp(a));

    // sum the first 3
    let max_sum = totals[..3].iter().sum();

    return max_sum;
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\r\n\r\n")
        .map(|linegroup| {
            linegroup
                .lines()
                .map(|line| line.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24000)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45000)
    }
}
