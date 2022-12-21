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
    columns: [Vec<bool>; 7],
}

impl Game {
    fn new() -> Self {
        Self {
            pieces: 0,
            offset: 0,
            columns: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
        }
    }

    fn max_height(&self) -> usize {
        self.columns
            .iter()
            .filter_map(|column| column.iter().rposition(|c| *c).map(|idx| idx + 1))
            .max()
            .unwrap_or(0)
    }

    fn add(&mut self, piece: &Piece) {
        self.pieces += 1;

        match piece.shape {
            Shape::HorizonalBar => {
                for x in piece.x..piece.x + 4 {
                    while self.columns[x].len() < piece.y + 1 {
                        self.columns[x].push(false);
                    }
                }

                let &Piece { x, y, .. } = piece;

                self.columns[x + 0][y + 0] = true;
                self.columns[x + 1][y + 0] = true;
                self.columns[x + 2][y + 0] = true;
                self.columns[x + 3][y + 0] = true;
            }
            Shape::Plus => {
                for x in piece.x..piece.x + 3 {
                    while self.columns[x].len() < piece.y + 3 {
                        self.columns[x].push(false);
                    }
                }

                let &Piece { x, y, .. } = piece;

                self.columns[x + 0][y + 1] = true;
                self.columns[x + 1][y + 0] = true;
                self.columns[x + 1][y + 1] = true;
                self.columns[x + 1][y + 2] = true;
                self.columns[x + 2][y + 1] = true;
            }
            Shape::BackwardsL => {
                for x in piece.x..piece.x + 3 {
                    while self.columns[x].len() < piece.y + 3 {
                        self.columns[x].push(false);
                    }
                }

                let &Piece { x, y, .. } = piece;

                self.columns[x + 0][y + 0] = true;
                self.columns[x + 1][y + 0] = true;
                self.columns[x + 2][y + 0] = true;
                self.columns[x + 2][y + 1] = true;
                self.columns[x + 2][y + 2] = true;
            }
            Shape::VerticalBar => {
                while self.columns[piece.x].len() < piece.y + 4 {
                    self.columns[piece.x].push(false);
                }

                let &Piece { x, y, .. } = piece;

                self.columns[x + 0][y + 0] = true;
                self.columns[x + 0][y + 1] = true;
                self.columns[x + 0][y + 2] = true;
                self.columns[x + 0][y + 3] = true;
            }
            Shape::Square => {
                for x in piece.x..piece.x + 2 {
                    while self.columns[x].len() < piece.y + 2 {
                        self.columns[x].push(false);
                    }
                }

                let &Piece { x, y, .. } = piece;

                self.columns[x + 0][y + 0] = true;
                self.columns[x + 0][y + 1] = true;
                self.columns[x + 1][y + 0] = true;
                self.columns[x + 1][y + 1] = true;
            }
        }

        // flood fill spaces from the top, see what we can reach
        let max_height = self.max_height() + 1;

        if max_height > 1000000 {
            let mut reachable = HashMap::new();
            let mut queue = VecDeque::new();

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
                //println!("truncated {} lines", y_min);

                for x in 0..7 {
                    self.columns[x].drain(0..y_min);
                }

                self.offset += y_min;
            }
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
        x <= 6 && !*self.columns[x].get(y).unwrap_or(&false)
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

        /*
        println!("=== NEW PIECE ===");
        println!("{:?}", piece);
        println!("=== NEW PIECE ===");
        */

        /*
        println!("{}", repeat('-').take(game.max_height()).collect::<String>());
        for column in game.columns.iter() {
            println!("{}", column.iter().map(|c| if *c { '#' } else { '.' }).collect::<String>())
        }
        println!("{}", repeat('-').take(game.max_height()).collect::<String>());
        */

        while game.step(&mut piece, moves.next().unwrap()) {}

        //println!("Came to rest: {:?}", &piece);

        // solidify the piece in the game board
        game.add(&piece);
    }

    game.max_height() + game.offset
}

fn part2(input: &str) -> usize {
    let parsed_moves = parse_input(input);
    let mut shapes = SHAPES.iter().cycle();
    let mut moves = parsed_moves.iter().cycle();

    let mut game = Game::new();

    while game.pieces < 1000000000000 {
        // piece is 2 from left and 3 from top of all fixed pieces
        let mut piece = Piece {
            shape: shapes.next().unwrap().clone(),
            x: 2,
            y: game.max_height() + 3,
        };

        /*
        println!("=== NEW PIECE ===");
        println!("{:?}", piece);
        println!("=== NEW PIECE ===");
        */

        /*
        println!("{}", repeat('-').take(game.max_height()).collect::<String>());
        for column in game.columns.iter() {
            println!("{}", column.iter().map(|c| if *c { '#' } else { '.' }).collect::<String>())
        }
        println!("{}", repeat('-').take(game.max_height()).collect::<String>());
        */

        while game.step(&mut piece, moves.next().unwrap()) {}

        //println!("Came to rest: {:?}", &piece);

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
