use std::fs;
use std::str::FromStr;

/// PART 1
///
/// The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
///
/// Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
///
/// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
///
/// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
///
/// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
///
/// For example, suppose you were given the following strategy guide:
///
/// A Y
/// B X
/// C Z
/// This strategy guide predicts and recommends the following:
///
/// In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
/// In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
/// The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
/// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
///
/// What would your total score be if everything goes exactly according to your strategy guide?


fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn beats(&self, other: &Shape) -> bool {
        match self {
            Shape::Rock => other == &Shape::Scissors,
            Shape::Paper => other == &Shape::Rock,
            Shape::Scissors => other == &Shape::Paper,
        }
    }
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Round {
    elf: Shape,
    me: Shape,
}

impl Round {
    fn new(elf: Shape, me: Shape) -> Round {
        Round { elf, me }
    }
    fn round_score(&self) -> u32 {
        if self.elf == self.me {
            3
        } else if self.elf.beats(&self.me) {
            0
        } else {
            6
        }
    }

    fn total_score(&self) -> u32 {
        self.me.score() + self.round_score()
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let elf = parts.next().unwrap().parse().unwrap();
        let me = parts.next().unwrap().parse().unwrap();
        Ok(Round::new(elf, me))
    }
}


fn process_part1(text: String) -> i32 {
    let rounds = process_rounds1(text);
    rounds.iter().map(|r| r.total_score() as i32).sum()
}

fn process_rounds1(text: String) -> Vec<Round> {
    text.split('\n').map(|s| s.parse().unwrap()).collect()
}

// --- Part Two ---
//
// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
//
// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//
// In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
// In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
// In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

fn process_part2(text: String) -> i32 {
    let rounds = process_rounds2(text);
    rounds.iter().map(|r| r.total_score() as i32).sum()
}

fn process_rounds2(text: String) -> Vec<Round> {
    text.split('\n').map(decode_round).collect()
}

fn decode_round(text: &str) -> Round {
    let mut parts = text.split_whitespace();
    let elf = parts.next().unwrap().parse().unwrap();

    // If the second part is X, I need to lose, so I need to play the shape that beats the elf's shape
    // If the second part is Y, I need to draw, so I need to play the same shape as the elf
    // If the second part is Z, I need to win, so I need to play the shape that the elf loses to
    let me = match parts.next().unwrap() {
        "X" => match elf {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        "Y" => elf,
        "Z" => match elf {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        _ => panic!("Invalid input"),
    };

    Round::new(elf, me)
}