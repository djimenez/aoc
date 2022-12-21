use regex::Regex;
use std::collections::HashMap;

const DEFAULT_INPUT: &str = include_str!("input");

pub fn run() {
    println!("part1: {}", part1(DEFAULT_INPUT));
    println!("part2: {}", part2(DEFAULT_INPUT));
}

fn part1(input: &str) -> i64 {
    let monkeys = parse_input(input);

    eval(&monkeys, "root")
}

fn part2(input: &str) -> i64 {
    let monkeys = parse_input(input);

    let ast = construct_ast(&monkeys, "root");
    let ast = solve_ast(ast);

    // match it against In = Val and return the value
    // if let Ast::Eq(box Ast::In, box Ast::Val(value)) = ast {
    if let Ast::Eq(left, right) = ast {
        if let (Ast::In, Ast::Val(value)) = (*left, *right) {
            return value;
        }
    }

    panic!("did not get expected simplified ast");
}

#[derive(Debug)]
enum Job {
    Id(i64),

    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug, Eq, PartialEq)]
enum Ast {
    Eq(Box<Self>, Box<Self>),

    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),

    Val(i64),
    In,
}

impl Ast {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut result = String::new();

        match self {
            Ast::In => {
                result.push_str("INPUT");
            }
            Ast::Val(value) => {
                result.push_str(&value.to_string());
            }
            Ast::Eq(left, right) => {
                result.push_str(&left.to_string());
                result.push_str(" = ");
                result.push_str(&right.to_string());
            }
            Ast::Add(left, right) => {
                result.push_str("(");
                result.push_str(&left.to_string());
                result.push_str(") + (");
                result.push_str(&right.to_string());
                result.push_str(")");
            }
            Ast::Sub(left, right) => {
                result.push_str("(");
                result.push_str(&left.to_string());
                result.push_str(") - (");
                result.push_str(&right.to_string());
                result.push_str(")");
            }
            Ast::Mul(left, right) => {
                result.push_str("(");
                result.push_str(&left.to_string());
                result.push_str(") * (");
                result.push_str(&right.to_string());
                result.push_str(")");
            }
            Ast::Div(left, right) => {
                result.push_str("(");
                result.push_str(&left.to_string());
                result.push_str(") / (");
                result.push_str(&right.to_string());
                result.push_str(")");
            }
        }

        result
    }
}

fn parse_input(input: &str) -> HashMap<String, Job> {
    let line_re = Regex::new(r"^([a-z]+): ([0-9]+|[a-z]+ [-+*/] [a-z]+)").unwrap();
    let job_re = Regex::new(r"^([a-z]+) ([-+*/]) ([a-z]+)").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = line_re.captures(line).unwrap();

            let monkey = String::from(&captures[1]);
            let job = &captures[2];

            if let Ok(number) = job.parse::<i64>() {
                (monkey, Job::Id(number))
            } else {
                let captures = job_re.captures(job).unwrap();

                let job = match &captures[2] {
                    "+" => Job::Add(String::from(&captures[1]), String::from(&captures[3])),
                    "-" => Job::Sub(String::from(&captures[1]), String::from(&captures[3])),
                    "*" => Job::Mul(String::from(&captures[1]), String::from(&captures[3])),
                    "/" => Job::Div(String::from(&captures[1]), String::from(&captures[3])),

                    _ => panic!("unimplemented monkey operator"),
                };

                (monkey, job)
            }
        })
        .collect()
}

// simple recursive traversal, does no caching or value replacement
fn eval(monkeys: &HashMap<String, Job>, monkey: &str) -> i64 {
    let job = &monkeys[monkey];

    match job {
        Job::Id(value) => *value,

        Job::Add(left, right) => eval(monkeys, left.as_str()) + eval(monkeys, right.as_str()),
        Job::Sub(left, right) => eval(monkeys, left.as_str()) - eval(monkeys, right.as_str()),
        Job::Mul(left, right) => eval(monkeys, left.as_str()) * eval(monkeys, right.as_str()),
        Job::Div(left, right) => eval(monkeys, left.as_str()) / eval(monkeys, right.as_str()),
    }
}

