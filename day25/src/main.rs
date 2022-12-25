use std::fs;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

/// Convert base 5 to base 10.
/// They symbols are 0, 1, 2, - (-1), = (-2)
/// 0 = 0
/// 1 = 1
/// 2 = 2
/// - = -1
/// = = -2
fn base5_to_base10(s: &str) -> i64 {
    let mut result = 0;
    let mut multiplier = 1;
    for c in s.chars().rev() {
        match c {
            '0' => result += 0,
            '1' => result += multiplier,
            '2' => result += 2 * multiplier,
            '-' => result += -multiplier,
            '=' => result += -2 * multiplier,
            _ => panic!("Invalid character"),
        }
        multiplier *= 5;
    }
    result
}

#[test]
fn test_base5_to_base_10() {
    assert_eq!(base5_to_base10("0"), 0);
    assert_eq!(base5_to_base10("1"), 1);
    assert_eq!(base5_to_base10("2"), 2);
    assert_eq!(base5_to_base10("-"), -1);
    assert_eq!(base5_to_base10("="), -2);
    assert_eq!(base5_to_base10("1="), 3);
    assert_eq!(base5_to_base10("1-"), 4);
    assert_eq!(base5_to_base10("10"), 5);
    assert_eq!(base5_to_base10("11"), 6);
    assert_eq!(base5_to_base10("1=-0-2"), 1747);
    assert_eq!(base5_to_base10("12111"), 906);
    assert_eq!(base5_to_base10("2=0="), 198);
    assert_eq!(base5_to_base10("21"), 11);
    assert_eq!(base5_to_base10("2=01"), 201);
    assert_eq!(base5_to_base10("111"), 31);
    assert_eq!(base5_to_base10("20012"), 1257);
    assert_eq!(base5_to_base10("112"), 32);
    assert_eq!(base5_to_base10("1=-1="), 353);
    assert_eq!(base5_to_base10("1-12"), 107);
    assert_eq!(base5_to_base10("122"), 37);
}

/// Convert base 10 to base 5.
/// They symbols are 0, 1, 2, - (-1), = (-2)
/// 0 = 0
/// 1 = 1
/// 2 = 2
/// - = -1
/// = = -2
fn base10_to_base5(decimal: i64) -> String {
    let mut result = String::new();
    let mut remainder = decimal;
    while remainder != 0 {
        let digit = remainder % 5;
        let char = match digit {
            digit if digit < 3 => std::char::from_digit(digit as u32, 10).unwrap(),
            digit if digit == 3 => {
                remainder += 2;
                '='
            }
            _ => {
                remainder += 1;
                '-'
            }
        };
        result.push(char);
        remainder /= 5;
    }
    result.chars().rev().collect()
}

#[test]
fn test_base10_to_base5() {
    // assert_eq!(base10_to_base5(0), "0");
    assert_eq!(base10_to_base5(1), "1");
    assert_eq!(base10_to_base5(2), "2");
    // assert_eq!(base10_to_base5(-1), "-");
    // assert_eq!(base10_to_base5(-2), "=");
    assert_eq!(base10_to_base5(3), "1=");
    assert_eq!(base10_to_base5(4), "1-");
    assert_eq!(base10_to_base5(5), "10");
    assert_eq!(base10_to_base5(6), "11");
    assert_eq!(base10_to_base5(1747), "1=-0-2");
    assert_eq!(base10_to_base5(906), "12111");
    assert_eq!(base10_to_base5(198), "2=0=");
    assert_eq!(base10_to_base5(11), "21");
    assert_eq!(base10_to_base5(201), "2=01");
    assert_eq!(base10_to_base5(31), "111");
    assert_eq!(base10_to_base5(1257), "20012");
    assert_eq!(base10_to_base5(32), "112");
    assert_eq!(base10_to_base5(353), "1=-1=");
    assert_eq!(base10_to_base5(107), "1-12");
    assert_eq!(base10_to_base5(37), "122");
}

fn process_part1(text: String) -> String {
    base10_to_base5(text.lines().map(base5_to_base10).sum())
}

fn process_part2(text: String) -> i32 {
    0
}
