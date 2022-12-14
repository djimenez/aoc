use std::cmp::Ordering;
use std::str::FromStr;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, pair)| {
            if pair[0] <= pair[1] {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = parse_input(input);

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
        match (self, other) {
            (Self::List(left), Self::List(right)) => {
                // vec already has lexographic code, use it
                left.cmp(right)
            }
            (Self::Value(left), Self::Value(right)) => left.cmp(right),
            (Self::List(left), Self::Value(right)) => {
                // allocating out of convenience
                left.cmp(&vec![Self::Value(*right)])
            }
            (Self::Value(left), Self::List(right)) => {
                // allocating out of convenience
                (&vec![Self::Value(*left)]).cmp(right)
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

        fn maybe_push_value(
            value: &mut String,
            list_stack: &mut Vec<Vec<PacketData>>,
        ) -> Result<(), &'static str> {
            // check if there's a value to push
            if value.len() > 0 {
                let list = list_stack
                    .last_mut()
                    .ok_or("no list on stack to push value into")?;
                list.push(PacketData::Value(
                    value.parse::<i64>().or(Err("could not parse value"))?,
                ));
                value.clear();
            }

            Ok(())
        }

        for chr in input.chars() {
            match chr {
                '[' => {
                    list_stack.push(Vec::new());
                }

                ']' => {
                    maybe_push_value(&mut value, &mut list_stack)?;

                    // pop the list from the stack and push it as a List
                    let list = list_stack.pop().ok_or("] but no list on stack")?;

                    if let Some(last_list) = list_stack.last_mut() {
                        last_list.push(PacketData::List(list));
                    } else {
                        return Ok(PacketData::List(list));
                    }
                }

                ',' => {
                    maybe_push_value(&mut value, &mut list_stack)?;
                }

                _ => {
                    value.push(chr);
                }
            }
        }

        Err("Unexpected end of packet")
    }
}

fn parse_input(input: &str) -> Vec<PacketData> {
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
