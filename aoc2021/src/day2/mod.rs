const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let instructions = parse_input(input);
    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;

    for (cmd, units) in instructions {
        match cmd {
            "forward" => horizontal += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => (),
        }
    }

    horizontal * depth
}

fn part2(input: &str) -> u64 {
    let instructions = parse_input(input);
    let mut aim: u64 = 0;
    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;

    for (cmd, units) in instructions {
        match cmd {
            "forward" => {
                horizontal += units;
                depth += units * aim;
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => (),
        }
    }

    horizontal * depth
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, u64)> {
    input.lines().map(|line| {
        let mut words = line.trim_end().split(" ");
        let cmd = words.next().unwrap();
        let units = words.next().unwrap().parse::<u64>().unwrap();

        (cmd, units)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 900)
    }
}
