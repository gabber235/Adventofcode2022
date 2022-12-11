use std::fs;
use std::str::FromStr;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// --- Part 1 ---
// You avoid the ropes, plunge into the river, and swim to shore.
// The Elves yell something about meeting back up with them upriver, but the river is too loud to tell exactly what they're saying. They finish crossing the bridge and disappear from view.
// Situations like this must be why the Elves prioritized getting the communication system on your handheld device working. You pull it out of your pack, but the amount of water slowly draining from a big crack in its screen tells you it probably won't be of much immediate use.
// Unless, that is, you can design a replacement for the device's video system! It seems to be some kind of cathode-ray tube screen and simple CPU that are both driven by a precise clock circuit. The clock circuit ticks at a constant rate; each tick is called a cycle.
// Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which starts with the value 1. It supports only two instructions:
//
// addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
// noop takes one cycle to complete. It has no other effect.
// The CPU uses these instructions in a program (your puzzle input) to, somehow, tell the screen what to draw.
//
// Consider the following small program:
//
// noop
// addx 3
// addx -5
// Execution of this program proceeds as follows:
//
// At the start of the first cycle, the noop instruction begins execution. During the first cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing nothing.
// At the start of the second cycle, the addx 3 instruction begins execution. During the second cycle, X is still 1.
// During the third cycle, X is still 1. After the third cycle, the addx 3 instruction finishes execution, setting X to 4.
// At the start of the fourth cycle, the addx -5 instruction begins execution. During the fourth cycle, X is still 4.
// During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction finishes execution, setting X to -1.
// Maybe you can learn something by looking at the value of the X register throughout execution. For now, consider the signal strength (the cycle number multiplied by the value of the X register) during the 20th cycle and every 40 cycles after that (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).
//
// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the sum of these six signal strengths?

enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    fn cycle(&self, x: &mut i32) {
        match self {
            Instruction::AddX(v) => *x += v,
            Instruction::Noop => (),
        }
    }

    fn cycle_duration(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("addx") => Ok(Instruction::AddX(parts.next().unwrap().parse().unwrap())),
            Some("noop") => Ok(Instruction::Noop),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn print_special_cycles(cycle: i32, x: i32, signal_strength: &mut i32) {
    if (cycle - 20) % 40 == 0 {
        println!(
            "Cycle {}: X = {}, signal strength = {}",
            cycle,
            x,
            cycle * x
        );
        *signal_strength += cycle * x;
    }
}

fn process_part1(text: String) -> i32 {
    let instructions = parse_input(&text);

    let mut x = 1;

    let mut cycle = 0;

    let mut signal_strength = 0;

    for instruction in instructions {
        for _ in 0..instruction.cycle_duration() {
            cycle += 1;
            print_special_cycles(cycle, x, &mut signal_strength);
        }
        instruction.cycle(&mut x);
    }

    signal_strength
}

// --- Part Two ---
//
// It seems like the X register controls the horizontal position of a sprite. Specifically, the sprite is 3 pixels wide, and the X register sets the horizontal position of the middle of that sprite. (In this system, there is no such thing as "vertical position": if the sprite's horizontal position puts its pixels where the CRT is currently drawing, then those pixels will be drawn.)
//
// You count the pixels on the CRT: 40 wide and 6 high. This CRT screen draws the top row of pixels left-to-right, then the row below that, and so on. The left-most pixel in each row is in position 0, and the right-most pixel in each row is in position 39.
//
// Like the CPU, the CRT is tied closely to the clock circuit: the CRT draws a single pixel during each cycle. Representing each pixel of the screen as a #, here are the cycles during which the first and last pixel in each row are drawn:
//
// Cycle   1 -> ######################################## <- Cycle  40
// Cycle  41 -> ######################################## <- Cycle  80
// Cycle  81 -> ######################################## <- Cycle 120
// Cycle 121 -> ######################################## <- Cycle 160
// Cycle 161 -> ######################################## <- Cycle 200
// Cycle 201 -> ######################################## <- Cycle 240
// So, by carefully timing the CPU instructions and the CRT drawing operations, you should be able to determine whether the sprite is visible the instant each pixel is drawn. If the sprite is positioned such that one of its three pixels is the pixel currently being drawn, the screen produces a lit pixel (#); otherwise, the screen leaves the pixel dark (.).
//
// Render the image given by your program. What eight capital letters appear on your CRT?

fn process_part2(text: String) -> i32 {
    let instructions = parse_input(&text);
    let mut x: i32 = 1;
    let mut cycle = 0;

    let mut screen = vec![vec![' '; 40]; 6];

    for instruction in instructions {
        for _ in 0..instruction.cycle_duration() {
            let position = cycle % 40;
            // If the position is one more or less than the X register, then the sprite is visible.

            screen[cycle as usize / 40][position as usize] =
                if ((x - 1)..=(x + 1)).contains(&position) {
                    '#'
                } else {
                    '.'
                };
            print_screen(x, cycle, &screen);
            cycle += 1;
        }
        instruction.cycle(&mut x);
    }

    0
}

fn print_screen(x: i32, cycle: i32, screen: &[Vec<char>]) {
    println!("||||||||||||||||| {} [{}] ||||||||||||||||||||", cycle, x);
    for (row_index, row) in screen.iter().enumerate() {
        // Print the character from the screen. If the cycle in the row and the column index is (x-1)..=(x+1), then print the character in red if it is visible, otherwise print a X.
        for (column_index, c) in row.iter().enumerate() {
            if ((x - 1)..=(x + 1)).contains(&(column_index as i32)) {
                if cycle as usize / 40 == row_index {
                    print!(
                        "\x1b[31m{}\x1b[0m",
                        if c.is_whitespace() { 'X' } else { *c }
                    );
                } else {
                    print!("{}", c);
                }
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}
