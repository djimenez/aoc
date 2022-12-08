const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    // assumes count > 0 and from != to
    for (count, from, to) in instructions {
        for _ in 0..count {
            let value = stacks[from].pop().unwrap();
            stacks[to].push(value);
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
    let mut values: Vec<char> = Vec::new();

    // assumes count > 0 and from != to
    for (count, from, to) in instructions {
        let from_idx = stacks[from].len() - count;

        // we can't mut borrow multiple indexes of the same vec, so collect values into reused char vec
        values.extend(stacks[from].drain(from_idx..));
        stacks[to].extend_from_slice(values.as_slice());

        values.clear();
    }

    // collect top letters
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut sections = input.split("\r\n\r\n");

    let crates = sections.next().unwrap();
    let instructions = sections.next().unwrap();

    // simpler to parse crates in reverse line order
    let mut crate_lines = crates.lines().rev();

    // the first (reversed) line will tell us how many stacks there are
    let stack_ids: Vec<&str> = crate_lines.next().unwrap().split_whitespace().collect();
    let mut stacks = Vec::with_capacity(stack_ids.len());

    for _ in stack_ids {
        stacks.push(Vec::new());
    }

    // NOTE: we consumed the label line already
    for crate_line in crate_lines {
        // look for alpha characters, and convert their position in the line to a stack index
        for (crate_line_idx, crate_line_char) in crate_line.chars().enumerate() {
            if crate_line_char.is_alphabetic() {
                let stack_idx = (crate_line_idx - 1) / 4;
                stacks[stack_idx].push(crate_line_char);
            }
        }
    }

    let mut parts: Vec<&str> = Vec::with_capacity(5);

    // parse instruction lines into tuples (count, from, to)
    let parsed_instructions = instructions
        .lines()
        .map(|instruction| {
            parts.extend(instruction.split_whitespace());

            let count = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap() - 1;
            let to = parts[5].parse::<usize>().unwrap() - 1;

            parts.clear();

            (count, from, to)
        })
        .collect();

    (stacks, parsed_instructions)
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
