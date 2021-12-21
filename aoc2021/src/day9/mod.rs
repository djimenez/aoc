
const DEFAULT_INPUT : &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input : &str) -> u64 {
    0
}

fn part2(input : &str) -> u64 {
    0
}

fn parse_input(input : &str) -> impl Iterator<Item = &str> {
    input
      .lines()
      .map(|line| line.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT : &str = include_str!("test");
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0)
    }
}