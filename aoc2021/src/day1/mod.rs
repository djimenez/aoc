const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let depths = parse_input(input);
    let mut increases: u64 = 0;

    // window each pair of depths
    for i in 0..depths.len() - 1 {
        if depths[i] < depths[i + 1] {
            increases += 1;
        }
    }

    increases
}

fn part2(input: &str) -> u64 {
    let depths = parse_input(input);
    let mut increases: u64 = 0;

    // window each quadruple of depths
    for i in 0..depths.len() - 3 {
        if depths[i] < depths[i + 3] {
            increases += 1;
        }
    }

    increases
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim_end().parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5)
    }
}
