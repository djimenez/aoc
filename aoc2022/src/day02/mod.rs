use std::fmt::Debug;
use std::str::FromStr;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let games = parse_games::<Part1Game>(input);
    let mut score = 0;

    for game in games {
        score += compute_score(game.their_move, game.our_move);
    }

    score
}

fn part2(input: &str) -> u64 {
    let games = parse_games::<Part2Game>(input);
    let mut score = 0;

    for game in games {
        let our_move = game.compute_our_move();

        score += compute_score(game.their_move, our_move);
    }

    score
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameMove {
    ROCK = 1, // explicit values cause we do GameMove as u64 in compute_score
    PAPER = 2,
    SCISSORS = 3,
}

impl FromStr for GameMove {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::ROCK),
            "B" | "Y" => Ok(Self::PAPER),
            "C" | "Z" => Ok(Self::SCISSORS),
            _ => Err("Unknown move"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameStrategy {
    LOSE,
    DRAW,
    WIN,
}

impl FromStr for GameStrategy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::LOSE),
            "Y" => Ok(Self::DRAW),
            "Z" => Ok(Self::WIN),
            _ => Err("Unknown strategy"),
        }
    }
}

#[derive(Debug)]
struct Part1Game {
    their_move: GameMove,
    our_move: GameMove,
}

impl FromStr for Part1Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");

        // not sure what to do instead of expect's
        let their_move: GameMove = splits.next().expect("did not find their move").parse()?;
        let our_move: GameMove = splits.next().expect("did not find our move").parse()?;

        Ok(Self {
            their_move,
            our_move,
        })
    }
}

#[derive(Debug)]
struct Part2Game {
    their_move: GameMove,
    our_strategy: GameStrategy,
}

impl Part2Game {
    fn compute_our_move(&self) -> GameMove {
        match self.our_strategy {
            GameStrategy::DRAW => self.their_move,
            GameStrategy::LOSE => match self.their_move {
                GameMove::ROCK => GameMove::SCISSORS,
                GameMove::PAPER => GameMove::ROCK,
                GameMove::SCISSORS => GameMove::PAPER,
            },
            GameStrategy::WIN => match self.their_move {
                GameMove::ROCK => GameMove::PAPER,
                GameMove::PAPER => GameMove::SCISSORS,
                GameMove::SCISSORS => GameMove::ROCK,
            },
        }
    }
}

impl FromStr for Part2Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");

        // not sure what to do instead of expect's
        let their_move: GameMove = splits.next().expect("did not find game move").parse()?;
        let our_strategy: GameStrategy =
            splits.next().expect("did not find game strategy").parse()?;

        Ok(Self {
            their_move,
            our_strategy,
        })
    }
}

fn parse_games<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        // don't know what to do instead of expect
        .map(|line| line.parse().expect("could not parse line"))
        .collect()
}

fn compute_score(theirs: GameMove, mine: GameMove) -> u64 {
    mine as u64
        + match (theirs, mine) {
            (a, b) if a == b => 3,
            (GameMove::ROCK, GameMove::PAPER) => 6,
            (GameMove::PAPER, GameMove::SCISSORS) => 6,
            (GameMove::SCISSORS, GameMove::ROCK) => 6,
            _ => 0,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 15)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 12)
    }
}
