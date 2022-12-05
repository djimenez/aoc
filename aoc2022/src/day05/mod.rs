const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split_whitespace().collect();

        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            if stacks[from].len() > 0 {
                let value = stacks[from].pop().unwrap();
                stacks[to].push(value);
            }
        }
    }

    // collect top letters
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split_whitespace().collect();

        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        let from_idx = stacks[from].len() - count;
        let values: Vec<char> = stacks[from].drain(from_idx..).collect();
        stacks[to].extend_from_slice(&values[..]);
    }

    // collect top letters
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<&str>) {
    // will hold crate letters
    let mut stacks = Vec::new();

    let mut sections = input.split("\r\n\r\n");

    let crates = sections.next().unwrap();
    let instructions = sections.next().unwrap();

    // simpler to parse crates in reverse line order
    let mut crate_lines = crates.lines().rev().map(|line| line.trim_end());

    // the first line will tell us how many stacks there are
    let stack_ids: Vec<&str> = crate_lines.next().unwrap().split_whitespace().collect();

    for _ in stack_ids {
        stacks.push(Vec::new());
    }

    for crate_line in crate_lines {
        for (crate_line_idx, crate_line_char) in crate_line.chars().enumerate() {
            if crate_line_char.is_alphabetic() {
                let stack_idx = (crate_line_idx - 1) / 4;
                stacks[stack_idx].push(crate_line_char);
            }
        }
    }

    (
        stacks,
        instructions.lines().map(|line| line.trim_end()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "CMZ")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "MCD")
    }
}
