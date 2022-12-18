use std::collections::{HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// --- Day 18: Boiling Boulders ---
// You and the elephants finally reach fresh air. You've emerged near the base of a large volcano that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and toward the ocean.
// Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a little longer. Outside the cave, you can see the lava landing in a pond and hear it loudly hissing as it solidifies.
// Depending on the specific compounds in the lava and speed at which it cools, it might be forming obsidian! The cooling rate should be based on the surface area of the lava droplets, so you take a quick scan of a droplet as it flies past you (your puzzle input).
// Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low and, as a result, it approximates the shape of the lava droplet with 1x1x1 cubes on a 3D grid, each given as its x,y,z position.
// To approximate the surface area, count the number of sides of each cube that are not immediately connected to another cube. So, if your scan were only two adjacent cubes like 1,1,1 and 2,1,1, each cube would have a single side covered and five sides exposed, a total surface area of 10 sides.
// Here's a larger example:
//
// 2,2,2
// 1,2,2
// 3,2,2
// 2,1,2
// 2,3,2
// 2,2,1
// 2,2,3
// 2,2,4
// 2,2,6
// 1,2,5
// 3,2,5
// 2,1,5
// 2,3,5
// In the above example, after counting up all the sides that aren't connected to another cube, the total surface area is 64.
//
// What is the surface area of your scanned lava droplet?

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }

    fn neighbours(&self) -> CubeNeighboursIterator {
        CubeNeighboursIterator {
            cube: *self,
            index: 0,
        }
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Ok(Cube { x, y, z })
    }
}

struct CubeNeighboursIterator {
    cube: Cube,
    index: usize,
}

impl Iterator for CubeNeighboursIterator {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        let Cube { x, y, z } = self.cube;
        let result = match self.index {
            0 => Some(Cube::new(x - 1, y, z)),
            1 => Some(Cube::new(x + 1, y, z)),
            2 => Some(Cube::new(x, y - 1, z)),
            3 => Some(Cube::new(x, y + 1, z)),
            4 => Some(Cube::new(x, y, z - 1)),
            5 => Some(Cube::new(x, y, z + 1)),
            _ => None,
        };
        self.index += 1;
        result
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    cubes: Vec<Cube>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<_>>();
        Ok(Grid { cubes })
    }
}

fn process_part1(text: String) -> i32 {
    let grid = text.parse::<Grid>().unwrap();

    let mut surface_area = 0;

    for cube in &grid.cubes {
        for neighbour in cube.neighbours() {
            if !grid.cubes.contains(&neighbour) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

// --- Part Two ---
// Something seems off about your calculation. The cooling rate depends on exterior surface area, but your calculation also included the surface area of air pockets trapped in the lava droplet.
// Instead, consider only cube sides that could be reached by the water and steam as the lava droplet tumbles into the pond. The steam will expand to reach as much as possible, completely displacing any air on the outside of the lava droplet but never expanding diagonally.
// In the larger example above, exactly one cube of air is trapped within the lava droplet (at 2,2,5), so the exterior surface area of the lava droplet is 58.
// What is the exterior surface area of your scanned lava droplet?

fn in_range(cube: &Cube, min: &Cube, max: &Cube) -> bool {
    cube.x >= min.x
        && cube.x <= max.x
        && cube.y >= min.y
        && cube.y <= max.y
        && cube.z >= min.z
        && cube.z <= max.z
}

fn process_part2(text: String) -> i32 {
    let grid = text.parse::<Grid>().unwrap();

    // Lets first pick a point outside the grid
    let min_x = grid.cubes.iter().map(|cube| cube.x).min().unwrap();
    let min_y = grid.cubes.iter().map(|cube| cube.y).min().unwrap();
    let min_z = grid.cubes.iter().map(|cube| cube.z).min().unwrap();

    let max_x = grid.cubes.iter().map(|cube| cube.x).max().unwrap();
    let max_y = grid.cubes.iter().map(|cube| cube.y).max().unwrap();
    let max_z = grid.cubes.iter().map(|cube| cube.z).max().unwrap();

    let min_side = Cube::new(min_x - 1, min_y - 1, min_z - 1);
    let max_side = Cube::new(max_x + 1, max_y + 1, max_z + 1);

    // Now lets flood fill the grid from the outside
    let mut surface_area = 0;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(min_side);
    queue.push_back(max_side);

    while let Some(cube) = queue.pop_front() {
        if visited.contains(&cube) {
            continue;
        }

        visited.insert(cube);

        for neighbour in cube.neighbours() {
            if !in_range(&neighbour, &min_side, &max_side) {
                continue;
            }

            if grid.cubes.contains(&neighbour) {
                surface_area += 1;
            } else if !visited.contains(&neighbour) {
                queue.push_back(neighbour);
            }
        }
    }

    surface_area
}