fn construct_ast(monkeys: &HashMap<String, Job>, monkey: &str) -> Ast {
    if monkey == "root" {
        let job = &monkeys[monkey];
        let (left, right) = match job {
            Job::Add(left, right) => (left, right),
            Job::Sub(left, right) => (left, right),
            Job::Mul(left, right) => (left, right),
            Job::Div(left, right) => (left, right),

            _ => panic!("root job makes no sense for part 2"),
        };

        Ast::Eq(
            Box::new(construct_ast(monkeys, left.as_str())),
            Box::new(construct_ast(monkeys, right.as_str())),
        )
    } else if monkey == "humn" {
        Ast::In
    } else {
        let job = &monkeys[monkey];

        match job {
            Job::Id(value) => Ast::Val(*value),

            Job::Add(left, right) => Ast::Add(
                Box::new(construct_ast(monkeys, left.as_str())),
                Box::new(construct_ast(monkeys, right.as_str())),
            ),
            Job::Sub(left, right) => Ast::Sub(
                Box::new(construct_ast(monkeys, left.as_str())),
                Box::new(construct_ast(monkeys, right.as_str())),
            ),
            Job::Mul(left, right) => Ast::Mul(
                Box::new(construct_ast(monkeys, left.as_str())),
                Box::new(construct_ast(monkeys, right.as_str())),
            ),
            Job::Div(left, right) => Ast::Div(
                Box::new(construct_ast(monkeys, left.as_str())),
                Box::new(construct_ast(monkeys, right.as_str())),
            ),
        }
    }
}

fn eval_ast(ast: Ast) -> Ast {
    match ast {
        Ast::Val(value) => Ast::Val(value),
        Ast::In => Ast::In,

        Ast::Add(left, right) => {
            let left_eval = eval_ast(*left);
            let right_eval = eval_ast(*right);

            match (left_eval, right_eval) {
                (Ast::Val(left_value), Ast::Val(right_value)) => Ast::Val(left_value + right_value),

                // simplify additive identities for trees
                (left_eval, Ast::Val(0)) => left_eval,
                (Ast::Val(0), right_eval) => right_eval,

                (left_eval, right_eval) => Ast::Add(Box::new(left_eval), Box::new(right_eval)),
            }
        }
        Ast::Sub(left, right) => {
            let left_eval = eval_ast(*left);
            let right_eval = eval_ast(*right);

            match (left_eval, right_eval) {
                (Ast::Val(left_value), Ast::Val(right_value)) => Ast::Val(left_value - right_value),

                // simplify additive identity for trees
                (left_eval, Ast::Val(0)) => left_eval,

                (left_eval, right_eval) => Ast::Sub(Box::new(left_eval), Box::new(right_eval)),
            }
        }
        Ast::Mul(left, right) => {
            let left_eval = eval_ast(*left);
            let right_eval = eval_ast(*right);

            match (left_eval, right_eval) {
                // if we multiply by 0, simplify to 0
                (_, Ast::Val(0)) | (Ast::Val(0), _) => Ast::Val(0),

                // if we multiply other values, evaluate
                (Ast::Val(left_value), Ast::Val(right_value)) => Ast::Val(left_value * right_value),

                // if we multiply tree by 1, simplify
                (left_eval, Ast::Val(1)) => left_eval,
                (Ast::Val(1), right_eval) => right_eval,

                (left_eval, right_eval) => Ast::Mul(Box::new(left_eval), Box::new(right_eval)),
            }
        }
        Ast::Div(left, right) => {
            let left_eval = eval_ast(*left);
            let right_eval = eval_ast(*right);

            match (left_eval, right_eval) {
                // if we divide 0 or by 1, simplify
                (Ast::Val(0), _) => Ast::Val(0),
                (left_eval, Ast::Val(1)) => left_eval,

                // allow divide by 0 to panic on its own
                (Ast::Val(left_value), Ast::Val(right_value)) => Ast::Val(left_value / right_value),

                (left_eval, right_eval) => Ast::Div(Box::new(left_eval), Box::new(right_eval)),
            }
        }
        Ast::Eq(left, right) => {
            // only extracted to recusively simplify with eval
            let left_eval = eval_ast(*left);
            let right_eval = eval_ast(*right);

            Ast::Eq(Box::new(left_eval), Box::new(right_eval))
        }
    }
}

