use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// Your handheld device has located an alternative exit from the cave for you and the elephants. The ground is rumbling almost continuously now, but the strange valves bought you some time. It's definitely getting warmer in here, though.
// The tunnels eventually open into a very tall, narrow chamber. Large, oddly-shaped rocks are falling into the chamber from above, presumably due to all the rumbling. If you can't work out where the rocks will fall next, you might be crushed!
// The five types of rocks have the following peculiar shapes, where # is rock and . is empty space:

// ####
//
// .#.
// ###
// .#.
//
// ..#
// ..#
// ###
//
// #
// #
// #
// #
//
// ##
// ##
// The rocks fall in the order shown above: first the - shape, then the + shape, and so on. Once the end of the list is reached, the same order repeats: the - shape falls first, sixth, 11th, 16th, etc.
// The rocks don't spin, but they do get pushed around by jets of hot gas coming out of the walls themselves. A quick scan reveals the effect the jets of hot gas will have on the rocks as they fall (your puzzle input).
// For example, suppose this was the jet pattern in your cave:

// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
// In jet patterns, < means a push to the Left, while > means a push to the Right. The pattern above means that the jets will push a falling rock Right, then Right, then Right, then Left, then Left, then Right, and so on. If the end of the list is reached, it repeats.
// The tall, vertical chamber is exactly seven units wide. Each rock appears so that its Left edge is two units away from the Left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
// After a rock appears, it alternates between being pushed by a jet of hot gas one unit (in the direction indicated by the next symbol in the jet pattern) and then falling one unit down. If any movement would cause any part of the rock to move into the walls, floor, or a stopped rock, the movement instead does not occur. If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock, the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.
// To prove to the elephants your simulation is accurate, they want to know how tall the tower will get after 2022 rocks have stopped (but before the 2023rd rock begins falling). In this example, the tower of rocks will be 3068 units tall.

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Minus,
    Plus,
    InvertedL,
    Line,
    Square,
}

