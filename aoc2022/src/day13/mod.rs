use std::cmp::Ordering;
use std::str::FromStr;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    let pairs = parse_input(input);

    //dbg!(pairs);
    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.left.cmp(&pair.right) != Ordering::Greater)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = parse_input2(input);

    // sort the packets
    packets.sort();

    let divider1 = "[[2]]".parse().unwrap();
    let divider2 = "[[6]]".parse().unwrap();

    // find where we could put the divisors
    let index1 = packets
        .binary_search(&divider1)
        .expect_err("divisor existed in packets")
        + 1;
    let index2 = packets
        .binary_search(&divider2)
        .expect_err("divisor existed in packets")
        + 2;

    index1 * index2
}

#[derive(Debug, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Value(i64),
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::List(left) => {
                match other {
                    Self::List(right) => {
                        // vec already had lexographic code, use it
                        left.cmp(right)
                    }

                    Self::Value(right) => {
                        // allocating out of convenience
                        let new_right = PacketData::List(vec![PacketData::Value(*right)]);
                        self.cmp(&new_right)
                    }
                }
            }

            Self::Value(left) => {
                match other {
                    Self::Value(right) => left.cmp(right),

                    Self::List(_right) => {
                        // allocating out of convenience
                        let new_left = PacketData::List(vec![PacketData::Value(*left)]);
                        new_left.cmp(other)
                    }
                }
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for PacketData {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut list_stack = Vec::new();
        let mut value = String::new();

        for chr in input.chars() {
            match chr {
                '[' => {
                    list_stack.push(Vec::new());
                }

                ']' => {
                    // check if there's a value to push
                    if value.len() > 0 {
                        let list = list_stack.last_mut().unwrap();

                        list.push(PacketData::Value(value.parse::<i64>().unwrap()));

                        value.clear();
                    }

                    // pop the list from the stack and push it as a List
                    let list = list_stack.pop().unwrap();

                    if list_stack.len() > 0 {
                        list_stack.last_mut().unwrap().push(PacketData::List(list));
                    } else {
                        return Ok(PacketData::List(list));
                    }
                }

                ',' => {
                    // check if there's a value to push
                    if value.len() > 0 {
                        let list = list_stack.last_mut().unwrap();

                        list.push(PacketData::Value(value.parse().unwrap()));

                        value.clear();
                    }
                }

                c => {
                    value.push(c);
                }
            }
        }

        Err("could not parse packet")
    }
}

#[derive(Debug)]
struct PacketPair {
    left: PacketData,
    right: PacketData,
}

fn parse_input(input: &str) -> Vec<PacketPair> {
    input
        .split("\r\n\r\n")
        .map(|packets| {
            let mut lines = packets.lines();

            let left = lines.next().unwrap().parse().unwrap();
            let right = lines.next().unwrap().parse().unwrap();

            PacketPair { left, right }
        })
        .collect()
}

fn parse_input2(input: &str) -> Vec<PacketData> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140)
    }
}
