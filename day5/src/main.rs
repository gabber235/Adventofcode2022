use regex::Regex;
use std::fs;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// --- Part 1 ---
// The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
// The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
// The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
// They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
//
//     [D]
// [N] [C]
// [Z] [M] [P]
// 1   2   3
//
// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
// In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.
//
// Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//
// [D]
// [N] [C]
// [Z] [M] [P]
// 1   2   3
// In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:
//
// [Z]
// [N]
// [C] [D]
// [M] [P]
// 1   2   3
// Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:
//
// [Z]
// [N]
// [M]     [D]
// [C]     [P]
// 1   2   3
// Finally, one crate is moved from stack 1 to stack 2:
//
// [Z]
// [N]
// [D]
// [C] [M] [P]
// 1   2   3
// The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.
//
// After the rearrangement procedure completes, what crate ends up on top of each stack?

fn split_starting_stacks_from_procedure(text: &str) -> (Vec<&str>, Vec<&str>) {
    let split = text.split("\n\n").collect::<Vec<&str>>();
    let mut starting_stacks: Vec<&str> = split[0].lines().collect();
    let procedure = split[1].lines().collect();

    starting_stacks.pop();
    (starting_stacks, procedure)
}

#[derive(Debug)]
struct Stack {
    stack: Vec<char>,
}

impl Stack {
    fn pop(&mut self) -> char {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, c: char) {
        self.stack.push(c);
    }

    fn top(&self) -> char {
        self.stack[self.stack.len() - 1]
    }
}

/// Parses the strings from bottom to top. The first element is the bottom of the stack.
/// Returns a vector of the stacks.
///
/// # Example
/// ```
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///
/// => vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
fn parse_stacks(stacks: Vec<&str>) -> Vec<Stack> {
    let mut parsed_stacks = Vec::new();

    let count = stacks.last().unwrap().chars().count();
    for i in (1..count).step_by(4) {
        parsed_stacks.push(Stack { stack: Vec::new() });
    }

    for stack in stacks.iter().rev() {
        for i in (1..=stack.len()).step_by(4) {
            let char = stack.chars().nth(i).unwrap();
            if char != ' ' {
                parsed_stacks[i / 4].stack.push(char);
            }
        }
    }

    parsed_stacks
}

#[derive(Debug)]
struct Procedure {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_procedure(procedure: Vec<&str>) -> Vec<Procedure> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    procedure
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Procedure {
                count: captures[1].parse().unwrap(),
                from: captures[2].parse().unwrap(),
                to: captures[3].parse().unwrap(),
            }
        })
        .collect()
}

fn parse_input(text: String) -> (Vec<Stack>, Vec<Procedure>) {
    let (starting_stacks, procedure) = split_starting_stacks_from_procedure(&text);
    let stacks = parse_stacks(starting_stacks);
    let procedure = parse_procedure(procedure);
    (stacks, procedure)
}

fn process_part1(text: String) -> String {
    let (mut stacks, procedure) = parse_input(text);

    for step in procedure {
        for _ in 0..step.count {
            let c = stacks[step.from - 1].pop();
            stacks[step.to - 1].push(c);
        }
    }

    stacks.iter().map(|stack| stack.top()).collect::<String>()
}

// --- Part Two ---
// As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
// Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
// The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
// Again considering the example above, the crates begin in the same configuration:
//
// [D]
// [N] [C]
// [Z] [M] [P]
// 1   2   3
// Moving a single crate from stack 2 to stack 1 behaves the same as before:
//
// [D]
// [N] [C]
// [Z] [M] [P]
// 1   2   3
// However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:
//
// [D]
// [N]
// [C] [Z]
// [M] [P]
// 1   2   3
// Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
//
// [D]
// [N]
// [C]     [Z]
// [M]     [P]
// 1   2   3
// Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:
//
// [D]
// [N]
// [Z]
// [M] [C] [P]
// 1   2   3
// In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.
//
// Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?

fn process_part2(text: String) -> String {
    let (mut stacks, procedure) = parse_input(text);

    for step in procedure {
        let mut to_move = Vec::new();
        for _ in 0..step.count {
            let c = stacks[step.from - 1].pop();
            to_move.push(c);
        }
        for c in to_move.iter().rev() {
            stacks[step.to - 1].push(*c);
        }
    }

    stacks.iter().map(|stack| stack.top()).collect::<String>()
}
