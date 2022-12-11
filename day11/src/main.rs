use crate::Context::{StressFree, Stressed};
use regex::Regex;
use std::collections::VecDeque;
use std::fs;
use std::str::{FromStr, Lines};

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

#[derive(Debug)]
struct Monkey {
    _id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,     // Which number an item should be divisible by
    if_true: u64,  // The next monkey to throw to if the item is divisible by test
    if_false: u64, // The next monkey to throw to if the item is not divisible by test

    inspect_count: u64,
}

impl FromStr for Monkey {
    type Err = ();

    /// Parse the following format:
    /// Monkey 0:
    ///   Starting items: 79, 98
    ///   Operation: new = old * 19
    ///   Test: divisible by 23
    ///     If true: throw to monkey 2
    ///     If false: throw to monkey 3
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Parse the monkey id
        let id_regex = Regex::new(r"Monkey (\d+):").unwrap();
        let id = parse_line(&mut lines, id_regex);

        // Parse the starting items
        let items_regex = Regex::new(r"Starting items: (.*)").unwrap();
        let items = parse_line::<String>(&mut lines, items_regex)
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();

        // Parse the operation
        let operation_regex = Regex::new(r"Operation: new = old (.*)").unwrap();
        let operation = parse_line(&mut lines, operation_regex);

        // Parse the test
        let test_regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        let test = parse_line(&mut lines, test_regex);

        // Parse the if_true
        let if_true_regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
        let if_true = parse_line(&mut lines, if_true_regex);

        // Parse the if_false
        let if_false_regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        let if_false = parse_line(&mut lines, if_false_regex);

        Ok(Monkey {
            _id: id,
            items,
            operation,
            test,
            if_true,
            if_false,
            inspect_count: 0,
        })
    }
}

fn parse_line<T>(lines: &mut Lines, regex: Regex) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    regex
        .captures(lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<T>()
        .unwrap()
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        let second = self.operand.apply(old);
        self.operator.apply(old, second)
    }
}

impl FromStr for Operation {
    type Err = ();

    /// Parse the following format:
    /// new = old * 19
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let operator = parts.next().unwrap().parse::<Operator>().unwrap();
        let operand = parts.next().unwrap().parse::<Operand>().unwrap();
        Ok(Operation { operator, operand })
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
            Operator::Subtract => a - b,
        }
    }
}

impl FromStr for Operator {
    type Err = ();

    /// Parse the following format:
    /// new = old * 19
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "-" => Ok(Operator::Subtract),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Constant(u64),
    Item,
}

impl Operand {
    fn apply(&self, item: u64) -> u64 {
        match self {
            Operand::Constant(c) => *c,
            Operand::Item => item,
        }
    }
}

impl FromStr for Operand {
    type Err = ();

    /// Parse the following format:
    /// new = old * 19
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Item),
            _ => Ok(Operand::Constant(s.parse::<u64>().unwrap())),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Context {
    StressFree,
    Stressed { total: u64 },
}

struct Inspection {
    monkey: usize,
    item: u64,
}

fn simplify_item(context: &Context, item: u64) -> u64 {
    match context {
        StressFree => item / 3,
        Stressed { total } => item % total,
    }
}

/// Inspect the first item in the monkey's inventory and return the result
/// of the inspection.
fn inspect_item(context: &Context, monkey: &mut Monkey) -> Option<Inspection> {
    let item = monkey.items.pop_front()?;

    monkey.inspect_count += 1;

    let new_item = monkey.operation.apply(item);
    let new_item = simplify_item(context, new_item);
    let next_monkey = if new_item % monkey.test == 0 {
        monkey.if_true
    } else {
        monkey.if_false
    };

    Some(Inspection {
        monkey: next_monkey as usize,
        item: new_item,
    })
}

fn simulate_monkey(context: &Context, monkey: &mut Monkey) -> Vec<Inspection> {
    let mut inspections = Vec::new();
    while let Some(inspection) = inspect_item(context, monkey) {
        inspections.push(inspection);
    }
    inspections
}

fn simulate_round(context: &Context, monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let inspections = simulate_monkey(context, &mut monkeys[i]);
        for inspection in inspections {
            monkeys[inspection.monkey].items.push_back(inspection.item);
        }
    }
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut counts: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

fn process_part1(text: String) -> u64 {
    let mut monkeys: Vec<Monkey> = text
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();

    for _ in 0..20 {
        simulate_round(&StressFree, &mut monkeys);
    }

    calculate_monkey_business(&monkeys)
}

fn process_part2(text: String) -> u64 {
    let mut monkeys: Vec<Monkey> = text
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();

    let context = Stressed {
        total: monkeys.iter().map(|m| m.test).product(),
    };

    for _ in 0..10000 {
        simulate_round(&context, &mut monkeys);
    }

    calculate_monkey_business(&monkeys)
}
