use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// Your scans show that the lava did indeed form obsidian!
// The wind has changed direction enough to stop sending lava droplets toward you, so you and the elephants exit the cave. As you do, you notice a collection of geodes around the pond. Perhaps you could use the obsidian to create some geode-cracking robots and break them open?
// To collect the obsidian from the bottom of the pond, you'll need waterproof obsidian-collecting robots. Fortunately, there is an abundant amount of clay nearby that you can use to make them waterproof.
// In order to harvest the clay, you'll need special-purpose clay-collecting robots. To make any type of robot, you'll need ore, which is also plentiful but in the opposite direction from the clay.
// Collecting ore requires ore-collecting robots with big drills. Fortunately, you have exactly one ore-collecting robot in your pack that you can use to kickstart the whole operation.
// Each robot can collect 1 of its resource type per minute. It also takes one minute for the robot factory (also conveniently from your pack) to construct any type of robot, although it consumes the necessary resources available when construction begins.
// The robot factory has many blueprints (your puzzle input) you can choose from, but once you've configured it with a blueprint, you can't change it. You'll need to work out which blueprint is best.
// For example:
//
// Blueprint 1:
// Each ore robot costs 4 ore.
// Each clay robot costs 2 ore.
// Each obsidian robot costs 3 ore and 14 clay.
// Each geode robot costs 2 ore and 7 obsidian.
//
// Blueprint 2:
// Each ore robot costs 2 ore.
// Each clay robot costs 3 ore.
// Each obsidian robot costs 3 ore and 8 clay.
// Each geode robot costs 3 ore and 12 obsidian.
// (Blueprints have been line-wrapped here for legibility. The robot factory's actual assortment of blueprints are provided one blueprint per line.)
// The elephants are starting to look hungry, so you shouldn't take too long; you need to figure out which blueprint would maximize the number of opened geodes after 24 minutes by figuring out which robots to build and when to build them.
// Determine the quality level of each blueprint by multiplying that blueprint's ID number with the largest number of geodes that can be opened in 24 minutes using that blueprint. In this example, the first blueprint has ID 1 and can open 9 geodes, so its quality level is 9. The second blueprint has ID 2 and can open 12 geodes, so its quality level is 24. Finally, if you add up the quality levels of all of the blueprints in the list, you get 33.
// Determine the quality level of each blueprint using the largest number of geodes it could produce in 24 minutes. What do you get if you add up the quality level of all of the blueprints in your list?

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_ore_cost: i32,
    clay_ore_cost: i32,
    obsidian_ore_cost: i32,
    obsidian_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obsidian_cost: i32,
}

impl FromStr for Blueprint {
    type Err = ();

    /// Parse a blueprint string
    /// Example String:
    /// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 16 obsidian.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(\d+)").unwrap();
        let caps = regex
            .find_iter(s)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Ok(Blueprint {
            id: caps[0],
            ore_ore_cost: caps[1],
            clay_ore_cost: caps[2],
            obsidian_ore_cost: caps[3],
            obsidian_clay_cost: caps[4],
            geode_ore_cost: caps[5],
            geode_obsidian_cost: caps[6],
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
struct State {
    time: u32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl State {
    fn earn(&self) -> Self {
        let mut new_state = self.clone();
        new_state.time += 1;
        new_state.ore += self.ore_robots;
        new_state.clay += self.clay_robots;
        new_state.obsidian += self.obsidian_robots;
        new_state.geode += self.geode_robots;
        new_state
    }
}

fn simulate_blueprint(blueprint: &Blueprint, max_time: u32) -> i128 {
    let mut states = VecDeque::new();
    let mut seen = HashSet::new();

    states.push_back(State {
        ore_robots: 1,
        ..Default::default()
    });

    let max_ore_cost = *[
        blueprint.ore_ore_cost,
        blueprint.clay_ore_cost,
        blueprint.obsidian_ore_cost,
        blueprint.geode_ore_cost,
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let mut max_geodes = 0;

    while let Some(state) = states.pop_front() {
        if state.time >= max_time {
            max_geodes = max_geodes.max(state.geode);
            continue;
        }
        if seen.contains(&state) {
            continue;
        }

        seen.insert(state.clone());

        if state.ore_robots < max_ore_cost && state.ore >= blueprint.ore_ore_cost {
            let mut new_state = state.earn();
            new_state.ore -= blueprint.ore_ore_cost;
            new_state.ore_robots += 1;
            states.push_back(new_state);
        }
        if state.clay_robots < blueprint.obsidian_clay_cost && state.ore >= blueprint.clay_ore_cost
        {
            let mut new_state = state.earn();
            new_state.ore -= blueprint.clay_ore_cost;
            new_state.clay_robots += 1;
            states.push_back(new_state);
        }
        if state.obsidian_robots < blueprint.geode_obsidian_cost
            && state.ore >= blueprint.obsidian_ore_cost
            && state.clay >= blueprint.obsidian_clay_cost
        {
            let mut new_state = state.earn();
            new_state.ore -= blueprint.obsidian_ore_cost;
            new_state.clay -= blueprint.obsidian_clay_cost;
            new_state.obsidian_robots += 1;
            states.push_back(new_state);
        }

        if state.ore >= blueprint.geode_ore_cost && state.obsidian >= blueprint.geode_obsidian_cost
        {
            let mut new_state = state.earn();
            new_state.ore -= blueprint.geode_ore_cost;
            new_state.obsidian -= blueprint.geode_obsidian_cost;
            new_state.geode_robots += 1;
            states.push_back(new_state);
        }

        states.push_back(state.earn());
    }

    max_geodes as i128
}

fn process_part1(text: String) -> i128 {
    let blueprints: Vec<Blueprint> = text
        .lines()
        .map(|l| Blueprint::from_str(l).unwrap())
        .collect();

    blueprints
        .par_iter()
        .map(|b| simulate_blueprint(b, 24) * (b.id as i128))
        .sum()
}

// --- Part Two ---
// While you were choosing the best blueprint, the elephants found some food on their own, so you're not in as much of a hurry; you figure you probably have 32 minutes before the wind changes direction again and you'll need to get out of range of the erupting volcano.
// Unfortunately, one of the elephants ate most of your blueprint list! Now, only the first three blueprints in your list are intact.
// In 32 minutes, the largest number of geodes blueprint 1 (from the example above) can open is 56. One way to achieve that is:
// However, blueprint 2 from the example above is still better; using it, the largest number of geodes you could open in 32 minutes is 62.
// You no longer have enough blueprints to worry about quality levels. Instead, for each of the first three blueprints, determine the largest number of geodes you could open; then, multiply these three values together.
// Don't worry about quality levels; instead, just determine the largest number of geodes you could open using each of the first three blueprints. What do you get if you multiply these numbers together?

fn process_part2(text: String) -> i128 {
    let blueprints: Vec<Blueprint> = text
        .lines()
        .map(|l| Blueprint::from_str(l).unwrap())
        .collect();

    blueprints
        .par_iter()
        .take(3)
        .map(|b| simulate_blueprint(b, 32))
        .product()
}
