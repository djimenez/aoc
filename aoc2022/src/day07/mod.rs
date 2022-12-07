use std::collections::HashMap;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let ops = parse_input(input);
    let dir_contents = walk_ops(ops);

    dir_contents
        .into_values()
        .filter(|file_size| *file_size <= 100000)
        .sum()
}

fn part2(input: &str) -> u64 {
    let ops = parse_input(input);
    let dir_contents = walk_ops(ops);

    // compute how much space we need
    let total_used = *dir_contents.get("/").unwrap();
    let current_unused = 70000000 - total_used;
    let min_to_free = 30000000 - current_unused;

    dir_contents
        .into_values()
        .filter(|file_size| *file_size >= min_to_free)
        .min()
        .unwrap()
}

#[derive(Debug)]
enum Op<'a> {
    Push(&'a str),
    Pop(),
    List(&'a str),
}

fn parse_input<'source>(input: &'source str) -> Vec<Op<'source>> {
    // uuuuuuuuuuuuuuugly
    input[2..]
        .split("$ ")
        .map(|cmd_and_result| {
            match &cmd_and_result[0..3] {
                "cd " => match &cmd_and_result[3..5] {
                    ".." => Op::Pop(),
                    _ => Op::Push(&cmd_and_result[3..].trim_end()),
                },
                // assumes ls\r\n
                _ => Op::List(&cmd_and_result[4..]),
            }
        })
        .collect()
}

fn walk_ops(ops: Vec<Op>) -> HashMap<String, u64> {
    let mut dir_stack = Vec::new();
    let mut dir_contents = HashMap::new();

    for op in ops {
        match op {
            Op::Push(dir) => {
                dir_stack.push(dir);
            }

            Op::Pop() => {
                // when we pop get our current value
                let child_dir = dir_stack.join("/");
                let child_size = *dir_contents.get(&child_dir).unwrap();

                dir_stack.pop();

                // now add it to the parent
                let parent_dir = dir_stack.join("/");
                dir_contents
                    .entry(parent_dir)
                    .and_modify(|parent_size| *parent_size += child_size);
            }

            Op::List(contents) => {
                let file_size = contents
                    .lines()
                    // parse will fail on dir lines, which is fine
                    .filter_map(|line| line.split(' ').next()?.parse::<u64>().ok())
                    .sum();

                let current_dir = dir_stack.join("/");
                dir_contents.insert(current_dir, file_size);
            }
        }
    }

    // implicitly pop the rest
    while dir_stack.len() > 1 {
        let child_dir = dir_stack.join("/");
        let child_size = *dir_contents.get(&child_dir).unwrap();

        dir_stack.pop();

        // now add it to the parent
        let parent_dir = dir_stack.join("/");
        dir_contents
            .entry(parent_dir)
            .and_modify(|parent_size| *parent_size += child_size);
    }

    dir_contents
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 95437)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642)
    }
}
