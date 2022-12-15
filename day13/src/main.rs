use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{write, Display, Formatter};
use std::fs;
use std::ops::Not;
use std::str::FromStr;
use OrderingResult::{Correct, Inconclusive, Incorrect};

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

#[derive(Debug, PartialEq)]
enum Data {
    List(Vec<Data>),
    Number(i32),
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::List(list) => {
                let inner = list
                    .iter()
                    .map(|data| data.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", inner)
            }
            Data::Number(number) => write!(f, "{}", number),
        }
    }
}

impl FromStr for Data {
    type Err = ();

    /// Parse the input into a Data structure
    /// Example: [1,2,3] -> List([Number(1), Number(2), Number(3)])
    /// Example: 1 -> Number(1)
    /// Example: [1,[2,3]] -> List([Number(1), List([Number(2), Number(3)])])
    /// Example: [[[[3]]]] -> List([List([List([List([Number(3)])])])])
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.next().unwrap_or_default();
        let last = chars.last().unwrap_or_default();
        if first == '[' && last == ']' {
            let mut list = Vec::new();
            let mut current = String::new();
            let mut depth = 0;
            for c in s.chars().skip(1).take(s.len() - 2) {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                }
                if depth == 0 && c == ',' {
                    list.push(current.parse().unwrap());
                    current = String::new();
                } else {
                    current.push(c);
                }
            }
            if !current.is_empty() {
                list.push(current.parse().unwrap());
            }
            Ok(Data::List(list))
        } else {
            Ok(Data::Number(s.parse().unwrap_or_default()))
        }
    }
}

fn parse_two_packets(text: &str) -> (Data, Data) {
    let mut packets = text.lines();
    let packet1 = packets.next().unwrap().parse().unwrap();
    let packet2 = packets.next().unwrap().parse().unwrap();
    (packet1, packet2)
}

fn parse_input(text: String) -> Vec<(Data, Data)> {
    text.split("\n\n").map(parse_two_packets).collect()
}

fn process_part1(text: String) -> i32 {
    let packets = parse_input(text);

    packets
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| in_order(left, right).is_correct())
        .map(|(i, _)| i as i32 + 1)
        .sum()
}

fn process_part2(text: String) -> i32 {
    let mut packets = text
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Data>>();

    // Divider packets
    let divider_packet_1 = "[[2]]".parse().unwrap();
    let divider_packet_2 = "[[6]]".parse().unwrap();
    packets.push(divider_packet_1);
    packets.push(divider_packet_2);

    // Sort the packets
    packets.sort_by(|left, right| in_order(left, right).to_ordering());

    // Find the index of the divider packets
    let divider_packet_1 = "[[2]]".parse().unwrap();
    let divider_packet_2 = "[[6]]".parse().unwrap();
    let divider_index_1 = packets.iter().position(|p| p == &divider_packet_1).unwrap() + 1;
    let divider_index_2 = packets.iter().position(|p| p == &divider_packet_2).unwrap() + 1;

    (divider_index_1 * divider_index_2) as i32
}

#[derive(Debug, PartialEq)]
enum OrderingResult {
    Correct,
    Inconclusive,
    Incorrect,
}

impl OrderingResult {
    fn is_correct(&self) -> bool {
        !matches!(self, OrderingResult::Incorrect)
    }

    fn to_ordering(&self) -> Ordering {
        match self {
            Correct => Less,
            Inconclusive => Less,
            Incorrect => Greater,
        }
    }
}

impl Not for OrderingResult {
    type Output = bool;

    fn not(self) -> Self::Output {
        !self.is_correct()
    }
}

/// Check if two packets are in order.
/// When comparing two values, the first value is called left and the second value is called right. Then:
///  - If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
///  - If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
///  - If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
fn in_order(left: &Data, right: &Data) -> OrderingResult {
    match (left, right) {
        (Data::Number(l), Data::Number(r)) => match l.cmp(r) {
            Less => Correct,
            Equal => Inconclusive,
            Greater => Incorrect,
        },
        (Data::List(l), Data::List(r)) => {
            let mut l = l.iter();
            let mut r = r.iter();
            loop {
                match (l.next(), r.next()) {
                    (Some(l), Some(r)) => {
                        let result = in_order(l, r);
                        match result {
                            Correct => return result,
                            Inconclusive => continue,
                            Incorrect => return result,
                        }
                    }
                    (Some(_), None) => return Incorrect,
                    (None, Some(_)) => return Correct,
                    (None, None) => return Inconclusive,
                }
            }
        }
        (Data::Number(n), Data::List(_)) => in_order(&Data::List(vec![Data::Number(*n)]), right),
        (Data::List(_), Data::Number(n)) => in_order(left, &Data::List(vec![Data::Number(*n)])),
    }
}

#[test]
fn test_packet_parsing() {
    let packet = "[1,2,3]".parse::<Data>().unwrap();
    assert_eq!(
        packet,
        Data::List(vec![Data::Number(1), Data::Number(2), Data::Number(3)])
    );

    let packet = "1".parse::<Data>().unwrap();
    assert_eq!(packet, Data::Number(1));

    let packet = "[1,[2,3]]".parse::<Data>().unwrap();
    assert_eq!(
        packet,
        Data::List(vec![
            Data::Number(1),
            Data::List(vec![Data::Number(2), Data::Number(3)])
        ])
    );

    let packet = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Data>().unwrap();
    assert_eq!(
        packet,
        Data::List(vec![
            Data::Number(1),
            Data::List(vec![
                Data::Number(2),
                Data::List(vec![
                    Data::Number(3),
                    Data::List(vec![
                        Data::Number(4),
                        Data::List(vec![Data::Number(5), Data::Number(6), Data::Number(0),])
                    ])
                ])
            ]),
            Data::Number(8),
            Data::Number(9),
        ])
    );

    let packet = "[[[]]]".parse::<Data>().unwrap();
    assert_eq!(
        packet,
        Data::List(vec![Data::List(vec![Data::List(vec![])])])
    );
}

#[test]
fn test_in_order_all_items_smaller() {
    let left = "[1, 1, 3, 1, 1]".parse::<Data>().unwrap();
    let right = "[1, 1, 5, 1, 1]".parse::<Data>().unwrap();
    assert!(in_order(&left, &right));
}

#[test]
fn test_in_order_left_array_is_smaller() {
    let left = "[[1],[2,3,4]]".parse::<Data>().unwrap();
    let right = "[[1],4]".parse::<Data>().unwrap();
    assert!(in_order(&left, &right));

    let left = "[[4,4],4,4]".parse::<Data>().unwrap();
    let right = "[[4,4],4,4,4]".parse::<Data>().unwrap();
    assert!(in_order(&left, &right));

    let left = "[]".parse::<Data>().unwrap();
    let right = "[3]".parse::<Data>().unwrap();
    assert!(in_order(&left, &right));
}

#[test]
fn test_not_in_order_left_items_larger() {
    let left = "[9]".parse::<Data>().unwrap();
    let right = "[[8,7,6]]".parse::<Data>().unwrap();
    assert!(!in_order(&left, &right));

    let left = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Data>().unwrap();
    let right = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Data>().unwrap();
    assert!(!in_order(&left, &right));
}

#[test]
fn test_not_in_order_left_array_larger() {
    let left = "[7,7,7,7]".parse::<Data>().unwrap();
    let right = "[7,7,7]".parse::<Data>().unwrap();
    assert!(!in_order(&left, &right));

    let left = "[[[]]]".parse::<Data>().unwrap();
    let right = "[[]]".parse::<Data>().unwrap();
    assert!(!in_order(&left, &right));
}
