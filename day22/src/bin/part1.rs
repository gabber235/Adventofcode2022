use crate::Tile::{Air, Path, Rock};
use std::fs;
use Direction::{East, North, South, West};

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Move(i32),
    RotateLeft,
    RotateRight,
}

/// Parse operations from string
/// Example: "10L2" -> (Move(10), RotateLeft(), Move(2))
fn parse_operations(text: &str) -> Vec<Operation> {
    let mut operations = Vec::new();
    let chars = text.chars();

    let mut current_number = String::new();

    for c in chars {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            let number = current_number.parse::<i32>().unwrap();
            current_number.clear();

            operations.push(Operation::Move(number));

            match c {
                'L' => operations.push(Operation::RotateLeft),
                'R' => operations.push(Operation::RotateRight),
                _ => panic!("Unknown operation: {}", c),
            }
        }
    }

    if !current_number.is_empty() {
        let number = current_number.parse::<i32>().unwrap();
        operations.push(Operation::Move(number));
    }

    operations
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tile {
    Air,
    Path,
    Rock,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    /// Get the tile next to the given position in right direction
    /// If there is no tile, we loop around the map and return the first path tile
    /// If there is a rock tile, we return the given position
    fn get_right_tile(&self, position: Position) -> Position {
        let mut x = position.x + 1;
        let y = position.y;

        let row = &self.tiles[y as usize];
        if x >= row.len() as i32 {
            x = 0;
        }

        while let Air = row[x as usize] {
            x += 1;
            if x >= row.len() as i32 {
                x = 0;
            }
        }

        if let Rock = row[x as usize] {
            return position;
        }

        Position { x, y }
    }

    /// Get the tile next to the given position in left direction
    /// If there is no tile, we loop around the map and return the first path tile
    /// If there is a rock tile, we return the given position
    fn get_left_tile(&self, position: Position) -> Position {
        let mut x = position.x - 1;
        let y = position.y;

        let row = &self.tiles[y as usize];
        if x < 0 {
            x = row.len() as i32 - 1;
        }

        while let Air = row[x as usize] {
            x -= 1;
            if x < 0 {
                x = row.len() as i32 - 1;
            }
        }

        if let Rock = row[x as usize] {
            return position;
        }

        Position { x, y }
    }

    /// Get the tile next to the given position in up direction
    /// If there is no tile, we loop around the map and return the first path tile
    /// If there is a rock tile, we return the given position
    fn get_up_tile(&self, position: Position) -> Position {
        let mut y = position.y - 1;
        let x = position.x;

        if y < 0 {
            y = self.tiles.len() as i32 - 1;
        }

        while x >= self.tiles[y as usize].len() as i32
            || matches!(self.tiles[y as usize][x as usize], Tile::Air)
        {
            y -= 1;
            if y < 0 {
                y = self.tiles.len() as i32 - 1;
            }
        }

        if let Rock = self.tiles[y as usize][x as usize] {
            return position;
        }

        Position { x, y }
    }

    /// Get the tile next to the given position in down direction
    /// If there is no tile, we loop around the map and return the first path tile
    /// If there is a rock tile, we return the given position
    fn get_down_tile(&self, position: Position) -> Position {
        let mut y = position.y + 1;
        let x = position.x;

        if y >= self.tiles.len() as i32 {
            y = 0;
        }

        while x >= self.tiles[y as usize].len() as i32
            || matches!(self.tiles[y as usize][x as usize], Tile::Air)
        {
            y += 1;
            if y >= self.tiles.len() as i32 {
                y = 0;
            }
        }

        if let Rock = self.tiles[y as usize][x as usize] {
            return position;
        }

        Position { x, y }
    }

    /// Get the tile next to the given position in the given direction
    /// If there is no tile, we loop around the map and return the first path tile
    /// If there is a rock tile, we return the given position
    fn get_tile(&self, position: Position, direction: Direction) -> Position {
        match direction {
            North => self.get_up_tile(position),
            South => self.get_down_tile(position),
            East => self.get_right_tile(position),
            West => self.get_left_tile(position),
        }
    }
}

