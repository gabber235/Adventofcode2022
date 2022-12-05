use std::cmp::Ordering;
use std::fs;
use std::iter::Sum;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}
fn process_part1(text: String) -> i32 {
    let elfs = get_elfs(text);

    elfs.iter().max().unwrap().0.iter().sum()
}

fn process_part2(text: String) -> i32 {
    let mut elfs = get_elfs(text);

    // Get the top 3 elfs
    elfs.sort();
    elfs.reverse();
    let best3: Vec<i32> = elfs.iter().take(3).map(|elf| elf.total()).collect();
    best3.iter().sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Elf(Vec<i32>);

impl Elf {
    fn total(&self) -> i32 {
        self.0.iter().sum()
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total().cmp(&other.total()))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_elfs(text: String) -> Vec<Elf> {
    // Split the text by empty lines
    let groups = text.split("\n\n");
    let elfs: Vec<Elf> = groups.map(fetch_calories_for_elf).collect();
    elfs
}

fn fetch_calories_for_elf(carry: &str) -> Elf {
    let calories = carry
        .split('\n')
        .map(|c| c.parse::<i32>().unwrap_or(0))
        .collect();
    Elf(calories)
}
