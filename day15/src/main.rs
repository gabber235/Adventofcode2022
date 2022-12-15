use rayon::prelude::*;
use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels. You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.
// The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them. You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.
// Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon. Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely; however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance. (There is never a tie where two beacons are the same distance to a sensor.)
// None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't. For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.
// So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    closest_beacon: Beacon,
    manhattan_distance: i32,
}

impl FromStr for Sensor {
    type Err = ();

    /// Parse a sensor from a string
    /// Example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"Sensor at x=(?P<x>-?\d+), y=(?P<y>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let x = caps["x"].parse().unwrap();
        let y = caps["y"].parse().unwrap();
        let bx = caps["bx"].parse().unwrap();
        let by = caps["by"].parse().unwrap();

        let position = Point { x, y };
        let closest_beacon = Beacon {
            position: Point { x: bx, y: by },
        };
        let manhattan_distance = position.manhattan_distance(&closest_beacon.position);

        Ok(Sensor {
            position,
            closest_beacon,
            manhattan_distance,
        })
    }
}

impl Sensor {
    fn get_all_points_just_outside_reach(&self) -> Vec<Point> {
        let x = self.position.x;
        let y = self.position.y;
        let manhattan_distance = self.manhattan_distance;

        (0..(manhattan_distance + 1))
            .into_iter()
            .flat_map(|d| {
                [
                    Point {
                        x: x - manhattan_distance - 1 + d,
                        y: y - d,
                    },
                    Point {
                        x: x - d,
                        y: y + manhattan_distance + 1 - d,
                    },
                    Point {
                        x: x + manhattan_distance + 1 - d,
                        y: y + d,
                    },
                    Point {
                        x: x + d,
                        y: y - manhattan_distance - 1 + d,
                    },
                ]
            })
            .collect()
    }
}

#[derive(Debug)]
struct Beacon {
    position: Point,
}

#[derive(Debug)]
struct Cave {
    sensors: Vec<Sensor>,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensors = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Cave { sensors })
    }
}

impl Cave {
    // The minimum x is defined as the minimum x of all sensors minus its manhattan distance
    fn min_x(&self) -> i32 {
        self.sensors
            .iter()
            .map(|s| s.position.x - s.manhattan_distance)
            .min()
            .unwrap()
    }

    // The maximum x is defined as the maximum x of all sensors plus its manhattan distance
    fn max_x(&self) -> i32 {
        self.sensors
            .iter()
            .map(|s| s.position.x + s.manhattan_distance)
            .max()
            .unwrap()
    }

    // The minimum y is defined as the minimum y of all sensors minus its manhattan distance
    fn min_y(&self) -> i32 {
        self.sensors
            .iter()
            .map(|s| s.position.y - s.manhattan_distance)
            .min()
            .unwrap()
    }

    // The maximum y is defined as the maximum y of all sensors plus its manhattan distance
    fn max_y(&self) -> i32 {
        self.sensors
            .iter()
            .map(|s| s.position.y + s.manhattan_distance)
            .max()
            .unwrap()
    }

    fn print(&self) {
        self.print_with_bounds(self.min_x(), self.max_x(), self.min_y(), self.max_y())
    }

    // Print the cave. The sensors are represented by a 'S' and the beacons by a 'B'
    // Empty space is represented by a '.'
    // Since the sensors are not able to detect the beacons outside of their manhattan distance, we
    // print a '#' for the points inside the manhattan distance
    fn print_with_bounds(&self, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = Point { x, y };
                if self.sensors.iter().any(|s| s.position == point) {
                    print!("S");
                } else if self
                    .sensors
                    .iter()
                    .any(|s| s.closest_beacon.position == point)
                {
                    print!("B");
                } else if self
                    .sensors
                    .iter()
                    .any(|s| point.manhattan_distance(&s.position) <= s.manhattan_distance)
                {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn process_part1(text: String) -> i32 {
    let cave = text.parse::<Cave>().unwrap();

    let y = 2000000;

    // Count the number of points inside the manhattan distance of all sensors at y=10
    let min_x = cave.min_x();
    let max_x = cave.max_x();

    (min_x..=max_x)
        .map(|x| Point { x, y })
        .map(|point| {
            i32::from(cave.sensors.iter().any(|s| {
                s.position != point
                    && s.closest_beacon.position != point
                    && point.manhattan_distance(&s.position) <= s.manhattan_distance
            }))
        })
        .sum::<i32>()
}

// --- Part Two ---
// Your handheld device indicates that the distress signal is coming from a beacon nearby. The distress beacon is not detected by any sensor, but the distress beacon must have x and y coordinates each no lower than 0 and no larger than 4000000.
// To isolate the distress beacon's signal, you need to determine its tuning frequency, which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.
// In the example above, the search space is smaller: instead, the x and y coordinates can each be at most 20. With this reduced search area, there is only a single position that could have a beacon: x=14, y=11. The tuning frequency for this distress beacon is 56000011.
// Find the only possible position for the distress beacon. What is its tuning frequency?

fn find_empty_space_within_bounds(
    cave: &Cave,
    lower_bound: i32,
    upper_bound: i32,
) -> Option<Point> {
    // Get all the bordering points just outside the manhattan distance of all sensors (+1)
    // Then filter out the points that are outside of the bounds
    // Then filter out the points that are inside the manhattan distance of any sensor
    // If there is at least one point, return it

    cave.sensors
        .par_iter()
        .flat_map(|sensor| sensor.get_all_points_just_outside_reach())
        .filter(|point| {
            point.x >= lower_bound
                && point.x <= upper_bound
                && point.y >= lower_bound
                && point.y <= upper_bound
        })
        .filter(|point| {
            !cave
                .sensors
                .iter()
                .any(|s| point.manhattan_distance(&s.position) <= s.manhattan_distance)
        })
        .find_first(|point| {
            !cave
                .sensors
                .iter()
                .any(|s| point.manhattan_distance(&s.position) <= s.manhattan_distance)
        })
}

fn process_part2(text: String) -> i64 {
    let cave = text.parse::<Cave>().unwrap();

    let lower_bound = 0;
    let upper_bound = 4000000;

    let beacon_point = find_empty_space_within_bounds(&cave, lower_bound, upper_bound);

    if let Some(beacon_point) = beacon_point {
        (beacon_point.x as i64) * 4000000 + (beacon_point.y as i64)
    } else {
        panic!("No beacon point found");
    }
}
