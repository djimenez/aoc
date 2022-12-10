const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2:\r\n{}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut cpu = CPU::new(instructions);
    let mut signals = Vec::new();

    for cycle in 1..=220 {
        let x = cpu.next();

        if (cycle + 20) % 40 == 0 {
            signals.push(cycle * x);
        }
    }

    signals.iter().sum()
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let mut cpu = CPU::new(instructions);
    let mut crt = Vec::with_capacity(6);

    for _ in 0..6 {
        let mut scanline = String::with_capacity(40);

        for i in 0..40 {
            let x = cpu.next();

            if i == x - 1 || i == x || i == x + 1 {
                scanline.push('#');
            } else {
                scanline.push('.');
            }
        }

        crt.push(scanline);
    }

    crt.join("\r\n")
}

struct CPU {
    x: i64,

    program: Vec<Instruction>,
    instruction: usize,
    cycles: u8,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> CPU {
        let cycles = program[0].cycles();

        CPU {
            x: 1,
            program,
            instruction: 0,
            cycles,
        }
    }

    fn next(&mut self) -> i64 {
        if self.cycles == 0 {
            let instruction = &self.program[self.instruction];

            // apply instruction
            match instruction {
                Instruction::Addx(x) => self.x += x,
                Instruction::Noop => self.x += 0,
            }

            self.instruction += 1;
            self.cycles = self.program[self.instruction].cycles();
        }

        self.cycles -= 1;
        self.x
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");

            match split.next().expect("missing instruction") {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(
                    split
                        .next()
                        .expect("missing argument for addx")
                        .parse()
                        .expect("addx argument unparsable"),
                ),
                _ => panic!("unknown instruction"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");
    const PART2_OUTPUT: &str = include_str!("test2output");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT)
    }
}
