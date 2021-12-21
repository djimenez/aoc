const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> u64 {
    let mut counter = BitCounter::default();

    for line in parse_input(input) {
        counter.count(line);
    }

    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    let max_index = counter.width - 1;

    for index in 0..counter.width {
        if counter.zeroes(index) < counter.ones(index) {
            gamma += 1 << (max_index - index);
        } else {
            epsilon += 1 << (max_index - index);
        }
    }

    gamma * epsilon
}

fn part2(input: &str) -> usize {
    let mut trie = CountingTrie::default();
    let mut width = 0;

    for value in parse_input(input) {
        width = value.len();
        trie.count(value);
    }

    let mut o2_rating = 0;
    let mut co2_rating = 0;

    // walk the prefixes simultaneously
    let mut o2_node = &trie;
    let mut co2_node = &trie;

    for index in 0..width {
        if o2_node.ones_count >= o2_node.zeroes_count {
            o2_rating += 1 << (width - 1 - index);

            if let Some(ref next) = o2_node.ones_child {
                o2_node = next.as_ref();
            }
        } else {
            if let Some(ref next) = o2_node.zeroes_child {
                o2_node = next.as_ref();
            }
        }

        if co2_node.zeroes_count == 1
            || co2_node.zeroes_count > 0 && co2_node.zeroes_count <= co2_node.ones_count
        {
            if let Some(ref next) = co2_node.zeroes_child {
                co2_node = next.as_ref();
            }
        } else {
            co2_rating += 1 << (width - 1 - index);

            if let Some(ref next) = co2_node.ones_child {
                co2_node = next.as_ref();
            }
        }
    }

    o2_rating * co2_rating
}

fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(|line| line.trim_end())
}

#[derive(Debug)]
struct BitCounter {
    width: usize,
    count: usize,
    ones: [usize; 64],
}

impl Default for BitCounter {
    fn default() -> Self {
        Self {
            width: 0,
            count: 0,
            ones: [0; 64],
        }
    }
}

impl BitCounter {
    fn count(&mut self, value: &str) {
        if self.count == 0 {
            self.width = value.len();
        } else if self.width != value.len() {
            panic!("BitCounter can't handle multiple widths");
        }

        self.count += 1;

        for (i, chr) in value.chars().enumerate() {
            if chr == '1' {
                self.ones[i] += 1;
            }
        }
    }

    // note: index is 0 for lsb
    fn zeroes(&self, index: usize) -> usize {
        return self.count - self.ones[index];
    }

    // note: index is 0 for lsb
    fn ones(&self, index: usize) -> usize {
        return self.ones[index];
    }
}

#[derive(Debug)]
struct CountingTrie {
    zeroes_count: usize,
    ones_count: usize,
    zeroes_child: Option<Box<Self>>,
    ones_child: Option<Box<Self>>,
}

impl Default for CountingTrie {
    fn default() -> Self {
        CountingTrie {
            zeroes_count: 0,
            ones_count: 0,

            zeroes_child: None,
            ones_child: None,
        }
    }
}

impl CountingTrie {
    fn count(&mut self, value: &str) {
        let mut node = self;

        for chr in value.chars() {
            match chr {
                '0' => {
                    node.zeroes_count += 1;
                    if node.zeroes_child.is_none() {
                        node.zeroes_child = Some(Box::new(CountingTrie::default()));
                    }

                    if let Some(ref mut child) = node.zeroes_child {
                        node = child.as_mut();
                    }
                }
                '1' => {
                    node.ones_count += 1;

                    if node.ones_child.is_none() {
                        node.ones_child = Some(Box::new(CountingTrie::default()));
                    }

                    if let Some(ref mut child) = node.ones_child {
                        node = child.as_mut();
                    }
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 198)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 230)
    }

    #[test]
    fn test_bitcounter_when_equal() {
        let mut counter = BitCounter::default();

        counter.count("0");
        counter.count("1");

        assert_eq!(counter.zeroes(0), 1);
        assert_eq!(counter.ones(0), 1);
    }

    #[test]
    fn test_counting_trie() {
        let mut trie = CountingTrie::default();

        for line in parse_input(TEST_INPUT) {
            trie.count(line);
        }

        assert_eq!(trie.zeroes_count, 5);
        assert_eq!(trie.ones_count, 7);
    }
}
