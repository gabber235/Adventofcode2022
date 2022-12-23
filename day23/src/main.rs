use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));

    let time = std::time::Instant::now();
    println!("Part2 Slow: {}", process_part2_slow(text.clone()));
    println!("Part2 Slow took: {}ms", time.elapsed().as_millis());

    let time = std::time::Instant::now();
    println!("Part2: {}", process_part2(text));
    println!("Part2 took: {}ms", time.elapsed().as_millis());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Returns the 8 neighbours of a point
    fn adjacent(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                neighbours.push(Point {
                    x: self.x + x,
                    y: self.y + y,
                });
            }
        }
        neighbours
    }
}

#[derive(Debug)]
struct Map {
    elves: Vec<Point>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .try_fold(Map { elves: Vec::new() }, |mut map, (y, line)| {
                line.chars().enumerate().try_for_each(|(x, c)| {
                    if c == '#' {
                        map.elves.push(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    Ok(())
                })?;
                Ok(map)
            })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let min_x = self.elves.iter().map(|p| p.x).min().unwrap() - 1;
        let max_x = self.elves.iter().map(|p| p.x).max().unwrap() + 1;
        let min_y = self.elves.iter().map(|p| p.y).min().unwrap() - 1;
        let max_y = self.elves.iter().map(|p| p.y).max().unwrap() + 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elves.contains(&Point { x, y }) {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Finds the points of interest in the given direction.
    /// These are the three points that are adjacent to the given point in the given direction.
    /// So for North the points are the point above and its two adjacent points.
    fn points_of_interest(&self, point: &Point) -> Vec<Point> {
        match self {
            Direction::North => {
                let above = Point {
                    x: point.x,
                    y: point.y - 1,
                };
                let left = Point {
                    x: point.x - 1,
                    y: point.y - 1,
                };
                let right = Point {
                    x: point.x + 1,
                    y: point.y - 1,
                };
                vec![above, left, right]
            }
            Direction::South => {
                let below = Point {
                    x: point.x,
                    y: point.y + 1,
                };
                let left = Point {
                    x: point.x - 1,
                    y: point.y + 1,
                };
                let right = Point {
                    x: point.x + 1,
                    y: point.y + 1,
                };
                vec![below, left, right]
            }
            Direction::East => {
                let right = Point {
                    x: point.x + 1,
                    y: point.y,
                };
                let above = Point {
                    x: point.x + 1,
                    y: point.y - 1,
                };
                let below = Point {
                    x: point.x + 1,
                    y: point.y + 1,
                };
                vec![right, above, below]
            }
            Direction::West => {
                let left = Point {
                    x: point.x - 1,
                    y: point.y,
                };
                let above = Point {
                    x: point.x - 1,
                    y: point.y - 1,
                };
                let below = Point {
                    x: point.x - 1,
                    y: point.y + 1,
                };
                vec![left, above, below]
            }
        }
    }
}

fn find_next_spot(map: &Map, point: &Point, directions: &VecDeque<Direction>) -> Point {
    // If there is nobody in the 8 adjacent spots, we stay at the same spot.
    let neighbours = point.adjacent();
    if neighbours.iter().all(|p| !map.elves.contains(p)) {
        return *point;
    }

    // Find the first direction that has an empty spot.
    for direction in directions {
        let points_of_interest = direction.points_of_interest(point);
        if points_of_interest.iter().all(|p| !map.elves.contains(p)) {
            return points_of_interest[0];
        }
    }

    // If there is no empty spot, we stay at the same spot.
    *point
}

fn process_part1(text: String) -> i32 {
    let mut map = Map::from_str(&text).unwrap();
    let mut directions = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    for _ in 0..10 {
        let mut moving_to = map
            .elves
            .iter()
            .map(|p| (p, find_next_spot(&map, p, &directions)))
            .collect::<Vec<_>>();

        // if all of them are moving to the same spot, we are done
        if moving_to.iter().all(|(old, new)| *old == new) {
            break;
        }

        // If two elves want to move to the same spot, none of them move.
        // So we remove all those pairs.
        let mut to_stay_same = Vec::new();
        for (p1, p2) in moving_to.iter() {
            if moving_to.iter().filter(|(_, p)| *p == *p2).count() > 1 {
                to_stay_same.push(*p1);
            }
        }

        moving_to.retain(|(p, _)| !to_stay_same.contains(p));
        // Add back the elves that are staying at the same spot.
        moving_to.extend(to_stay_same.iter().map(|p| (*p, **p)));

        // Update the map
        map.elves = moving_to.iter().map(|(_, p)| *p).collect();

        // Rotate the directions so that the first direction becomes the last one.
        let first = directions.pop_front().unwrap();
        directions.push_back(first);
    }

    println!("{}", map);

    let min_x = map.elves.iter().map(|p| p.x).min().unwrap();
    let max_x = map.elves.iter().map(|p| p.x).max().unwrap();
    let min_y = map.elves.iter().map(|p| p.y).min().unwrap();
    let max_y = map.elves.iter().map(|p| p.y).max().unwrap();

    (min_x..=max_x)
        .cartesian_product(min_y..=max_y)
        .filter(|(x, y)| !map.elves.contains(&Point { x: *x, y: *y }))
        .count() as i32
}

fn process_part2_slow(text: String) -> i32 {
    let mut map = Map::from_str(&text).unwrap();
    let mut directions = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut rounds = 0;
    loop {
        rounds += 1;
        let mut moving_to = map
            .elves
            .iter()
            .map(|p| (p, find_next_spot(&map, p, &directions)))
            .collect::<Vec<_>>();

        // if all of them are moving to the same spot, we are done
        if moving_to.iter().all(|(old, new)| *old == new) {
            break;
        }

        // If two elves want to move to the same spot, none of them move.
        // So we remove all those pairs.
        let mut to_stay_same = Vec::new();
        for (p1, p2) in moving_to.iter() {
            if moving_to.iter().filter(|(_, p)| *p == *p2).count() > 1 {
                to_stay_same.push(*p1);
            }
        }

        moving_to.retain(|(p, _)| !to_stay_same.contains(p));
        // Add back the elves that are staying at the same spot.
        moving_to.extend(to_stay_same.iter().map(|p| (*p, **p)));

        // Update the map
        map.elves = moving_to.iter().map(|(_, p)| *p).collect();

        // Rotate the directions so that the first direction becomes the last one.
        let first = directions.pop_front().unwrap();
        directions.push_back(first);
    }

    rounds
}

fn process_part2(text: String) -> i32 {
    let mut map = Map::from_str(&text).unwrap();
    let mut directions = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut rounds = 0;
    loop {
        rounds += 1;
        let moving_to = map
            .elves
            .par_iter()
            .map(|p| (p, find_next_spot(&map, p, &directions)))
            .collect::<Vec<_>>();

        // if all of them are moving to the same spot, we are done
        if moving_to.iter().all(|(old, new)| *old == new) {
            break;
        }

        map.elves = moving_to
            .par_iter()
            .map(|(old, new)| {
                if moving_to.iter().filter(|(_, p2)| *p2 == *new).count() > 1 {
                    **old
                } else {
                    *new
                }
            })
            .collect::<Vec<Point>>();

        // Rotate the directions so that the first direction becomes the last one.
        let first = directions.pop_front().unwrap();
        directions.push_back(first);
    }

    rounds
}
