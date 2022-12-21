use std::collections::VecDeque;
use std::fs;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(values: &[i64], key: i64, rounds: usize) -> i64 {
    let mut values = values
        .iter()
        .map(|v| *v * key)
        .enumerate()
        .collect::<VecDeque<_>>();

    for _ in 0..rounds {
        for i in 0..values.len() {
            let current_index = values.iter().position(|(index, _)| *index == i).unwrap();

            values.rotate_left(current_index);
            let (index, value) = values.pop_front().unwrap();
            let d = value.rem_euclid(values.len() as i64) as usize;
            values.rotate_left(d);
            values.push_front((index, value));
        }
    }

    let zero_index = values.iter().position(|(_, value)| *value == 0).unwrap();

    let position_of_interest = vec![1000, 2000, 3000];

    position_of_interest
        .iter()
        .map(|pos| values[(zero_index + pos) % values.len()].1 as i64)
        .sum()
}

fn process_part1(text: String) -> i64 {
    let original_numbers = parse_input(&text);

    solve(&original_numbers, 1, 1)
}

fn process_part2(text: String) -> i64 {
    let original_numbers = parse_input(&text);

    solve(&original_numbers, 811_589_153, 10)
}
