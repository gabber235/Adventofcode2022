use std::collections::HashSet;

use crate::Stage::{FetchSnacks, MoveWithElfs, ReturnWithSnacks};
use rayon::prelude::*;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let time = SystemTime::now();
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part1: {}ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    println!("Part2: {}", process_part2(text));
    println!("Part2: {}ms", time.elapsed().unwrap().as_millis());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

impl Blizzard {
    /// Move the blizzard one step in the direction it is facing
    /// If it hits a wall, it turns wraps around the grid
    /// The walls are the outermost rows and columns
    fn simulate(&self, grid: &Grid) -> Blizzard {
        let new_position = match self.direction {
            Direction::Up => Point {
                x: self.position.x,
                y: if self.position.y <= 1 {
                    (grid.height - 2) as i32
                } else {
                    self.position.y - 1
                },
            },
            Direction::Down => Point {
                x: self.position.x,
                y: if self.position.y >= (grid.height - 2) as i32 {
                    1
                } else {
                    self.position.y + 1
                },
            },
            Direction::Left => Point {
                x: if self.position.x <= 1 {
                    (grid.width - 2) as i32
                } else {
                    self.position.x - 1
                },
                y: self.position.y,
            },
            Direction::Right => Point {
                x: if self.position.x >= (grid.width - 2) as i32 {
                    1
                } else {
                    self.position.x + 1
                },
                y: self.position.y,
            },
        };

        Blizzard {
            position: new_position,
            direction: self.direction,
        }
    }
}

#[derive(Debug)]
struct Grid {
    blizzards: Vec<Blizzard>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let width = lines[0].len();
        let height = lines.len();

        let start = Point {
            x: lines[0].find('.').unwrap() as i32,
            y: 0,
        };
        let end = Point {
            x: lines[height - 1].find('.').unwrap() as i32,
            y: (height - 1) as i32,
        };

        let blizzards = lines[1..height - 1]
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    c.try_into().ok().map(|direction| Blizzard {
                        position: Point {
                            x: x as i32,
                            y: (y + 1) as i32,
                        },
                        direction,
                    })
                })
            })
            .collect();

        Ok(Grid {
            blizzards,
            width,
            height,
            start,
            end,
        })
    }
}

impl Grid {
    fn is_valid_position(&self, position: Point) -> bool {
        (position.x > 0
            && position.x < (self.width - 1) as i32
            && position.y > 0
            && position.y < (self.height - 1) as i32)
            || position == self.start
            || position == self.end
    }

    fn has_blizzard(&self, position: Point) -> bool {
        self.blizzards
            .iter()
            .any(|blizzard| blizzard.position == position)
    }

    fn is_valid_choice(&self, point: &Point) -> bool {
        self.is_valid_position(*point) && !self.has_blizzard(*point)
    }

    fn find_new_choices(&self, choice: &Point) -> Vec<Point> {
        // Choices are the four adjacent points, or the same point.
        // A choice is only valid if it is not a wall and not a blizzard
        vec![
            Point {
                x: choice.x,
                y: choice.y,
            },
            Point {
                x: choice.x + 1,
                y: choice.y,
            },
            Point {
                x: choice.x - 1,
                y: choice.y,
            },
            Point {
                x: choice.x,
                y: choice.y + 1,
            },
            Point {
                x: choice.x,
                y: choice.y - 1,
            },
        ]
        .iter()
        .filter(|point| self.is_valid_choice(point))
        .cloned()
        .collect()
    }
}

fn process_part1(text: String) -> i32 {
    let mut grid = text.parse::<Grid>().unwrap();

    let mut choices = HashSet::new();
    choices.insert(grid.start);

    let mut minute = 0;

    loop {
        minute += 1;
        // print(&grid, &choices);
        grid.blizzards = grid
            .blizzards
            .par_iter()
            .map(|blizzard| blizzard.simulate(&grid))
            .collect();

        choices = choices
            .par_iter()
            .flat_map(|choice| grid.find_new_choices(choice))
            .collect();

        // If the end is in the choices, we have found a path
        if choices.contains(&grid.end) {
            return minute;
        }
    }
}

// --- Part Two ---
// As the expedition reaches the far side of the valley, one of the Elves looks especially dismayed:
// He forgot his snacks at the entrance to the valley!
// Since you're so good at dodging blizzards, the Elves humbly request that you go back for his snacks. From the same initial conditions, how quickly can you make it from the start to the goal, then back to the start, then back to the goal?
// In the above example, the first trip to the goal takes 18 minutes, the trip back to the start takes 23 minutes, and the trip back to the goal again takes 13 minutes, for a total time of 54 minutes.
// What is the fewest number of minutes required to reach the goal, go back to the start, then reach the goal again?

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Stage {
    MoveWithElfs,
    FetchSnacks,
    ReturnWithSnacks,
}

fn process_part2(text: String) -> i32 {
    let mut grid = text.parse::<Grid>().unwrap();

    let mut choices = HashSet::new();
    choices.insert(grid.start);

    let mut stage = Stage::MoveWithElfs;

    let mut minute = 0;

    loop {
        minute += 1;
        // print(&grid, &choices);
        grid.blizzards = grid
            .blizzards
            .par_iter()
            .map(|blizzard| blizzard.simulate(&grid))
            .collect();

        choices = choices
            .par_iter()
            .flat_map(|choice| grid.find_new_choices(choice))
            .collect();

        if stage == MoveWithElfs && choices.contains(&grid.end) {
            stage = FetchSnacks;
            choices.clear();
            choices.insert(grid.end);
        } else if stage == FetchSnacks && choices.contains(&grid.start) {
            stage = ReturnWithSnacks;
            choices.clear();
            choices.insert(grid.start);
        } else if stage == ReturnWithSnacks && choices.contains(&grid.end) {
            return minute;
        }
    }
}

fn print(grid: &Grid, choices: &HashSet<Point>) {
    println!("=====================");
    for y in 0..grid.height {
        for x in 0..grid.width {
            // If the point is a choice, print it
            if choices.contains(&Point {
                x: x as i32,
                y: y as i32,
            }) {
                print!("E");
                continue;
            }

            let blizzards_at_position = grid
                .blizzards
                .iter()
                .filter(|blizzard| {
                    blizzard.position
                        == Point {
                            x: x as i32,
                            y: y as i32,
                        }
                })
                .collect::<Vec<_>>();

            // If multiple blizzards are at the same position, print the number of blizzards
            if blizzards_at_position.len() > 1 {
                print!("{}", blizzards_at_position.len());
                continue;
            }

            // Otherwise print the direction of the blizzard
            if let Some(blizzard) = blizzards_at_position.first() {
                print!("{}", String::from(blizzard.direction));
                continue;
            }

            if x == grid.start.x as usize && y == grid.start.y as usize {
                print!("S");
                continue;
            }
            if x == grid.end.x as usize && y == grid.end.y as usize {
                print!("E");
                continue;
            }

            // Print a # if it is a wall
            if y == 0 || y == grid.height - 1 || x == 0 || x == grid.width - 1 {
                print!("#");
                continue;
            }

            // Print a . if it is a path
            print!(".");
        }
        println!();
    }
}
