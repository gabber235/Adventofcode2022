use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// --- Day 16: Proboscidea Volcanium ---
// The sensors have led you to the origin of the distress signal: yet another handheld device, just like the one the Elves gave you. However, you don't see any Elves around; instead, the device is surrounded by elephants! They must have gotten lost in these tunnels, and one of the elephants apparently figured out how to turn on the distress signal.
// The ground rumbles again, much stronger this time. What kind of cave is this, exactly? You scan the cave with your handheld device; it reports mostly igneous rock, some ash, pockets of pressurized gas, magma... this isn't just a cave, it's a volcano!
// You need to get the elephants out of here, quickly. Your device estimates that you have 30 minutes before the volcano erupts, so you don't have time to go back out the way you came in.
// You scan the cave for other options and discover a network of pipes and pressure-release valves. You aren't sure how such a system got into a volcano, but you don't have time to complain; your device produces a report (your puzzle input) of each valve's flow rate if it were opened (in pressure per minute) and the tunnels you could use to move between the valves.
// There's even a valve in the room you and the elephants are currently standing in labeled AA. You estimate it will take you one minute to open a single valve and one minute to follow any tunnel from one valve to another. What is the most pressure you could release?
// For example, suppose you had the following scan output:
//
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
// Valve EE has flow rate=3; tunnels lead to valves FF, DD
// Valve FF has flow rate=0; tunnels lead to valves EE, GG
// Valve GG has flow rate=0; tunnels lead to valves FF, HH
// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve II has flow rate=0; tunnels lead to valves AA, JJ
// Valve JJ has flow rate=21; tunnel leads to valve II
// All of the valves begin closed. You start at valve AA, but it must be damaged or jammed or something: its flow rate is 0, so there's no point in opening it. However, you could spend one minute moving to valve BB and another minute opening it; doing so would release pressure during the remaining 28 minutes at a flow rate of 13, a total eventual pressure release of 28 * 13 = 364. Then, you could spend your third minute moving to valve CC and your fourth minute opening it, providing an additional 26 minutes of eventual pressure release at a flow rate of 2, or 52 total pressure released by valve CC.
//
// Making your way through the tunnels like this, you could probably open many or all of the valves by the time 30 minutes have elapsed. However, you need to release as much pressure as possible, so you'll need to be methodical.

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
                .unwrap();
        let caps = re.captures(s).expect("Invalid input");
        let name = caps.get(1).unwrap().as_str().to_string();
        let flow_rate = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let tunnels = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        Ok(Valve {
            name,
            flow_rate,
            tunnels,
        })
    }
}

#[test]
fn test_valve_from_string() {
    let string = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
    let valve: Valve = string.parse().unwrap();
    assert_eq!(valve.name, "AA");
    assert_eq!(valve.flow_rate, 0);
    assert_eq!(valve.tunnels, vec!["DD", "II", "BB"]);

    let string = "Valve HH has flow rate=22; tunnel leads to valve GG";
    let valve: Valve = string.parse().unwrap();
    assert_eq!(valve.name, "HH");
    assert_eq!(valve.flow_rate, 22);
    assert_eq!(valve.tunnels, vec!["GG"]);
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Valve>,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s
            .lines()
            .map(|s| s.parse().unwrap())
            .map(|v: Valve| (v.name.clone(), v))
            .collect();
        Ok(Cave { valves })
    }
}

struct Choice {
    name: String,
    // The total amount that can be released after the given time.
    total_released: u64,
    opened: HashSet<String>,
}

fn calculate_new_choices(cave: &Cave, start_point: &Choice, time_left: u64) -> Vec<Choice> {
    let mut new_choices = Vec::new();

    let valve = cave.valves.get(&start_point.name).unwrap();
    if valve.flow_rate > 0 && !start_point.opened.contains(&valve.name) {
        // Open this valve.
        let mut opened = start_point.opened.clone();
        opened.insert(valve.name.clone());

        let new_release = start_point.total_released + time_left * valve.flow_rate;

        let name = valve.name.clone();

        new_choices.push(Choice {
            name,
            total_released: new_release,
            opened,
        });
    }
    for tunnel in &valve.tunnels {
        new_choices.push(Choice {
            name: tunnel.clone(),
            total_released: start_point.total_released,
            opened: start_point.opened.clone(),
        });
    }

    new_choices
}

