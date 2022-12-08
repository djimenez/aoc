use take_until::TakeUntilExt;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> usize {
    let mut trees = parse_input(input);

    // all outside trees are visible
    trees
        .first_mut()
        .unwrap()
        .iter_mut()
        .for_each(|tree| tree.visible = true);
    trees
        .last_mut()
        .unwrap()
        .iter_mut()
        .for_each(|tree| tree.visible = true);

    for row in trees.iter_mut() {
        row.first_mut().unwrap().visible = true;
        row.last_mut().unwrap().visible = true;
    }

    // for all interior trees, compute visibility naively
    for row_idx in 1..trees.len() - 1 {
        for col_idx in 1..trees[0].len() - 1 {
            trees[row_idx][col_idx].visible = is_visible(&trees, row_idx, col_idx);
        }
    }

    // sum visible trees
    trees
        .iter()
        .map(|row| row.iter().filter(|tree| tree.visible).count())
        .sum()
}

fn is_visible(trees: &Vec<Vec<Tree>>, i: usize, j: usize) -> bool {
    let height = trees[i][j].height;

    // left
    if trees[i][0..j].iter().all(|tree| tree.height < height) {
        return true;
    }

    // right
    if trees[i][j + 1..].iter().all(|tree| tree.height < height) {
        return true;
    }

    // up
    if trees[0..i].iter().all(|row| row[j].height < height) {
        return true;
    }

    // down
    if trees[i + 1..].iter().all(|row| row[j].height < height) {
        return true;
    }

    false
}

fn part2(input: &str) -> usize {
    let mut trees = parse_input(input);

    // all outside trees have a score of 0
    trees
        .first_mut()
        .unwrap()
        .iter_mut()
        .for_each(|tree| tree.score = 0);
    trees
        .last_mut()
        .unwrap()
        .iter_mut()
        .for_each(|tree| tree.score = 0);

    for row in trees.iter_mut() {
        row.first_mut().unwrap().score = 0;
        row.last_mut().unwrap().score = 0;
    }

    // for all interior trees compute score naively
    for row_idx in 1..trees.len() - 1 {
        for col_idx in 1..trees[0].len() - 1 {
            trees[row_idx][col_idx].score = compute_score(&trees, row_idx, col_idx);
        }
    }

    // find max score
    trees
        .iter()
        .map(|row| row.iter().map(|tree| tree.score).max().unwrap())
        .max()
        .unwrap()
}

fn compute_score(trees: &Vec<Vec<Tree>>, i: usize, j: usize) -> usize {
    let height = trees[i][j].height;

    let left = trees[i][0..j]
        .iter()
        .rev()
        .take_until(|tree| tree.height >= height)
        .count();
    let right = trees[i][j + 1..]
        .iter()
        .take_until(|tree| tree.height >= height)
        .count();
    let up = trees[0..i]
        .iter()
        .rev()
        .take_until(|row| row[j].height >= height)
        .count();
    let down = trees[i + 1..]
        .iter()
        .take_until(|row| row[j].height >= height)
        .count();

    left * right * up * down
}

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool,
    score: usize,
}

fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Tree {
                    height: char.to_digit(10).unwrap() as u8,
                    visible: false,
                    score: 0,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 8)
    }
}
