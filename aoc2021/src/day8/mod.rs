use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|display| {
            display
                .outputs
                .iter()
                .filter(|output| match output.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(|display| display.decode_output())
        .sum()
}

fn parse_input(input: &str) -> Vec<Display> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim().splitn(2, " | ").collect();

            assert_eq!(parts.len(), 2);

            let patterns: Vec<String> = parts[0]
                .split_whitespace()
                .map(|s| s.chars().sorted().collect())
                .collect();
            let outputs: Vec<String> = parts[1]
                .split_whitespace()
                .map(|s| s.chars().sorted().collect())
                .collect();

            Display { patterns, outputs }
        })
        .collect()
}

const ZERO: &str = "abcefg";
const ONE: &str = "cf";
const TWO: &str = "acdeg";
const THREE: &str = "acdfg";
const FOUR: &str = "bcdf";
const FIVE: &str = "abdfg";
const SIX: &str = "abdefg";
const SEVEN: &str = "acf";
const EIGHT: &str = "abcdefg";
const NINE: &str = "abcdfg";

#[derive(Debug)]
struct Display {
    // we want to sort the chars so we're owning the strings
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl Display {
    fn decode_output(&self) -> u64 {
        /*
        let patterns: Vec<u8> = self
            .patterns
            .iter()
            .map(|s| Self::pattern_to_bitfield(s))
            .collect();
        let outputs: Vec<u8> = self
            .outputs
            .iter()
            .map(|s| Self::pattern_to_bitfield(s))
            .collect();

        println!("{:?} {:?}", patterns, outputs);
        */
        let defaults = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let mut constraints: HashMap<char, HashSet<char>> = HashMap::new();

        for pattern in &self.patterns {
            let restrictions: HashSet<char> = match pattern.len() {
                2 => ['c', 'f'].into_iter().collect(),
                3 => ['a', 'c', 'f'].into_iter().collect(),
                4 => ['b', 'c', 'd', 'f'].into_iter().collect(),
                _ => ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect(),
            };

            for c in pattern.chars() {
                let mut constraint = constraints
                    .entry(c)
                    .or_insert(['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect());
                constraint.retain(|c| restrictions.contains(c));
            }
        }

        println!("{:#?}", constraints);

        0
    }

    fn pattern_to_bitfield(pattern: &str) -> u8 {
        let mut value = 0;

        for c in pattern.chars() {
            value += match c {
                'a' => 1 << 0,
                'b' => 1 << 1,
                'c' => 1 << 2,
                'd' => 1 << 3,
                'e' => 1 << 4,
                'f' => 1 << 5,
                'g' => 1 << 6,
                _ => 0,
            };
        }

        value
    }

    fn bitfield_to_pattern(bitfield: u8) -> String {
        let mut pattern = String::with_capacity(bitfield.count_ones() as usize);

        if bitfield & 1 > 0 {
            pattern.push('a');
        }
        if bitfield & 1 << 1 > 0 {
            pattern.push('b');
        }
        if bitfield & 1 << 2 > 0 {
            pattern.push('c');
        }
        if bitfield & 1 << 3 > 0 {
            pattern.push('d');
        }
        if bitfield & 1 << 4 > 0 {
            pattern.push('e');
        }
        if bitfield & 1 << 5 > 0 {
            pattern.push('f');
        }
        if bitfield & 1 << 6 > 0 {
            pattern.push('g');
        }

        pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 61229)
    }

    #[test]
    fn test_parse_input() {
        let values = parse_input(TEST_INPUT);

        assert_eq!(values.len(), 10)
    }

    #[test]
    fn test_pattern_to_bitfield() {
        assert_eq!(Display::pattern_to_bitfield("abcdefg"), 0b1111111)
    }

    #[test]
    fn test_bitfield_to_pattern() {
        assert_eq!(Display::bitfield_to_pattern(0b1111111), "abcdefg")
    }
}
