const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let pairs = parse_input(input);
    let mut covered_count = 0;

    for pair in pairs {
        // we assume that start <= end for all ranges
        if pair.first_start >= pair.second_start && pair.first_end <= pair.second_end
            || pair.first_start <= pair.second_start && pair.first_end >= pair.second_end
        {
            covered_count += 1;
        }
    }

    covered_count
}

fn part2(input: &str) -> u64 {
    let pairs = parse_input(input);
    let mut overlap_count = 0;

    for pair in pairs {
        // we assume that start <= end for all ranges
        if pair.first_start >= pair.second_start && pair.first_start <= pair.second_end
            || pair.first_start <= pair.second_start && pair.first_end >= pair.second_start
        {
            overlap_count += 1;
        }
    }

    overlap_count
}

#[derive(Debug)]
struct Pair {
    first_start: u64,
    first_end: u64,

    second_start: u64,
    second_end: u64,
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .filter_map(parse_line)
        // do the thing
        .collect()
}

fn parse_line(input: &str) -> Option<Pair> {
    let mut ranges = input.trim_end().split(",");
    let (first_start, first_end) = parse_range(ranges.next()?)?;
    let (second_start, second_end) = parse_range(ranges.next()?)?;

    Some(Pair {
        first_start,
        first_end,

        second_start,
        second_end,
    })
}

fn parse_range(input: &str) -> Option<(u64, u64)> {
    let mut splits = input.split("-");

    let start = splits.next()?.parse().ok()?;
    let end = splits.next()?.parse().ok()?;

    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4)
    }
}
