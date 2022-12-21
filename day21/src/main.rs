use evalexpr::{eval_int_with_context_mut, HashMapContext, Value};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

use crate::Math::{Human, Number};
use crate::Operation::{LeftDivide, LeftSubtract};
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

#[derive(Debug)]
enum Monkey {
    Number(i64),
    Math(String, String, Operation),
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse::<i64>() {
            Ok(Monkey::Number(number))
        } else {
            let mut parts = s.split_whitespace();
            let left = parts.next().unwrap().to_string();
            let op = parts.next().unwrap().to_string();
            let right = parts.next().unwrap().to_string();
            Ok(Monkey::Math(left, right, op.parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftSubtract,
    LeftDivide,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err(()),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
            Operation::LeftSubtract => write!(f, "-"),
            Operation::LeftDivide => write!(f, "/"),
        }
    }
}

/// Parse input into a map of monkeys
/// The key is the monkey's name
/// Examples:
/// root: pppw + sjmn
/// dbpl: 5
/// cczh: sllz + lgvd
/// zczc: 2
/// ptdq: humn - dvpt
fn parse_input(text: String) -> HashMap<String, Monkey> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_string();
        let monkey = parts.next().unwrap().parse().unwrap();
        map.insert(name, monkey);
    }
    map
}

fn find_value_for_monkey(monkey: &Monkey, map: &HashMap<String, Monkey>) -> i64 {
    match monkey {
        Monkey::Number(n) => *n,
        Monkey::Math(left, right, op) => {
            let left_value = find_value_for_monkey(map.get(left).unwrap(), map);
            let right_value = find_value_for_monkey(map.get(right).unwrap(), map);
            match op {
                Operation::Add => left_value + right_value,
                Operation::Subtract => left_value - right_value,
                Operation::Multiply => left_value * right_value,
                Operation::Divide => left_value / right_value,
                Operation::LeftSubtract => right_value - left_value,
                Operation::LeftDivide => right_value / left_value,
            }
        }
    }
}

fn process_part1(text: String) -> i64 {
    let map = parse_input(text);

    find_value_for_monkey(map.get("root").unwrap(), &map)
}

#[derive(Debug, Clone)]
enum Math {
    Number(i64),
    Human(Vec<(i64, Operation)>),
}

/// Find the values of the monkeys. The monkey with the name "humn" is the one we are looking for.
/// That will be our x.
fn find_math(id: &String, monkey: &Monkey, map: &HashMap<String, Monkey>) -> Math {
    if id == "humn" {
        return Human(vec![]);
    }
    match monkey {
        Monkey::Number(n) => Number(*n),
        Monkey::Math(left, right, op) => {
            let left_value = find_math(left, map.get(left).unwrap(), map);
            let right_value = find_math(right, map.get(right).unwrap(), map);

            match (left_value, right_value) {
                (Number(left), Number(right)) => match op {
                    Operation::Add => Number(left + right),
                    Operation::Subtract => Number(left - right),
                    Operation::Multiply => Number(left * right),
                    Operation::Divide => Number(left / right),
                    Operation::LeftSubtract => Number(right - left),
                    Operation::LeftDivide => Number(right / left),
                },
                (Number(left), Human(mut human)) => {
                    human.push((
                        left,
                        match op {
                            Operation::Subtract => LeftSubtract,
                            Operation::Divide => LeftDivide,
                            x => x.clone(),
                        },
                    ));
                    Human(human)
                }
                (Human(mut human), Number(right)) => {
                    human.push((right, op.clone()));
                    Human(human)
                }
                _ => panic!("Not implemented"),
            }
        }
    }
}

fn unwrap_human(human: &Math, value: &Math) -> i64 {
    let Human(vec) = human else {
        panic!("Not a human");
    };
    let Number(mut result) = value else {
        panic!("Not a number");
    };

    for (n, op) in vec.into_iter().rev() {
        match op {
            Operation::Add => result = result - n,
            Operation::Subtract => result = result + n,
            Operation::Multiply => result = result / n,
            Operation::Divide => result = result * n,
            Operation::LeftSubtract => result = n - result,
            Operation::LeftDivide => result = n / result,
        }
    }
    result
}

fn process_part2(text: String) -> i64 {
    let map = parse_input(text);

    let (left, right) = if let Monkey::Math(left, right, _) = map.get("root").unwrap() {
        (left, right)
    } else {
        panic!("root is not a math operation");
    };

    let left_value = find_math(left, map.get(left).unwrap(), &map);
    let right_value = find_math(right, map.get(right).unwrap(), &map);

    if let Human(_) = left_value {
        unwrap_human(&left_value, &right_value)
    } else {
        unwrap_human(&right_value, &left_value)
    }
}
