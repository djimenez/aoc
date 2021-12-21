const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let mut bingo = parse_input(input);

    bingo.boards.len() as u64
}

fn part2(input: &str) -> u64 {
    let mut bingo = parse_input(input);

    bingo.boards.len() as u64
}

fn parse_input(input: &str) -> Bingo {
    let mut bingo = Bingo::new();
    let mut lines = input.lines();

    // expect first line to be called numbers as csv
    let first_line = lines.next().unwrap().trim_end();

    for (index, call) in first_line
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .enumerate()
    {
        bingo.calls[index] = call;
    }

    // parse boards
    while let Some(_) = lines.next() {
        let mut board = Board::default();

        lines.next();
        lines.next();
        lines.next();
        lines.next();
        lines.next();

        bingo.boards.push(board);
    }

    bingo
}

fn parse_board<'a>(lines: impl Iterator<Item = &'a str>) -> Board {
    let mut pos: [u8; 25] = [0; 25];

    for (index, number) in lines
        .flat_map(|line| line.split_whitespace())
        .map(|num| num.parse::<u8>().unwrap())
        .enumerate()
    {
        pos[number as usize] = index as u8;
    }

    Board {
        pos,

        row: [0; 5],
        col: [0; 5],
    }
}

#[derive(Debug)]
struct Bingo {
    calls: [u8; 27],
    boards: Vec<Board>,
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            calls: [0; 27],
            boards: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
struct Board {
    pos: [u8; 25],

    row: [u8; 5],
    col: [u8; 5],
}

impl Board {
    fn mark(&mut self, num: u8) -> bool {
        let pos = self.pos[num as usize];

        let r = pos as usize / 5;
        let c = pos as usize % 5;

        self.row[r] += 1;
        self.col[c] += 1;

        self.row[r] >= 5 || self.col[c] >= 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0)
    }

    #[test]
    fn test_parse_input() {
        let bingo = parse_input(TEST_INPUT);

        assert_eq!(bingo.calls.len(), 27);
        assert_eq!(bingo.boards.len(), 3);
    }
}