#[test]
fn test_get_right_tile() {
    let map = Map {
        tiles: vec![vec![Air, Air, Path, Path, Rock, Path]],
    };

    assert_eq!(
        map.get_right_tile(Position { x: 2, y: 0 }),
        Position { x: 3, y: 0 }
    );

    assert_eq!(
        map.get_right_tile(Position { x: 3, y: 0 }),
        Position { x: 3, y: 0 }
    );

    assert_eq!(
        map.get_right_tile(Position { x: 5, y: 0 }),
        Position { x: 2, y: 0 }
    );
}

#[test]
fn test_get_left_tile() {
    let map = Map {
        tiles: vec![vec![Air, Air, Path, Path, Rock, Path]],
    };

    assert_eq!(
        map.get_left_tile(Position { x: 2, y: 0 }),
        Position { x: 5, y: 0 }
    );

    assert_eq!(
        map.get_left_tile(Position { x: 3, y: 0 }),
        Position { x: 2, y: 0 }
    );

    assert_eq!(
        map.get_left_tile(Position { x: 5, y: 0 }),
        Position { x: 5, y: 0 }
    );
}

#[test]
fn test_get_up_tile() {
    let map = Map {
        tiles: vec![
            vec![Air],
            vec![Air],
            vec![Air, Air],
            vec![Air, Path],
            vec![Air, Path],
            vec![Air, Rock],
            vec![Air, Path],
        ],
    };

    assert_eq!(
        map.get_up_tile(Position { x: 1, y: 4 }),
        Position { x: 1, y: 3 }
    );

    assert_eq!(
        map.get_up_tile(Position { x: 1, y: 3 }),
        Position { x: 1, y: 6 }
    );

    assert_eq!(
        map.get_up_tile(Position { x: 1, y: 6 }),
        Position { x: 1, y: 6 }
    );
}

#[test]
fn test_get_down_tile() {
    let map = Map {
        tiles: vec![
            vec![Air],
            vec![Air],
            vec![Air, Air],
            vec![Air, Path],
            vec![Air, Path],
            vec![Air, Rock],
            vec![Air, Path],
        ],
    };

    assert_eq!(
        map.get_down_tile(Position { x: 1, y: 3 }),
        Position { x: 1, y: 4 }
    );

    assert_eq!(
        map.get_down_tile(Position { x: 1, y: 6 }),
        Position { x: 1, y: 3 }
    );

    assert_eq!(
        map.get_down_tile(Position { x: 1, y: 4 }),
        Position { x: 1, y: 4 }
    );
}

fn parse_map(text: &str) -> Map {
    let mut tiles = Vec::new();

    for line in text.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            match c {
                '.' => row.push(Path),
                '#' => row.push(Rock),
                ' ' => row.push(Air),
                _ => panic!("Unknown tile: {}", c),
            }
        }

        tiles.push(row);
    }

    Map { tiles }
}

fn parse_input(text: String) -> (Map, Vec<Operation>) {
    let spit: Vec<&str> = text.split("\n\n").collect();

    let map = parse_map(spit[0]);
    let operations = parse_operations(spit[1]);

    (map, operations)
}

fn process_part1(text: String) -> i32 {
    let (map, operations) = parse_input(text);

    let mut position = Position {
        // The starting position is always at the top of the map with the first path tile
        x: map.tiles[0].iter().position(|x| matches!(x, Path)).unwrap() as i32,
        y: 0,
    };

    let mut direction = East;

    for operation in operations {
        match operation {
            Operation::RotateLeft => direction = direction.rotate_left(),
            Operation::RotateRight => direction = direction.rotate_right(),
            Operation::Move(steps) => {
                for _ in 0..steps {
                    position = map.get_tile(position, direction);
                }
            }
        }
    }

    let column = position.x + 1;
    let row = position.y + 1;

    let facing = match direction {
        East => 0,
        South => 1,
        West => 2,
        North => 3,
    };

    println!("Part 1: 1000 * {} + 4 * {} + {}", row, column, facing);

    1000 * row + 4 * column + facing
}
