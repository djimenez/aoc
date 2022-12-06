use std::collections::hash_map::Entry;
use std::collections::HashMap;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT).unwrap());
    println!("part2: {}", part2(DEFAULT_INPUT).unwrap());
}

fn part1(input: &str) -> Option<usize> {
    let stream = parse_input(input);

    find_unique_seq(stream, 4)
}

fn part2(input: &str) -> Option<usize> {
    let stream = parse_input(input);

    find_unique_seq(stream, 14)
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim_end().chars().collect()
}

fn find_unique_seq(stream: Vec<char>, n: usize) -> Option<usize> {
    let mut map = HashMap::with_capacity(n);

    // insert n chars, then we'll loop rest
    for idx in 0..n {
        map.entry(stream[idx])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for idx in n..stream.len() {
        // assumed but not benchmarked: removing entries that are zero is faster than filter and count
        if map.len() == n {
            return Some(idx);
        }

        // increment next char count
        map.entry(stream[idx])
            .and_modify(|count| *count += 1)
            .or_insert(1);

        // decrement the char falling out of the window
        if let Entry::Occupied(entry) = map.entry(stream[idx - n]).and_modify(|count| *count -= 1) {
            // this feels awkward, but better than looking it up again (i think?)
            if *entry.get() == 0 {
                entry.remove_entry();
            }
        }
    }

    // needed for last window location
    if map.len() == n {
        return Some(stream.len());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Some(7))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Some(19))
    }
}
