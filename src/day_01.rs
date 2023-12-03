#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

const DIGITS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digits(row: &String) -> Vec<u16> {
    let mut digits: Vec<u16> = Vec::new();
    let len: usize = row.len();
    for j in 0..len {
        if row.chars().nth(j).unwrap().is_digit(10) {
            digits.push(row.chars().nth(j).unwrap().to_digit(10).unwrap() as u16)
        } else {
            for i in 0..10 {
                if DIGITS[i].len() <= len - j {
                    let splice: String = row
                        .chars()
                        .into_iter()
                        .skip(j)
                        .take(DIGITS[i].len())
                        .collect();
                    if splice == DIGITS[i] {
                        digits.push(i as u16);
                    }
                }
            }
        }
    }
    digits
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(1);
    let mut sum: u16 = 0;
    for row in input {
        let parsed = parse_digits(&row);
        println!("{:?}", parsed);
        sum += parsed.get(0).unwrap() * 10 + parsed.get(parsed.len() - 1).unwrap();
    }
    println!("{}", sum);
}