impl Rock {
    fn local_space_points(&self) -> Vec<Point> {
        match self {
            Rock::Minus => vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ],
            Rock::Plus => vec![
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(1, 2),
                Point::new(0, 1),
                Point::new(2, 1),
            ],
            Rock::InvertedL => vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(2, 2),
            ],
            Rock::Line => vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
            ],
            Rock::Square => vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
        }
    }

    fn world_space_points(&self, relative: &Point) -> Vec<Point> {
        self.local_space_points()
            .iter()
            .map(|p| *p + *relative)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Room {
    width: usize,
    // All the points where a rock is stopped
    solids: Vec<Point>,
}

impl Room {
    fn collides_with_wall(&self, rock: &Rock, point: &Point) -> bool {
        let points = rock.world_space_points(point);
        points
            .iter()
            .any(|p| p.x < 0 || p.x >= self.width as i32 || self.solids.contains(p))
    }

    fn collides_with_bottom(&self, rock: &Rock, point: &Point) -> bool {
        let points = rock.world_space_points(point);
        points.iter().any(|p| p.y < 0 || self.solids.contains(p))
    }

    fn print(&self, rock: &Rock, rock_position: &Point) {
        let points = rock.world_space_points(rock_position);

        let max_y = self
            .solids
            .iter()
            .map(|p| p.y)
            .max()
            .unwrap_or(0)
            .max(points.iter().map(|p| p.y).max().unwrap_or(0));
        let max_x = self.width as i32;

        for y in (-1..=max_y).rev() {
            for x in -1..=max_x {
                if x == -1 || x == max_x {
                    print!("|");
                } else if y == -1 {
                    print!("-");
                } else if points.contains(&Point::new(x, y)) {
                    print!("@");
                } else if self.solids.contains(&Point::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn height(&self) -> i32 {
        self.solids.iter().map(|p| p.y).max().unwrap_or(0) + 1
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_y = self.solids.iter().map(|p| p.y).max().unwrap_or(0);
        let max_x = self.width as i32;

        for y in (-1..=max_y).rev() {
            for x in -1..=max_x {
                if x == -1 || x == max_x {
                    write!(f, "|")?;
                } else if y == -1 {
                    write!(f, "-")?;
                } else if self.solids.contains(&Point::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

fn process_part1(text: String) -> i32 {
    let directions = parse_input(&text);

    let mut room = Room {
        width: 7,
        solids: vec![],
    };

    let rock_types = vec![
        Rock::Minus,
        Rock::Plus,
        Rock::InvertedL,
        Rock::Line,
        Rock::Square,
    ];

    let mut direction_index = 0;

    for i in 0..2022 {
        let rock = &rock_types[i % 5];

        let max_y = room.solids.iter().map(|p| p.y + 1).max().unwrap_or(0);

        let mut position = Point::new(2, max_y + 3);

        // println!("\nRock begins falling");
        // room.print(rock, &position);

        loop {
            let direction = directions[direction_index];
            direction_index = (direction_index + 1) % directions.len();

            let new_position = match direction {
                Direction::Left => Point::new(position.x - 1, position.y),
                Direction::Right => Point::new(position.x + 1, position.y),
            };

            if !room.collides_with_wall(rock, &new_position) {
                position = new_position;
            }

            // println!("\nJet pushed to the {:?}", direction);
            // room.print(rock, &position);

            let new_position = Point::new(position.x, position.y - 1);

            if room.collides_with_bottom(rock, &new_position) {
                room.solids.append(&mut rock.world_space_points(&position));
                break;
            } else {
                position = new_position;
            }

            // println!("\nRock falls 1 unit:");
            // room.print(rock, &position);
        }
        // println!("\n-----------\n{}", room);
    }

    room.height()
}

fn process_part2(text: String) -> usize {
    let directions = parse_input(&text);

    let mut room = Room {
        width: 7,
        solids: vec![],
    };

    let rock_types = vec![
        Rock::Minus,
        Rock::Plus,
        Rock::InvertedL,
        Rock::Line,
        Rock::Square,
    ];

    let mut direction_index = 0;

    let mut deltas = [0; 5000];
    let mut previous = 0;

    for i in 0..5000 {
        let rock = &rock_types[i % 5];

        let max_y = room.solids.iter().map(|p| p.y + 1).max().unwrap_or(0);

        let mut position = Point::new(2, max_y + 3);

        // println!("\nRock begins falling");
        // room.print(rock, &position);

        loop {
            let direction = directions[direction_index];
            direction_index = (direction_index + 1) % directions.len();

            let new_position = match direction {
                Direction::Left => Point::new(position.x - 1, position.y),
                Direction::Right => Point::new(position.x + 1, position.y),
            };

            if !room.collides_with_wall(rock, &new_position) {
                position = new_position;
            }

            // println!("\nJet pushed to the {:?}", direction);
            // room.print(rock, &position);

            let new_position = Point::new(position.x, position.y - 1);

            if room.collides_with_bottom(rock, &new_position) {
                room.solids.append(&mut rock.world_space_points(&position));
                break;
            } else {
                position = new_position;
            }

            // println!("\nRock falls 1 unit:");
            // room.print(rock, &position);
        }
        // println!("\n-----------\n{}", room);

        deltas[i] = (room.height() - previous) as usize;
        previous = room.height();
    }

    println!("Calculated deltas, now looking for a pattern");
    let (offset, size) = (0..1000)
        .find_map(|offset| {
            let delta_iter = deltas.iter().skip(offset);
            let size = (2..=2500).find(|size| {
                let window = deltas[offset..offset + size].iter().cycle();
                delta_iter.clone().zip(window).all(|(a, b)| a == b)
            });
            size.map(|size| (offset, size))
        })
        .expect("No pattern found");

    let mut delta_iter = deltas.iter();
    let mut count = 1_000_000_000_000;
    let offset_delta = delta_iter.by_ref().take(offset).sum::<usize>();
    count -= offset;
    let cycle_deltas: Vec<usize> = delta_iter.take(size).copied().collect();
    let cycle_delta = cycle_deltas.iter().sum::<usize>();
    let cycle_count = count / size;
    count %= size;
    let remaining_height = cycle_deltas.into_iter().take(count).sum::<usize>();
    let height: usize = offset_delta + cycle_count * cycle_delta + remaining_height;

    height
}