fn maximal_released(
    cave: &Cave,
    opened: HashSet<String>,
    current_released: u64,
    time_left: u64,
) -> u64 {
    // Calculate the maximal amount of pressure that can be released in the given time.
    cave.valves
        .iter()
        .filter(|(_, v)| v.flow_rate > 0 && !opened.contains(&v.name))
        .map(|(_, v)| v.flow_rate)
        .sum::<u64>()
        * time_left
        + current_released
}

fn process_part1(text: String) -> i64 {
    let cave: Cave = text.parse().unwrap();

    let mut choices = HashMap::new();
    choices.insert(
        (0, "AA".to_string()),
        Choice {
            name: "AA".to_string(),
            total_released: 0,
            opened: HashSet::new(),
        },
    );

    let max_time = 30;

    for minute in 0..max_time {
        choices = choices
            .into_par_iter()
            .flat_map(|(_, choice)| {
                calculate_new_choices(&cave, &choice, max_time - minute)
                    .into_par_iter()
                    .collect::<Vec<Choice>>()
            })
            .map(|c| ((c.total_released, c.name.clone()), c))
            .collect();
    }

    choices
        .iter()
        .map(|(_, c)| c.total_released as i64)
        .max()
        .unwrap()
}

// --- Part Two ---
// You're worried that even with an optimal approach, the pressure released won't be enough. What if you got one of the elephants to help you?
// It would take you 4 minutes to teach an elephant how to open the right valves in the right order, leaving you with only 26 minutes to actually execute your plan. Would having two of you working together be better, even if it means having less time? (Assume that you teach the elephant before opening any valves yourself, giving you both the same full 26 minutes.)

struct Choice2 {
    you: String,
    elephant: String,
    // The total amount that can be released after the given time.
    total_released: u64,
    opened: HashSet<String>,
}

fn compute_new_choices2(cave: &Cave, start_point: &Choice2, time_left: u64) -> Vec<Choice2> {
    let mut new_choices = Vec::new();
    let start_point_you = Choice {
        name: start_point.you.clone(),
        total_released: 0,
        opened: start_point.opened.clone(),
    };
    let start_point_elephant = Choice {
        name: start_point.elephant.clone(),
        total_released: 0,
        opened: start_point.opened.clone(),
    };

    let new_choices_for_you = calculate_new_choices(cave, &start_point_you, time_left);

    let new_choices_for_elephant = calculate_new_choices(cave, &start_point_elephant, time_left);

    for new_choice_you in new_choices_for_you {
        for new_choice_elephant in &new_choices_for_elephant {
            // We need to check if the elephant is going to open a valve that the you is going to open.

            if new_choice_you
                .opened
                .intersection(&new_choice_elephant.opened)
                .count()
                > start_point.opened.len()
            {
                continue;
            }

            let mut opened = new_choice_you.opened.clone();
            opened.extend(new_choice_elephant.opened.clone());

            let new_release = start_point.total_released
                + new_choice_you.total_released
                + new_choice_elephant.total_released;

            new_choices.push(Choice2 {
                you: new_choice_you.name.clone(),
                elephant: new_choice_elephant.name.clone(),
                total_released: new_release,
                opened,
            });
        }
    }

    new_choices
}

fn process_part2(text: String) -> i64 {
    let cave: Cave = text.parse().unwrap();

    let mut choices = HashMap::new();
    choices.insert(
        (0, "AA".to_string(), "AA".to_string()),
        Choice2 {
            you: "AA".to_string(),
            elephant: "AA".to_string(),
            total_released: 0,
            opened: HashSet::new(),
        },
    );

    let max_time = 26;

    for minute in 0..max_time {
        println!("Minute {}, choices: {}", minute, choices.len());

        let max_released = choices
            .par_iter()
            .map(|(_, c)| c.total_released as i64)
            .max()
            .unwrap();

        choices = choices
            .into_par_iter()
            .flat_map(|(_, choice)| {
                compute_new_choices2(&cave, &choice, max_time - minute - 1)
                    .into_par_iter()
                    .collect::<Vec<Choice2>>()
            })
            // Only if the maximal amount of pressure that can be released is higher than the current
            // maximal amount of pressure that can be released, we need to keep the choice.
            .filter(|c| {
                maximal_released(
                    &cave,
                    c.opened.clone(),
                    c.total_released,
                    max_time - minute - 1,
                ) as i64
                    >= max_released
            })
            .map(|c| ((c.total_released, c.you.clone(), c.elephant.clone()), c))
            .collect();
    }

    choices
        .iter()
        .map(|(_, c)| c.total_released as i64)
        .max()
        .unwrap()
}
