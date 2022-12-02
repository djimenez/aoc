const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let mut bingo = parse_input(input);

    *bingo.play().first().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut bingo = parse_input(input);

    *bingo.play().last().unwrap()
}

fn parse_input(input: &str) -> Bingo {
    let mut bingo = Bingo::new();
    let mut lines = input.lines();

    if let Some(line) = lines.next() {
        bingo.calls.extend(
            line.trim_end()
                .split(',')
                .map(|str| str.parse::<u8>().unwrap()),
        );
    }

    while let Some(_) = lines.next() {
        let mut board = Board::default();

        for row in 0..5 {
            // its so beautiful... /s
            for (col, value) in lines
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|str| str.parse().unwrap())
                .enumerate()
            {
                board.card[5 * row + col] = value;
            }
        }

        //println!("{:?}", board);

        bingo.boards.push(board);
    }

    bingo
}

#[derive(Debug)]
struct Bingo {
    calls: Vec<u8>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            calls: Vec::new(),
            boards: Vec::new(),
        }
    }

    fn play(&mut self) -> Vec<u64> {
        // I tried to return an iterator directly, but it blew up.. maybe revisit or try experimental generators
        let mut scores = Vec::new();

        for call in &self.calls {
            // puzzle description does not describe what happens if multiple boards win in
            // same round, so we assume they can't given the input and just process in order
            for board in &mut self.boards {
                if let Some(score) = board.mark(*call) {
                    scores.push(score);
                }
            }
        }

        scores
    }
}

#[derive(Debug, Default)]
struct Board {
    card: [u8; 25],
    marks: [bool; 25],
    won: bool,

    row: [u8; 5],
    col: [u8; 5],
}

impl Board {
    fn mark(&mut self, num: u8) -> Option<u64> {
        if !self.won {
            if let Some(pos) = self.card.iter().position(|val| *val == num) {
                if !self.marks[pos] {
                    self.marks[pos] = true;

                    let r = pos as usize / 5;
                    let c = pos as usize % 5;

                    self.row[r] += 1;
                    self.col[c] += 1;

                    if self.row[r] >= 5 || self.col[c] >= 5 {
                        self.won = true;

                        let mut sum = 0;
                        for i in 0..25 {
                            if !self.marks[i] {
                                sum += self.card[i] as u64;
                            }
                        }

                        return Some(num as u64 * sum);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4512)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1924)
    }

    #[test]
    fn test_parse_input() {
        let bingo = parse_input(TEST_INPUT);

        assert_eq!(bingo.calls.len(), 27);
        assert_eq!(bingo.boards.len(), 3);
    }
}
