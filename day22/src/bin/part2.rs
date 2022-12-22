use crate::Tile::{Air, Path, Rock};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use num_integer::gcd;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let (mut map, operations) = parse_input(text);

    map.build_cube();

    let start_column = map
        .tiles
        .iter()
        .filter(|(position, tile)| tile == &&Path && position.row == 1)
        .map(|(position, _)| position.column)
        .min()
        .unwrap();

    let mut position = Position {
        // The starting position is always at the top of the map with the first path tile
        column: start_column,
        row: 1,
    };

    let mut direction = Right;

    for operation in operations {
        match operation {
            Operation::RotateLeft => direction = direction.turn_left(),
            Operation::RotateRight => direction = direction.turn_right(),
            Operation::Move(steps) => {
                for _ in 0..steps {
                    let mut next = direction.movement(position);
                    let mut next_facing = direction;

                    if map.at(&next) == Air {
                        (next, next_facing) = map.warp(position, direction);
                    }

                    match map.at(&next) {
                        Rock => break,
                        Path => {
                            position = next;
                            direction = next_facing;
                        }
                        Air => panic!("Unexpected air tile"),
                    }
                }
            }
        }
    }

    let column = position.column;
    let row = position.row;

    let facing = match direction {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };

    println!(
        "Part 1: 1000 * {} + 4 * {} + {} = {}",
        row,
        column,
        facing,
        1000 * row + 4 * column + facing
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl Direction {
    fn movement(&self, position: Position) -> Position {
        match self {
            Up => Position {
                row: position.row - 1,
                column: position.column,
            },
            Down => Position {
                row: position.row + 1,
                column: position.column,
            },
            Left => Position {
                row: position.row,
                column: position.column - 1,
            },
            Right => Position {
                row: position.row,
                column: position.column + 1,
            },
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
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

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Air,
    Path,
    Rock,
}

type Grid = HashMap<Position, Tile>;

#[derive(Debug)]
struct Map {
    tiles: Grid,
    width: usize,
    height: usize,
    cube_stitching: HashMap<(Position, Direction), (Position, Direction)>,
}

impl Map {
    pub fn at(&self, pos: &Position) -> Tile {
        *self.tiles.get(pos).unwrap_or(&Air)
    }

    pub fn warp(&self, pos: Position, direction: Direction) -> (Position, Direction) {
        assert_eq!(self.at(&pos), Path);
        assert_eq!(self.at(&direction.movement(pos)), Air);
        *self.cube_stitching.get(&(pos, direction)).unwrap()
    }

    // Great thanks to https://gist.github.com/mgedmin/71d632e40d4de5c9486a4616ffb53208
    fn build_cube(&mut self) {
        // I think there might be foldings where this will give the wrong size (double the real
        // one), but it works on both my example and my sample input.
        let cube_size = gcd(self.width, self.height);
        let mut cube = 0;
        let mut faces: Vec<Face> = Vec::new();
        let mut face_at: HashMap<Position, usize> = HashMap::new();
        for row in 0..self.height / cube_size {
            for col in 0..self.width / cube_size {
                let pos = Position {
                    row: 1 + row * cube_size,
                    column: 1 + col * cube_size,
                };
                if self.at(&pos) != Air {
                    face_at.insert(pos, cube);
                    faces.push(Face {
                        position: pos,
                        size: cube_size,
                        which: None,
                        flat_edges: HashMap::new(),
                    });
                    cube += 1;
                }
            }
        }
        assert_eq!(cube, 6);
        faces[0].which = Some(CubeFace::Front);
        faces[0].flat_edges.insert(Right, CubeFace::Right);
        faces[0].flat_edges.insert(Left, CubeFace::Left);
        faces[0].flat_edges.insert(Up, CubeFace::Top);
        faces[0].flat_edges.insert(Down, CubeFace::Bottom);

        let next_left: HashMap<(CubeFace, CubeFace), CubeFace> = HashMap::from([
            // I'm sure this can be derived from the vertices somehow
            ((CubeFace::Front, CubeFace::Top), CubeFace::Left),
            ((CubeFace::Front, CubeFace::Left), CubeFace::Bottom),
            ((CubeFace::Front, CubeFace::Bottom), CubeFace::Right),
            ((CubeFace::Front, CubeFace::Right), CubeFace::Top),
            ((CubeFace::Left, CubeFace::Top), CubeFace::Back),
            ((CubeFace::Left, CubeFace::Back), CubeFace::Bottom),
            ((CubeFace::Left, CubeFace::Bottom), CubeFace::Front),
            ((CubeFace::Left, CubeFace::Front), CubeFace::Top),
            ((CubeFace::Right, CubeFace::Top), CubeFace::Front),
            ((CubeFace::Right, CubeFace::Front), CubeFace::Bottom),
            ((CubeFace::Right, CubeFace::Bottom), CubeFace::Back),
            ((CubeFace::Right, CubeFace::Back), CubeFace::Top),
            ((CubeFace::Back, CubeFace::Top), CubeFace::Right),
            ((CubeFace::Back, CubeFace::Right), CubeFace::Bottom),
            ((CubeFace::Back, CubeFace::Bottom), CubeFace::Left),
            ((CubeFace::Back, CubeFace::Left), CubeFace::Top),
            ((CubeFace::Top, CubeFace::Front), CubeFace::Right),
            ((CubeFace::Top, CubeFace::Right), CubeFace::Back),
            ((CubeFace::Top, CubeFace::Back), CubeFace::Left),
            ((CubeFace::Top, CubeFace::Left), CubeFace::Front),
            ((CubeFace::Bottom, CubeFace::Front), CubeFace::Left),
            ((CubeFace::Bottom, CubeFace::Left), CubeFace::Back),
            ((CubeFace::Bottom, CubeFace::Back), CubeFace::Right),
            ((CubeFace::Bottom, CubeFace::Right), CubeFace::Front),
        ]);

        let mut queue = VecDeque::from([0]);
        while let Some(u) = queue.pop_front() {
            let which_u = faces[u].which.unwrap();
            for direction in [Right, Down, Left, Up] {
                if let Some(v) = face_at.get(&faces[u].flat_sibling(direction)) {
                    if faces[*v].which.is_some() {
                        continue;
                    }
                    if !faces[u].flat_edges.contains_key(&direction) {
                        continue;
                    }
                    let which_v = faces[u].flat_edges[&direction];
                    faces[*v].which = Some(which_v);

                    let mut insert = |direction, face| {
                        faces[*v].flat_edges.insert(direction, face);
                    };
                    insert(direction, which_u.opposite());
                    insert(direction.turn_left(), next_left[&(which_u, which_v)]);
                    insert(
                        direction.turn_right(),
                        next_left[&(which_u, which_v)].opposite(),
                    );
                    insert(direction.opposite(), which_u);
                    queue.push_back(*v);
                }
            }
        }
        // A cube has 6 sides, 8 vertices and 12 edges
        // In a folding, 5 edges stay connected, the other 7 get split into 14 unconnected square
        // edges.

        let mut stitchings: HashMap<CubeEdge, (Direction, CubeVertex, Vec<Position>)> =
            HashMap::new();
        for face in faces {
            for direction in [Right, Down, Left, Up] {
                if !face_at.contains_key(&face.flat_sibling(direction)) {
                    let (vertex, tiles) = face.edge_tiles(direction);
                    let pos = tiles[0];
                    let other_face = face.flat_edges[&direction];
                    if face.common_edge(other_face).is_none() {
                        println!(
                            "No common edge between {:?} and {:?}!",
                            face.which.unwrap(),
                            other_face
                        );
                        panic!();
                    }
                    let edge = face.common_edge(other_face).unwrap();
                    if let Some((other_direction, other_vertex, other_tiles)) =
                        stitchings.get(&edge)
                    {
                        let pairs: Vec<(&Position, &Position)> = if vertex == *other_vertex {
                            tiles.iter().zip(other_tiles).collect()
                        } else {
                            tiles.iter().rev().zip(other_tiles).collect()
                        };
                        for (pos, other) in pairs {
                            self.cube_stitching
                                .insert((*pos, direction), (*other, other_direction.opposite()));
                            self.cube_stitching
                                .insert((*other, *other_direction), (*pos, direction.opposite()));
                        }
                    } else {
                        stitchings.insert(edge, (direction, vertex, tiles));
                    }
                }
            }
        }
    }
}

// Let's number the vertices of the front face:
//
//    0 --- 1
//    |     |
//    |     |
//    3 --- 2
//
// back face:
//
//    4 --- 5
//    |     |
//    |     |
//    7 --- 6
//
// A set of vertices determines the face.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, IntoPrimitive)]
enum CubeFace {
    Front = 0b0000_1111,
    Top = 0b0011_0011,
    Bottom = 0b1100_1100,
    Left = 0b1001_1001,
    Right = 0b0110_0110,
    Back = 0b1111_0000,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
enum CubeEdge {
    FrontTop = 0b0000_0011,
    FrontRight = 0b0000_0110,
    FrontBottom = 0b0000_1100,
    FrontLeft = 0b0000_1001,
    BackTop = 0b0011_0000,
    BackRight = 0b0110_0000,
    BackBottom = 0b1100_0000,
    BackLeft = 0b1001_0000,
    LeftTop = 0b0001_0001,
    LeftBottom = 0b1000_1000,
    RightTop = 0b0010_0010,
    RightBottom = 0b0100_0100,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
enum CubeVertex {
    FrontTopLeft = 0b0000_0001,
    FrontTopRight = 0b0000_0010,
    FrontBottomRight = 0b0000_0100,
    FrontBottomLeft = 0b0000_1000,
    BackTopLeft = 0b0001_0000,
    BackTopRight = 0b0010_0000,
    BackBottomRight = 0b0100_0000,
    BackBottomLeft = 0b1000_0000,
}

impl CubeFace {
    fn opposite(self) -> CubeFace {
        use CubeFace::*;
        match self {
            Front => Back,
            Back => Front,
            Left => Right,
            Right => Left,
            Top => Bottom,
            Bottom => Top,
        }
    }

    #[allow(dead_code)]
    fn touching(self) -> [CubeFace; 4] {
        use CubeFace::*;
        match self {
            Front => [Top, Left, Bottom, Right],
            Left => [Top, Back, Bottom, Front],
            Right => [Top, Front, Bottom, Back],
            Back => [Top, Right, Bottom, Left],
            Top => [Front, Right, Back, Left],
            Bottom => [Front, Left, Back, Right],
        }
    }

    fn common_edge(self, other: CubeFace) -> Option<CubeEdge> {
        let a: u8 = self.into();
        let b: u8 = other.into();
        (a & b).try_into().ok()
    }

    fn common_vertex(self, other1: CubeFace, other2: CubeFace) -> Option<CubeVertex> {
        let a: u8 = self.into();
        let b: u8 = other1.into();
        let c: u8 = other2.into();
        (a & b & c).try_into().ok()
    }
}

struct Face {
    position: Position,
    size: usize,
    which: Option<CubeFace>,
    flat_edges: HashMap<Direction, CubeFace>,
}

impl Face {
    fn common_edge(&self, other: CubeFace) -> Option<CubeEdge> {
        self.which?.common_edge(other)
    }

    fn common_vertex(&self, other1: CubeFace, other2: CubeFace) -> Option<CubeVertex> {
        self.which?.common_vertex(other1, other2)
    }

    fn edge_tiles(&self, direction: Direction) -> (CubeVertex, Vec<Position>) {
        let n = self.size;
        let vertex = self
            .common_vertex(
                self.flat_edges[&direction],
                self.flat_edges[&direction.turn_left()],
            )
            .unwrap();
        let tiles = match direction {
            Right => (0..n)
                .map(|n| Position {
                    row: self.position.row + n,
                    column: self.position.column + self.size - 1,
                })
                .collect(),
            Down => (0..n)
                .rev()
                .map(|n| Position {
                    row: self.position.row + self.size - 1,
                    column: self.position.column + n,
                })
                .collect(),
            Left => (0..n)
                .rev()
                .map(|n| Position {
                    row: self.position.row + n,
                    column: self.position.column,
                })
                .collect(),
            Up => (0..n)
                .map(|n| Position {
                    row: self.position.row,
                    column: self.position.column + n,
                })
                .collect(),
        };
        (vertex, tiles)
    }

    fn flat_sibling(&self, direction: Direction) -> Position {
        match direction {
            Right => Position {
                row: self.position.row,
                column: self.position.column + self.size,
            },
            Down => Position {
                row: self.position.row + self.size,
                column: self.position.column,
            },
            Left => Position {
                row: self.position.row,
                column: self.position.column.saturating_sub(self.size),
            },
            Up => Position {
                row: self.position.row.saturating_sub(self.size),
                column: self.position.column,
            },
        }
    }
}

fn parse_grid(input: &[&str]) -> Grid {
    let mut grid: Grid = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.chars().enumerate() {
            let pos = Position {
                row: i + 1,
                column: j + 1,
            };
            match cell {
                '#' => {
                    grid.insert(pos, Rock);
                }
                '.' => {
                    grid.insert(pos, Path);
                }
                _ => (),
            }
        }
    }

    grid
}

fn parse_map(text: &str) -> Map {
    let grid = parse_grid(text.lines().collect::<Vec<_>>().as_slice());

    let width = text.lines().map(|l| l.len()).max().unwrap();
    let height = text.lines().count();

    Map {
        tiles: grid,
        width,
        height,
        cube_stitching: HashMap::new(),
    }
}

fn parse_input(text: String) -> (Map, Vec<Operation>) {
    let spit: Vec<&str> = text.split("\n\n").collect();

    let map = parse_map(spit[0]);
    let operations = parse_operations(spit[1]);

    (map, operations)
}
