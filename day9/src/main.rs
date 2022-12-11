use std::alloc::System;
use std::borrow::BorrowMut;
use std::cmp::{max, min, Ordering};
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;
use Ordering::{Equal, Greater, Less};

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let start = SystemTime::now();
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
    println!("Time: {:?}", start.elapsed().unwrap());
}

// ------ Part 1 -----
// This rope bridge creaks as you walk along it. You aren't sure how old it is, or whether it can even support your weight.
// It seems to support the Elves just fine, though. The bridge spans a gorge which was carved out by the massive river far below you.
// You step carefully; as you do, the ropes stretch and twist. You decide to distract yourself by modeling rope physics; maybe you can even figure out where not to step.
// Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. If the head moves far enough away from the tail, the tail is pulled toward the head.
// Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a two-dimensional grid. Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.
// Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):
// If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:
// Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:
//You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the tail both start at the same position, overlapping.
//
// For example:
//
// R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2
// This series of motions moves the head right four steps, then up four steps, then left three steps, then down one step, and so on. After each step, you'll need to update the position of the tail if the step means the head is no longer adjacent to the tail. Visually, these motions occur as follows (s marks the starting position as a reference point):

// After simulating the rope, you can count up all of the positions the tail visited at least once. In this diagram, s again marks the starting position (which the tail also visited) and # marks other positions the tail visited:
//
// ..##..
// ...##.
// .####.
// ....#.
// s###..
// So, there are 13 positions the tail visited at least once.
//
// Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u16,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();

        Ok(Instruction {
            direction: split[0].parse().expect("Could not parse direction"),
            steps: split[1].parse().expect("Could not parse int"),
        })
    }
}

fn parse_instructions(text: String) -> Vec<Instruction> {
    text.lines().filter_map(|line| line.parse().ok()).collect()
}

#[derive(Debug, Default)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Knot { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    fn from_knot(knot: &Knot) -> Self {
        Self::new(knot.x, knot.y)
    }
}

fn simulate_head(head: &mut Knot, direction: &Direction) {
    match direction {
        Direction::Left => head.x -= 1,
        Direction::Right => head.x += 1,
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
    }
}

fn is_horizontal_away(head: &Knot, tail: &Knot) -> bool {
    let x_diff = (head.x - tail.x).abs();
    let y_diff = (head.y - tail.y).abs();

    x_diff == 2 && y_diff == 0
}

fn is_vertical_away(head: &Knot, tail: &Knot) -> bool {
    let x_diff = (head.x - tail.x).abs();
    let y_diff = (head.y - tail.y).abs();
    x_diff == 0 && y_diff == 2
}

fn is_valid_pos(head: &Knot, tail: &Knot) -> bool {
    let x_diff = (head.x - tail.x).abs();
    let y_diff = (head.y - tail.y).abs();

    x_diff <= 1 && y_diff <= 1
}

fn simulate_tail(head: &Knot, tail: &mut Knot) {
    if tail.x == head.x && tail.y == head.y {
        return;
    }

    if is_valid_pos(head, tail) {
        return;
    }

    if is_horizontal_away(head, tail) {
        // Move horizontally to keep up
        move_x(head, tail);
        return;
    }

    if is_vertical_away(head, tail) {
        // Move vertically to keep up
        move_y(head, tail);
    }

    move_diagonal(head, tail);
}

fn move_y(head: &Knot, tail: &mut Knot) {
    match head.y.cmp(&tail.y) {
        Less => tail.y -= 1,
        Greater => tail.y += 1,
        Equal => {}
    }
}

fn move_x(head: &Knot, tail: &mut Knot) {
    match head.x.cmp(&tail.x) {
        Less => tail.x -= 1,
        Greater => tail.x += 1,
        Equal => {}
    }
}

fn move_diagonal(head: &Knot, tail: &mut Knot) {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;
    match (x_diff.cmp(&0), y_diff.cmp(&0)) {
        (Less, Less) => {
            tail.x -= 1;
            tail.y -= 1;
        }
        (Less, Greater) => {
            tail.x -= 1;
            tail.y += 1;
        }
        (Greater, Less) => {
            tail.x += 1;
            tail.y -= 1;
        }
        (Greater, Greater) => {
            tail.x += 1;
            tail.y += 1;
        }
        _ => {}
    }
}

fn simulate(instructions: Vec<Instruction>, head: &mut Knot, tail: &mut Knot) -> HashSet<Pos> {
    let mut visited = HashSet::new();

    for instruction in instructions {
        let direction = instruction.direction;
        for _ in 0..instruction.steps {
            simulate_head(head, &direction);
            simulate_tail(head, tail);

            visited.insert(Pos::from_knot(tail));
        }
    }

    visited
}

fn process_part1(text: String) -> i32 {
    let instructions = parse_instructions(text);

    let mut head = Default::default();
    let mut tail = Default::default();
    let positions = simulate(instructions, &mut head, &mut tail);

    positions.len() as i32
}

// --- Part Two ---
// A rope snaps! Suddenly, the river is getting a lot closer than you remember. The bridge is still there, but some of the ropes that broke are now whipping toward you as you fall through the air!
// The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid being hit. Fortunately, your simulation can be extended to support longer ropes.
// Rather than two knots, you now must simulate a rope consisting of ten knots. One knot is still the head of the rope and moves according to the series of motions. Each knot further down the rope follows the knot in front of it using the same rules as before.

fn simulate_multi_knots(
    instructions: Vec<Instruction>,
    head: &mut Knot,
    tails: &mut [Knot],
) -> HashSet<Pos> {
    let mut visited = HashSet::new();

    for instruction in instructions {
        let direction = instruction.direction;
        for _ in 0..instruction.steps {
            simulate_head(head, &direction);

            // Simulate the tails to the previous knot
            let mut prev_tail = head.borrow_mut();
            for tail in tails.iter_mut() {
                simulate_tail(prev_tail, tail);
                prev_tail = tail;
            }

            // Add the last tail to the visited positions
            visited.insert(Pos::from_knot(tails.last().unwrap()));
        }
    }

    visited
}

fn print_debug(head: &Knot, tails: &mut [Knot], size: i32) {
    // Print a grid with . for empty and H for head and the index of the tail for the tail
    let min_x = min(min(head.x, tails.iter().map(|k| k.x).min().unwrap()), 0);
    let max_x = max(max(head.x, tails.iter().map(|k| k.x).max().unwrap()), size);
    let min_y = min(min(head.y, tails.iter().map(|k| k.y).min().unwrap()), 0);
    let max_y = max(max(head.y, tails.iter().map(|k| k.y).max().unwrap()), size);

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if x == head.x && y == head.y {
                print!("H");
            } else if let Some(_tail) = tails.iter().find(|k| k.x == x && k.y == y) {
                print!(
                    "{}",
                    tails.iter().position(|k| k.x == x && k.y == y).unwrap() + 1
                );
            } else if x == 0 && y == 0 {
                print!("s")
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("=======")
}

fn process_part2(text: String) -> i32 {
    let instructions = parse_instructions(text);

    let mut head = Default::default();

    // Create 9 tails
    let mut tails = (0..9).map(|_| Default::default()).collect::<Vec<_>>();
    let positions = simulate_multi_knots(instructions, &mut head, &mut tails);

    // print_debug(&head, &tails);

    positions.len() as i32
}

#[test]
fn when_moving_diagonal_expect_tail_to_also_move_diagonal() {
    let head = Knot::new(2, 2);
    let mut tail = Knot::new(0, 0);

    simulate_tail(&head, &mut tail);

    assert_eq!(tail.x, 1);
    assert_eq!(tail.y, 1);
}
