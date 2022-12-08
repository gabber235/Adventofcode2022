use colored::Colorize;
use std::fmt::Display;
use std::fs;
use std::ops::Not;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// --- Part 1 ---
// The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.
// First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.
// The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
// All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:
//
// The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
// The top-middle 5 is visible from the top and right.
// The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
// The left-middle 5 is visible, but only from the right.
// The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
// The right-middle 3 is visible from the right.
// In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
// With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.
//
// Consider your map; how many trees are visible from outside the grid?

#[derive(Debug)]
struct TreeVisibility {
    height: u8,
    visible: bool,
}

impl Display for TreeVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f, "{}", format!("{}", self.height).green().bold())
        } else {
            write!(f, "{}", format!("{}", self.height).red())
        }
    }
}

fn parse_input_part1(text: String) -> Vec<Vec<TreeVisibility>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|c| TreeVisibility {
                    height: c.to_digit(10).unwrap() as u8,
                    visible: false,
                })
                .collect()
        })
        .collect()
}

fn is_edge_tree(grid: &Vec<Vec<TreeVisibility>>, x: usize, y: usize) -> bool {
    x == 0 || y == 0 || x == grid.len() - 1 || y == grid[0].len() - 1
}

fn is_visible(grid: &Vec<Vec<TreeVisibility>>, x: usize, y: usize) -> bool {
    if is_edge_tree(grid, x, y) {
        return true;
    }

    let height = grid[y][x].height;

    grid[y].iter().take(x).any(|t| t.height >= height).not()
        || grid[y].iter().skip(x + 1).any(|t| t.height >= height).not()
        || grid.iter().take(y).any(|row| row[x].height >= height).not()
        || grid
            .iter()
            .skip(y + 1)
            .any(|row| row[x].height >= height)
            .not()
}

fn process_part1(text: String) -> usize {
    let mut grid = parse_input_part1(text);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x].visible = is_visible(&grid, x, y);
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|tree| tree.visible).count())
        .sum()
}

// --- Part Two ---
//
// Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.
//
// To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)
//
// The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.
//
// In the example above, consider the middle 5 in the second row:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Looking up, its view is not blocked; it can see 1 tree (of height 3).
// Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
// Looking right, its view is not blocked; it can see 2 trees.
// Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
// A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
//
// However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
// Looking left, its view is not blocked; it can see 2 trees.
// Looking down, its view is also not blocked; it can see 1 tree.
// Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
// This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
//
// Consider each tree on your map. What is the highest scenic score possible for any tree?

struct TreeScenic {
    height: u8,
    scenic: u32,
}

impl Display for TreeScenic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            format!("{}", self.height).green().bold(),
            format!("[{}]", self.scenic).bright_black().italic()
        )
    }
}

fn parse_input_part2(text: String) -> Vec<Vec<TreeScenic>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|c| TreeScenic {
                    height: c.to_digit(10).unwrap() as u8,
                    scenic: 0,
                })
                .collect()
        })
        .collect()
}

trait CountUntilInclusive: Iterator {
    fn count_until_inclusive<F>(self, predicate: F) -> usize
    where
        F: Fn(&Self::Item) -> bool;
}

impl<T, I: Iterator<Item = T>> CountUntilInclusive for I {
    fn count_until_inclusive<F>(self, predicate: F) -> usize
    where
        F: Fn(&Self::Item) -> bool,
    {
        let mut count = 0;
        for item in self {
            count += 1;
            if predicate(&item) {
                break;
            }
        }
        count
    }
}

fn scenic_score(grid: &[Vec<TreeScenic>], x: usize, y: usize) -> u32 {
    let height = grid[y][x].height;

    grid[y]
        .iter()
        .take(x)
        .rev()
        .count_until_inclusive(|t| t.height >= height) as u32
        * grid[y]
            .iter()
            .skip(x + 1)
            .count_until_inclusive(|t| t.height >= height) as u32
        * grid
            .iter()
            .take(y)
            .rev()
            .count_until_inclusive(|row| row[x].height >= height) as u32
        * grid
            .iter()
            .skip(y + 1)
            .count_until_inclusive(|row| row[x].height >= height) as u32
}

fn process_part2(text: String) -> u32 {
    let mut grid = parse_input_part2(text);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x].scenic = scenic_score(&grid, x, y);
        }
    }

    grid.iter()
        .map(|row| row.iter().map(|tree| tree.scenic).max().unwrap())
        .max()
        .unwrap()
}
