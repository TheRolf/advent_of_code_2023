use advent_of_code_2023::*;

const DIGITS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digits(row: &String, part2: bool) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    let len: usize = row.len();
    for j in 0..len {
        let current_char: char = row.chars().nth(j).unwrap();
        if current_char.is_digit(10) {
            digits.push(current_char.to_digit(10).unwrap())
        } else if part2 {
            for i in 0..10 {
                if DIGITS[i].len() <= len - j {
                    let splice: String = row
                        .chars()
                        .into_iter()
                        .skip(j)
                        .take(DIGITS[i].len())
                        .collect();
                    if splice == DIGITS[i] {
                        digits.push(i as u32);
                    }
                }
            }
        }
    }
    digits
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(1, false);
    let mut sum_a: u32 = 0;
    let mut sum_b: u32 = 0;
    for row in input {
        let mut parsed: Vec<u32> = parse_digits(&row, false);
        sum_a += parsed[0] * 10 + parsed.last().unwrap();
        parsed = parse_digits(&row, true);
        sum_b += parsed[0] * 10 + parsed.last().unwrap();
    }
    println!("{}", sum_a);
    println!("{}", sum_b);
}
