const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let mut monkeys = parse_input(input);

    for _ in 0..20 {
        // can't directly iterate cause we have to do mut borrow to pass items
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut throws = Vec::new();

            for mut item in monkey.items.drain(..) {
                item = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                };

                item /= 3;

                // can't directly modify other monkey in this loop, at least with how i wrote sucky rust
                if item % monkey.test == 0 {
                    throws.push((monkey.true_monkey, item));
                } else {
                    throws.push((monkey.false_monkey, item));
                }

                monkey.inspections += 1;
            }

            for (monkey_idx, item) in throws {
                monkeys[monkey_idx].items.push(item);
            }
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();

    // sort descending
    inspections.sort_by(|a, b| b.cmp(a));

    // compute monkey business
    inspections[0] * inspections[1]
}

fn part2(input: &str) -> u64 {
    let mut monkeys = parse_input(input);

    // we need to keep worry managable using mod math, since they all divide by different numbers
    // combine them all into a lcm - assuming all mutually prime and unique
    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test)
        .reduce(|a, b| a * b)
        .unwrap();

    for _ in 0..10000 {
        // can't directly iterate cause we have to do mut borrow to pass items
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut throws = Vec::new();

            for mut item in monkey.items.drain(..) {
                item = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                };

                // mischief managed
                item %= lcm;

                // can't directly modify other monkey in this loop, at least with how i wrote sucky rust
                if item % monkey.test == 0 {
                    throws.push((monkey.true_monkey, item));
                } else {
                    throws.push((monkey.false_monkey, item));
                }

                monkey.inspections += 1;
            }

            for (monkey_idx, item) in throws {
                monkeys[monkey_idx].items.push(item);
            }
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();

    // sort descending
    inspections.sort_by(|a, b| b.cmp(a));

    // compute monkey business
    inspections[0] * inspections[1]
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    inspections: u64,
    items: Vec<u64>,

    operation: Operation,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(parse_monkey)
        .collect()
}

fn parse_monkey(lines: &[&str]) -> Monkey {
    // skip the first line
    // parse items
    let items = lines[1]["  Starting items: ".len()..]
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();

    // parse operation
    let mut operation_split = lines[2]["  Operation: new = old ".len()..].split(" ");

    let operator = operation_split.next().unwrap();
    let operand = operation_split.next().unwrap();

    let operation = match (operator, operand) {
        ("*", "old") => Operation::Square,
        ("*", _) => Operation::Multiply(operand.parse().unwrap()),
        ("+", _) => Operation::Add(operand.parse().unwrap()),

        _ => panic!("unknown operation: {}", operator),
    };

    // parse test
    let test = lines[3]["  Test: divisible by ".len()..].parse().unwrap();

    // parse true monkey
    let true_monkey = lines[4]["    If true: throw to monkey ".len()..]
        .parse()
        .unwrap();

    // parse false monkey
    let false_monkey = lines[5]["    If false: throw to monkey ".len()..]
        .parse()
        .unwrap();

    Monkey {
        inspections: 0,
        items,

        operation,
        test,

        true_monkey,
        false_monkey,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158)
    }
}
