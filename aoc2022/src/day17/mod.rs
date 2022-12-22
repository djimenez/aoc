use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::repeat;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

const SHAPES: [Shape; 5] = [
    Shape::HorizonalBar,
    Shape::Plus,
    Shape::BackwardsL,
    Shape::VerticalBar,
    Shape::Square,
];

#[derive(Clone, Copy, Debug)]
enum Shape {
    HorizonalBar,
    Plus,
    BackwardsL,
    VerticalBar,
    Square,
}

#[derive(Debug)]
struct Piece {
    shape: Shape,
    x: usize,
    y: usize,
}

impl Piece {
    fn check(&self, game: &Game) -> bool {
        match self.shape {
            Shape::HorizonalBar => {
                game.is_empty(self.x, self.y)
                    && game.is_empty(self.x + 1, self.y)
                    && game.is_empty(self.x + 2, self.y)
                    && game.is_empty(self.x + 3, self.y)
            }
            Shape::Plus => {
                game.is_empty(self.x + 1, self.y)
                    && game.is_empty(self.x, self.y + 1)
                    && game.is_empty(self.x + 2, self.y + 1)
                    && game.is_empty(self.x + 1, self.y + 2)
            }
            Shape::BackwardsL => {
                game.is_empty(self.x, self.y)
                    && game.is_empty(self.x + 1, self.y)
                    && game.is_empty(self.x + 2, self.y)
                    && game.is_empty(self.x + 2, self.y + 1)
                    && game.is_empty(self.x + 2, self.y + 2)
            }
            Shape::VerticalBar => {
                game.is_empty(self.x, self.y)
                    && game.is_empty(self.x, self.y + 1)
                    && game.is_empty(self.x, self.y + 2)
                    && game.is_empty(self.x, self.y + 3)
            }
            Shape::Square => {
                game.is_empty(self.x, self.y)
                    && game.is_empty(self.x, self.y + 1)
                    && game.is_empty(self.x + 1, self.y)
                    && game.is_empty(self.x + 1, self.y + 1)
            }
        }
    }

    fn left(&mut self, game: &Game) {
        if self.x > 0 {
            self.x -= 1;

            if !self.check(game) {
                // rollback
                self.x += 1;
            }
        }
    }

    fn right(&mut self, game: &Game) {
        if self.x < 6 {
            self.x += 1;

            if !self.check(game) {
                // rollback
                self.x -= 1;
            }
        }
    }

