const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let mut state = parse_input(input);

    for _ in 0..80 {
        state.step();
        //println!("{:?} {}", state, state.total());
    }

    state.total()
}

fn part2(input: &str) -> u64 {
    let mut state = parse_input(input);

    for _ in 0..256 {
        state.step();
        //println!("{:?} {}", state, state.total());
    }

    state.total()
}

fn parse_input(input: &str) -> State {
    let mut state = State { fish: [0; 9] };

    for value in input
        .trim_end()
        .split(',')
        .map(|value| value.parse::<usize>().unwrap())
    {
        state.fish[value] += 1;
    }

    state
}

#[derive(Debug, Default)]
struct State {
    fish: [u64; 9],
}

impl State {
    fn step(&mut self) {
        let spawning = self.fish[0];

        self.fish[0] = self.fish[1];
        self.fish[1] = self.fish[2];
        self.fish[2] = self.fish[3];
        self.fish[3] = self.fish[4];
        self.fish[4] = self.fish[5];
        self.fish[5] = self.fish[6];
        self.fish[6] = self.fish[7] + spawning;
        self.fish[7] = self.fish[8];
        self.fish[8] = spawning;
    }

    fn total(&self) -> u64 {
        let mut total = 0;

        for value in self.fish {
            total += value;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5934)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 26984457539)
    }
}
