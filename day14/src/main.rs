use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan. After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point. For example:
//
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
// This scan means that there are two paths of rock; the first path consists of two straight lines, and the second path consists of three straight lines. (Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)
//
// The sand is pouring into the cave from point 500,0.
//
// Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.
//
// A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right. Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.
// Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn down(&self) -> Self {
        Point::new(self.x, self.y + 1)
    }

    fn left(&self) -> Self {
        Point::new(self.x - 1, self.y)
    }

    fn right(&self) -> Self {
        Point::new(self.x + 1, self.y)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Ok(Point { x, y })
    }
}

/// A path is a list of points that form a line.
/// Every next point is either vertically or horizontally on the same line as the previous point.
#[derive(Debug, PartialEq, Eq)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    /// Checks if a point intersects with the path
    fn has_rock_at(&self, point: &Point) -> bool {
        self.points.windows(2).any(|window| {
            let p1 = window[0];
            let p2 = window[1];
            if p1.x == p2.x {
                // Vertical line
                point.x == p1.x && point.y >= p1.y && point.y <= p2.y
            } else {
                // Horizontal line
                point.y == p1.y && point.x >= p1.x && point.x <= p2.x
            }
        })
    }
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|p| p.parse().unwrap())
            .collect::<Vec<_>>();
        Ok(Path { points })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Sand,
    Rock,
    Air,
}

impl Tile {
    fn is_solid(&self) -> bool {
        match self {
            Tile::Sand | Tile::Rock => true,
            Tile::Air => false,
        }
    }
}

struct Cave {
    tiles: HashMap<Point, Tile>,
    // lowest_y: i32,
    highest_y: i32,
    // lowest_x: i32,
    // highest_x: i32,
    floor: Option<i32>,
}

impl From<Vec<Path>> for Cave {
    fn from(paths: Vec<Path>) -> Self {
        let highest_y = paths
            .iter()
            .map(|p| p.points.iter().map(|p| p.y).max().unwrap())
            .max()
            .unwrap();

        let highest_y = highest_y.max(0);

        let mut tiles = HashMap::new();

        for path in paths {
            for line_segment in path.points.windows(2) {
                let p1 = line_segment[0];
                let p2 = line_segment[1];
                if p1.x == p2.x {
                    // Vertical line
                    let y_start = p1.y.min(p2.y);
                    let y_end = p1.y.max(p2.y);
                    for y in y_start..=y_end {
                        tiles.insert(Point::new(p1.x, y), Tile::Rock);
                    }
                } else {
                    // Horizontal line
                    let x_start = p1.x.min(p2.x);
                    let x_end = p1.x.max(p2.x);
                    for x in x_start..=x_end {
                        tiles.insert(Point::new(x, p1.y), Tile::Rock);
                    }
                }
            }
        }

        Cave {
            tiles,
            highest_y,
            floor: None,
        }
    }
}

impl Cave {
    fn lowest_x(&self) -> i32 {
        self.tiles.keys().map(|p| p.x).min().unwrap()
    }

    fn highest_x(&self) -> i32 {
        self.tiles.keys().map(|p| p.x).max().unwrap()
    }

    fn lowest_y(&self) -> i32 {
        min(0, self.tiles.keys().map(|p| p.y).min().unwrap())
    }

    fn highest_y(&self) -> i32 {
        if let Some(floor) = self.floor {
            floor
        } else {
            self.tiles.keys().map(|p| p.y).max().unwrap()
        }
    }

    // fn print(&self) {
    //     for row in &self.tiles {
    //         for tile in row {
    //             match tile {
    //                 Tile::Sand => print!("o"),
    //                 Tile::Rock => print!("#"),
    //                 Tile::Air => print!("."),
    //             }
    //         }
    //         println!();
    //     }
    // }

    fn print(&self) {
        let lowest_x = self.lowest_x();
        let highest_x = self.highest_x();
        let lowest_y = self.lowest_y();
        let highest_y = self.highest_y();

        for y in lowest_y..=highest_y {
            for x in lowest_x..=highest_x {
                let tile = self.local_tile(&Point::new(x, y));
                match tile {
                    Tile::Sand => print!("o"),
                    Tile::Rock => print!("#"),
                    Tile::Air => print!("."),
                }
            }
            println!();
        }
    }

    fn add_floor(&mut self) {
        self.floor = Some(self.highest_y + 2);
        self.highest_y += 1;
    }

    fn local_tile(&self, point: &Point) -> Tile {
        if let Some(floor) = self.floor {
            if point.y == floor {
                return Tile::Rock;
            }
        }

        self.tiles.get(point).cloned().unwrap_or(Tile::Air)
    }

    fn set_local_tile(&mut self, point: &Point, tile: Tile) {
        self.tiles.insert(*point, tile);
    }

    fn add_new_sand(&mut self) -> bool {
        let mut insert_point = Point::new(500, 0);

        // Find the first place where the sand cant go down or diagonal down.
        // If we succeed, we can add sand at that point and return true.
        // If the sand falls outside of our cave then we return false.
        loop {
            // Exit condition
            if (insert_point.y > self.highest_y
                || insert_point.x < self.lowest_x()
                || insert_point.x > self.highest_x())
                && self.floor.is_none()
            {
                // We fell outside of our cave
                return false;
            }

            let tile_below = self.local_tile(&insert_point.down());
            if !tile_below.is_solid() {
                insert_point = insert_point.down();
                continue;
            }

            let tile_diagonal_left = self.local_tile(&insert_point.down().left());
            if !tile_diagonal_left.is_solid() {
                insert_point = insert_point.down().left();
                continue;
            }

            let tile_diagonal_right = self.local_tile(&insert_point.down().right());
            if !tile_diagonal_right.is_solid() {
                insert_point = insert_point.down().right();
                continue;
            }

            if insert_point == Point::new(500, 0) && self.local_tile(&insert_point) == Tile::Sand {
                return false;
            }

            // We found a place where the sand cant go down or diagonal down.
            // Add sand at that point and return true.
            self.set_local_tile(&insert_point, Tile::Sand);

            return true;
        }
    }
}

fn process_part1(text: String) -> i32 {
    let paths = text
        .lines()
        .map(|l| l.parse::<Path>().unwrap())
        .collect::<Vec<_>>();

    let mut cave = Cave::from(paths);

    let mut sand_added = 0;

    while cave.add_new_sand() {
        sand_added += 1;
    }

    // cave.print();
    sand_added
}

// You realize you misread the scan. There isn't an endless void at the bottom of the scan - there's floor, and you're standing on it!
// You don't have time to scan the floor, so assume the floor is an infinite horizontal line with a y coordinate equal to two plus the highest y coordinate of any point in your scan.
// In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11. (This is as if your scan contained one extra rock path like -infinity,11 -> infinity,11.) With the added floor, the example above now looks like this:
//To find somewhere safe to stand, you'll need to simulate falling sand until a unit of sand comes to rest at 500,0, blocking the source entirely and stopping the flow of sand into the cave. In the example above, the situation finally looks like this after 93 units of sand come to rest:

fn process_part2(text: String) -> i32 {
    let paths = text
        .lines()
        .map(|l| l.parse::<Path>().unwrap())
        .collect::<Vec<_>>();

    let mut cave = Cave::from(paths);
    cave.add_floor();

    let mut sand_added = 0;

    while cave.add_new_sand() {
        sand_added += 1;
        if sand_added % 5000 == 0 {
            println!("Sand added: {}", sand_added);
            // cave.print();
        }
    }

    // cave.print();
    sand_added
}
