use std::collections::HashMap;

const DEFAULT_INPUT: &str = include_str!("input");
const PLAYER_START_POS: &str = "Player x starting position: ";

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let (mut p1_pos, mut p2_pos) = parse_input(input);
    p1_pos -= 1;
    p2_pos -= 1;

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut die = TestDie::new();

    loop {
        let p1_roll = die.roll() + die.roll() + die.roll();
        p1_pos = (p1_pos + p1_roll) % 10;

        p1_score += p1_pos + 1;
        //println!("p1 ({}): {} {} {}", die.rolls, p1_roll, p1_pos + 1, p1_score);

        if p1_score >= 1000 {
            return p2_score * die.rolls;
        }

        let p2_roll = die.roll() + die.roll() + die.roll();
        p2_pos = (p2_pos + p2_roll) % 10;

        p2_score += p2_pos + 1;
        //println!("p2 ({}): {} {} {}", die.rolls, p2_roll, p2_pos + 1, p2_score);

        if p2_score >= 1000 {
            return p1_score * die.rolls;
        }
    }
}

fn part2(input: &str) -> u64 {
    let (mut p1_pos, mut p2_pos) = parse_input(input);
    p1_pos -= 1;
    p2_pos -= 1;

    let cache = &mut HashMap::new();

    let (p1_wins, p2_wins) = dirac(cache, p1_pos as u8, 0, p2_pos as u8, 0);

    std::cmp::max(p1_wins, p2_wins)
}

fn dirac(
    cache: &mut HashMap<DiracKey, (u64, u64)>,
    p1_pos: u8,
    p1_score: u8,
    p2_pos: u8,
    p2_score: u8,
) -> (u64, u64) {
    let key = &DiracKey {
        p1_pos,
        p1_score,
        p2_pos,
        p2_score,
    };

    if let Some(answer) = cache.get(key) {
        return *answer;
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                let p1_roll = i + j + k;
                let p1_pos_next = (p1_pos + p1_roll) % 10;
                let p1_score_next = p1_score + p1_pos_next + 1;

                if p1_score_next >= 21 {
                    p1_wins += 1;
                } else {
                    for x in 1..4 {
                        for y in 1..4 {
                            for z in 1..4 {
                                let p2_roll = x + y + z;
                                let p2_pos_next = (p2_pos + p2_roll) % 10;
                                let p2_score_next = p2_score + p2_pos_next + 1;

                                if p2_score_next >= 21 {
                                    p2_wins += 1;
                                } else {
                                    let (p1_wins_next, p2_wins_next) = dirac(
                                        cache,
                                        p1_pos_next,
                                        p1_score_next,
                                        p2_pos_next,
                                        p2_score_next,
                                    );

                                    p1_wins += p1_wins_next;
                                    p2_wins += p2_wins_next;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    cache.insert(*key, (p1_wins, p2_wins));

    (p1_wins, p2_wins)
}

fn parse_input(input: &str) -> (u64, u64) {
    let starting_positions: Vec<_> = input
        .lines()
        .map(|line| {
            let start = PLAYER_START_POS.len();
            let end = line.len();
            let value = &line[start..end];

            value.parse().unwrap()
        })
        .collect();

    (starting_positions[0], starting_positions[1])
}

#[derive(Debug)]
struct TestDie {
    value: u64,
    rolls: u64,
}

impl TestDie {
    fn new() -> Self {
        TestDie { value: 1, rolls: 0 }
    }

    fn roll(&mut self) -> u64 {
        self.rolls += 1;

        let value = self.value;

        self.value += 1;

        if self.value > 100 {
            self.value = 1;
        }

        value
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DiracKey {
    p1_pos: u8,
    p1_score: u8,
    p2_pos: u8,
    p2_score: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 739785)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 444356092776315)
    }
}
