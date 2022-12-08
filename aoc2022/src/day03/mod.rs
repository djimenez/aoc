use std::collections::HashSet;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let rucksacks = parse_input(input);
    let mut priority_sum = 0;

    for rucksack in rucksacks {
        let items: Vec<_> = rucksack.chars().collect();
        let size = items.len() / 2;

        let mut left = HashSet::new();
        let mut right = HashSet::new();

        for i in 0..size {
            left.insert(items[i]);
            right.insert(items[i + size]);
        }

        let misplaced = left.intersection(&right);
        let mut priority = 0;

        for item in misplaced {
            priority += compute_priority(*item);
        }

        priority_sum += priority;
    }

    priority_sum
}

fn part2(input: &str) -> u64 {
    let rucksacks = parse_input(input);
    let mut priority_sum = 0;

    // chunk ruckscacks into groups
    for chunk in rucksacks.chunks(3) {
        let intersections = chunk
            .iter()
            .map(|rucksack| {
                let mut set = HashSet::new();

                for item in rucksack.chars() {
                    set.insert(item);
                }

                set
            })
            // BitAnd creates a new HashSet from the intersection
            .reduce(|left, right| &left & &right)
            .unwrap();

        for item in intersections {
            priority_sum += compute_priority(item);
        }
    }

    priority_sum
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn compute_priority(c: char) -> u64 {
    if c.is_uppercase() {
        (c as u64) - ('A' as u64) + 27
    } else {
        (c as u64) - ('a' as u64) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 70)
    }
}
