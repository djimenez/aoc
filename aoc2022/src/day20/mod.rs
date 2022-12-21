const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> i64 {
    let mut file = parse_input(input);

    //debug_file(&file);
    mix_file(&mut file);

    // find 0
    let zero_idx = file.iter().position(|&(_, n)| n == 0).unwrap();
    //dbg!(file[(zero_idx + 1000) % file.len()].1, file[(zero_idx + 2000) % file.len()].1, file[(zero_idx + 3000) % file.len()].1);

    file[(zero_idx + 1000) % file.len()].1
        + file[(zero_idx + 2000) % file.len()].1
        + file[(zero_idx + 3000) % file.len()].1
}

fn part2(input: &str) -> i64 {
    let mut file = parse_input(input);

    // "decrypt" each entry
    for entry in file.iter_mut() {
        entry.1 = entry.1 * 811589153;
    }

    //debug_file(&file);

    // run through 10 times
    for _ in 0..10 {
        mix_file(&mut file);
        //debug_file(&file);
    }

    // find 0
    let zero_idx = file.iter().position(|&(_, n)| n == 0).unwrap();
    //dbg!(file[(zero_idx + 1000) % file.len()].1, file[(zero_idx + 2000) % file.len()].1, file[(zero_idx + 3000) % file.len()].1);

    file[(zero_idx + 1000) % file.len()].1
        + file[(zero_idx + 2000) % file.len()].1
        + file[(zero_idx + 3000) % file.len()].1
}

fn parse_input(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .enumerate()
        .collect()
}

#[allow(dead_code)]
fn debug_file(file: &Vec<(usize, i64)>) {
    let repr = file
        .iter()
        .map(|(_, n)| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", repr);
}

fn mix_file(file: &mut Vec<(usize, i64)>) {
    for idx in 0..file.len() {
        let mut file_idx = file.iter().position(|&(order, _)| order == idx).unwrap();
        let (_, mut n) = file[file_idx];

        // all the edge cases for direct remove / inserts broke me, so I just do minimal swaps instead
        n %= file.len() as i64 - 1;

        while n > 0 {
            let next_idx = (file_idx + 1) % file.len();

            file.swap(file_idx, next_idx);

            file_idx = next_idx;
            n -= 1;
        }

        while n < 0 {
            let prev_idx = (file_idx + file.len() - 1) % file.len();
            file.swap(file_idx, prev_idx);

            file_idx = prev_idx;
            n += 1;
        }

        //debug_file(&file);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1623178306)
    }
}