    fn down(&mut self, game: &Game) -> bool {
        if self.y > 0 {
            self.y -= 1;

            if !self.check(game) {
                self.y += 1;
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Game {
    pieces: usize,
    offset: usize,
    rows: Vec<u8>,
}

impl Game {
    fn new() -> Self {
        Self {
            pieces: 0,
            offset: 0,
            rows: Vec::new(),
        }
    }

    fn max_height(&self) -> usize {
        self.rows
            .iter()
            .rposition(|row| *row > 0)
            .map(|i| i + 1)
            .unwrap_or(0)
    }

    fn add(&mut self, piece: &Piece) {
        self.pieces += 1;

        match piece.shape {
            Shape::HorizonalBar => {
                while self.rows.len() < piece.y + 1 {
                    self.rows.push(0);
                }

                let &Piece { x, y, .. } = piece;
                self.rows[y + 0] |= 0b01111000 >> x;
            }
            Shape::Plus => {
                while self.rows.len() < piece.y + 3 {
                    self.rows.push(0);
                }

                let &Piece { x, y, .. } = piece;

                self.rows[y + 2] |= 0b00100000 >> x;
                self.rows[y + 1] |= 0b01110000 >> x;
                self.rows[y + 0] |= 0b00100000 >> x;
            }
            Shape::BackwardsL => {
                while self.rows.len() < piece.y + 3 {
                    self.rows.push(0);
                }

                let &Piece { x, y, .. } = piece;

                self.rows[y + 2] |= 0b00010000 >> x;
                self.rows[y + 1] |= 0b00010000 >> x;
                self.rows[y + 0] |= 0b01110000 >> x;
            }
            Shape::VerticalBar => {
                while self.rows.len() < piece.y + 4 {
                    self.rows.push(0);
                }

                let &Piece { x, y, .. } = piece;

                self.rows[y + 3] |= 0b01000000 >> x;
                self.rows[y + 2] |= 0b01000000 >> x;
                self.rows[y + 1] |= 0b01000000 >> x;
                self.rows[y + 0] |= 0b01000000 >> x;
            }
            Shape::Square => {
                while self.rows.len() < piece.y + 2 {
                    self.rows.push(0);
                }

                let &Piece { x, y, .. } = piece;

                self.rows[y + 1] |= 0b01100000 >> x;
                self.rows[y + 0] |= 0b01100000 >> x;
            }
        }
    }

    #[allow(dead_code)]
    fn collapse(&mut self) -> bool {
        // flood fill spaces from the top, see what we can reach
        let mut reachable = HashMap::new();
        let mut queue = VecDeque::new();

        let max_height = self.max_height();
        let mut y_min = max_height;

        queue.push_back((0, max_height));

        while queue.len() > 0 {
            let (x, y) = queue.pop_front().unwrap();

            if !reachable.contains_key(&(x, y)) {
                let empty = self.is_empty(x, y);
                reachable.insert((x, y), empty);

                if empty {
                    if y < y_min {
                        y_min = y;
                    }

                    if x > 0 {
                        queue.push_back((x - 1, y));
                    }
                    if x < 7 {
                        queue.push_back((x + 1, y));
                    }
                    if y > 0 {
                        queue.push_back((x, y - 1));
                    }
                    if y < max_height {
                        queue.push_back((x, y + 1));
                    }
                }
            }
        }

        // truncate upto y_min
        if y_min < max_height && y_min > 0 {
            self.rows.drain(0..y_min - 1);
            self.offset += y_min - 1;

            true
        } else {
            false
        }
    }

    fn step(&self, piece: &mut Piece, movement: &char) -> bool {
        match movement {
            '<' => {
                //println!("moving {:?} left", &piece);
                piece.left(self)
            }
            '>' => {
                //println!("moving {:?} right", &piece);
                piece.right(self)
            }
            _ => (),
        }

        piece.down(self)
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        if y < self.rows.len() {
            (x <= 6) && (self.rows[y] & (0b01000000 >> x) == 0)
        } else {
            x <= 6
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let max_height = self.max_height();

        println!("{}", repeat('-').take(max_height).collect::<String>());
        for x in 0..7 {
            let mask = 0b01000000 >> x;

            println!(
                "{}",
                self.rows
                    .iter()
                    .map(|row| if row & mask == 0 { '.' } else { '#' })
                    .collect::<String>()
            );
        }
        println!("{}", repeat('-').take(max_height).collect::<String>());
    }
}

fn part1(input: &str) -> usize {
    let parsed_moves = parse_input(input);
    let mut shapes = SHAPES.iter().cycle();
    let mut moves = parsed_moves.iter().cycle();

    let mut game = Game::new();

    while game.pieces < 2022 {
        // piece is 2 from left and 3 from top of all fixed pieces
        let mut piece = Piece {
            shape: shapes.next().unwrap().clone(),
            x: 2,
            y: game.max_height() + 3,
        };

        while game.step(&mut piece, moves.next().unwrap()) {}

        // solidify the piece in the game board
        game.add(&piece);
    }

    game.max_height() + game.offset
}

fn part2(input: &str) -> usize {
    let parsed_moves = parse_input(input);

    // For test, if we visualize how much height and how many movements are used on each round of shapes,
    // we'll start to see a pattern from the 4th round on with period 7 rounds, similar to:
    //
    // height: +11 movements: +26
    // height: +7 movements: +27
    // height: +8 movements: +30
    // height: +9 movements: +29
    // height: +6 movements: +27
    // height: +6 movements: +29
    // height: +6 movements: +32
    //
    // We want to look for a cycle automatically in test and input data so we can skip simulating large chunks

    let mut moves = parsed_moves.iter().cycle();
    let mut game = Game::new();

    // switch the logic to play the initial game in full cycles of shapes
    while game.pieces < 2022 {
        for shape in &SHAPES {
            // piece is 2 from left and 3 from top of all fixed pieces
            let mut piece = Piece {
                shape: shape.clone(),
                x: 2,
                y: game.max_height() + 3,
            };

            while game.step(&mut piece, moves.next().unwrap()) {}

            // solidify the piece in the game board
            game.add(&piece);
        }
    }

    let search_start = game.max_height();

    // do a single round
    for shape in &SHAPES {
        // piece is 2 from left and 3 from top of all fixed pieces
        let mut piece = Piece {
            shape: shape.clone(),
            x: 2,
            y: game.max_height() + 3,
        };

        while game.step(&mut piece, moves.next().unwrap()) {}

        // solidify the piece in the game board
        game.add(&piece);
    }

    let search_end = game.max_height();
    let search_pieces = game.pieces;

    // now continue rounds until we find a match
    loop {
        let candidate_start = game.max_height();

        for shape in &SHAPES {
            // piece is 2 from left and 3 from top of all fixed pieces
            let mut piece = Piece {
                shape: shape.clone(),
                x: 2,
                y: game.max_height() + 3,
            };

            while game.step(&mut piece, moves.next().unwrap()) {}

            // solidify the piece in the game board
            game.add(&piece);
        }

        let candidate_end = game.max_height();
        let candidate_pieces = game.pieces;

        if game.rows[search_start..search_end] == game.rows[candidate_start..candidate_end] {
            let aligned_pieces = candidate_pieces - search_pieces;
            let aligned_height = candidate_end - search_end;

            let pieces_needed = 1_000_000_000_000 - game.pieces;
            let alignments = pieces_needed / aligned_pieces;

            // skiiiiiiiiiiiiiiiiiiiiiip...
            game.offset += alignments * aligned_height;
            game.pieces += alignments * aligned_pieces;

            break;
        }
    }

    // continue the simulation for any more pieces needed
    let mut shapes = SHAPES.iter().cycle();

    while game.pieces < 1_000_000_000_000 {
        // piece is 2 from left and 3 from top of all fixed pieces
        let mut piece = Piece {
            shape: shapes.next().unwrap().clone(),
            x: 2,
            y: game.max_height() + 3,
        };

        while game.step(&mut piece, moves.next().unwrap()) {}

        // solidify the piece in the game board
        game.add(&piece);
    }

    game.max_height() + game.offset
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim_end().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3068)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1514285714288)
    }
}