// this was originally just in eval_ast, but I split it out
fn solve_ast(ast: Ast) -> Ast {
    if let Ast::Eq(left, right) = ast {
        let mut eq_left = eval_ast(*left);
        let mut eq_right = eval_ast(*right);

        // move operations and values between left and right sides until fully simplified
        loop {
            //println!("{} = {}", left_eval.to_string(), right_eval.to_string());
            match eq_left {
                Ast::Val(left_value) => {
                    if let Ast::Val(_) = eq_right {
                        // Val = Val, stop point
                        break;
                    } else {
                        // swap sides if Val is on left side and right also isn't a val
                        eq_left = eq_right;
                        eq_right = Ast::Val(left_value);
                    }
                }
                Ast::Add(left, right) => match (*left, *right) {
                    (Ast::Val(left_value), right) => {
                        //println!("subtracting {} from both sides", left_value);
                        eq_left = right;
                        eq_right =
                            eval_ast(Ast::Sub(Box::new(eq_right), Box::new(Ast::Val(left_value))));
                    }
                    (left, Ast::Val(right_value)) => {
                        //println!("subtracting {} from both sides", right_value);
                        eq_left = left;
                        eq_right = eval_ast(Ast::Sub(
                            Box::new(eq_right),
                            Box::new(Ast::Val(right_value)),
                        ));
                    }
                    (left, right) => {
                        // we unboxed them to match, have to rebox them
                        eq_left = Ast::Add(Box::new(left), Box::new(right));
                        break;
                    }
                },
                Ast::Sub(left, right) => match (*left, *right) {
                    (Ast::Val(left_value), right) => {
                        if let Ast::Val(right_value) = eq_right {
                            //println!("adding tree to both sides and subtracting {}", right_value);
                            // a - tree = c => a - c = tree
                            eq_left = Ast::Val(left_value - right_value);
                            eq_right = right;
                        } else {
                            eq_left = Ast::Sub(Box::new(Ast::Val(left_value)), Box::new(right));

                            break;
                        }
                    }
                    (left, Ast::Val(right_value)) => {
                        //println!("adding {} to both sides", right_value);
                        eq_left = left;
                        eq_right = eval_ast(Ast::Add(
                            Box::new(eq_right),
                            Box::new(Ast::Val(right_value)),
                        ));
                    }
                    (left, right) => {
                        // we unboxed them to match, have to rebox them
                        eq_left = Ast::Sub(Box::new(left), Box::new(right));
                        break;
                    }
                },
                Ast::Mul(left, right) => match (*left, *right) {
                    (Ast::Val(left_value), right) => {
                        //println!("dividing by {} on both sides", left_value);
                        eq_left = right;
                        eq_right =
                            eval_ast(Ast::Div(Box::new(eq_right), Box::new(Ast::Val(left_value))));
                    }
                    (left, Ast::Val(right_value)) => {
                        //println!("dividing by {} on both sides", right_value);
                        eq_left = left;
                        eq_right = eval_ast(Ast::Div(
                            Box::new(eq_right),
                            Box::new(Ast::Val(right_value)),
                        ));
                    }
                    (left, right) => {
                        // we unboxed them to match, have to rebox them
                        eq_left = Ast::Mul(Box::new(left), Box::new(right));
                        break;
                    }
                },
                Ast::Div(left, right) => match (*left, *right) {
                    (Ast::Val(left_value), right) => {
                        if let Ast::Val(right_value) = eq_right {
                            //println!("multiplying by tree and dividing by {} on both sides", right_value);
                            // weird case: val1 / tree = val2 => va1 / val2 = tree
                            eq_left = Ast::Val(left_value / right_value);
                            eq_right = right;
                        } else {
                            // have to rebox what we dereferenced to match
                            eq_left = Ast::Div(Box::new(Ast::Val(left_value)), Box::new(right));
                            break;
                        }
                    }
                    (left, Ast::Val(right_value)) => {
                        //println!("multiplying by {} on both sides", right_value);
                        eq_left = left;
                        eq_right = eval_ast(Ast::Mul(
                            Box::new(eq_right),
                            Box::new(Ast::Val(right_value)),
                        ));
                    }
                    (left, right) => {
                        // we unboxed them to match, have to rebox them
                        eq_left = Ast::Div(Box::new(left), Box::new(right));
                        break;
                    }
                },

                // In or Eq
                _ => break,
            }
        }

        return Ast::Eq(Box::new(eq_left), Box::new(eq_right));
    }

    panic!("not an Eq AST");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 152)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 301)
    }

    #[test]
    fn test_solve_ast_add() {
        let ast = Ast::Eq(
            Box::new(Ast::Add(Box::new(Ast::In), Box::new(Ast::Val(5)))),
            Box::new(Ast::Val(11)),
        );

        let ast = solve_ast(ast);

        if let Ast::Eq(left, right) = solve_ast(ast) {
            assert_eq!(*left, Ast::In);
            assert_eq!(*right, Ast::Val(6));
        }
    }

    #[test]
    fn test_solve_ast_sub() {
        // in - 5 = 11
        let ast = Ast::Eq(
            Box::new(Ast::Sub(Box::new(Ast::In), Box::new(Ast::Val(5)))),
            Box::new(Ast::Val(11)),
        );

        let ast = solve_ast(ast);

        if let Ast::Eq(left, right) = solve_ast(ast) {
            assert_eq!(*left, Ast::In);
            assert_eq!(*right, Ast::Val(16));
        }

        // 5 - in = 11
        let ast = Ast::Eq(
            Box::new(Ast::Sub(Box::new(Ast::Val(16)), Box::new(Ast::In))),
            Box::new(Ast::Val(11)),
        );

        let ast = solve_ast(ast);

        if let Ast::Eq(left, right) = solve_ast(ast) {
            assert_eq!(*left, Ast::In);
            assert_eq!(*right, Ast::Val(5));
        }
    }
}
