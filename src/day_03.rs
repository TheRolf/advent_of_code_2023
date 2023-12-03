#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;
use std::collections::HashMap;

const N_ROW: u8 = 140;
const N_COL: u8 = 140;
struct PartNumber {
    value: u32,
    row_index: u8,
    col_indices: Vec<u8>,
}

impl PartNumber {
    fn bordering_cells(&self) -> Vec<(u8, u8)> {
        let mut cells: Vec<(u8, u8)> = Vec::new();

        let first_pos: u8 = self.col_indices[0];
        if first_pos > 0 {
            cells.push((self.row_index, first_pos - 1))
        }

        let last_pos: u8 = self.col_indices[self.col_indices.len() - 1];
        if last_pos < N_COL - 1 {
            cells.push((self.row_index, last_pos + 1))
        }

        if self.row_index > 0 {
            for j in first_pos.max(1) - 1..(last_pos + 2).min(N_COL - 1) {
                cells.push((self.row_index - 1, j));
            }
        }

        if self.row_index < N_ROW - 1 {
            for j in first_pos.max(1) - 1..(last_pos + 2).min(N_COL - 1) {
                cells.push((self.row_index + 1, j));
            }
        }
        cells
    }

    fn print(&self) {
        println!(
            "{}, {}, {:?}\n\t{:?}",
            self.value,
            self.row_index,
            self.col_indices,
            self.bordering_cells()
        );
    }
}

fn parse_part_number(digits: &Vec<char>, row: u8, column_after: usize) -> PartNumber {
    let last_col: usize;
    let row_index: u8;
    if column_after == 0 {
        row_index = row - 1;
        last_col = (N_COL - 1) as usize;
    } else {
        row_index = row;
        last_col = column_after - 1;
    }
    let digits_str: String = digits.iter().collect();
    PartNumber {
        value: digits_str.parse::<u32>().unwrap(),
        row_index: row_index,
        col_indices: (last_col + 1 - digits.len()..=last_col)
            .map(|x: usize| x as u8)
            .collect(),
    }
}

fn parse_part_numbers(input: &Vec<Vec<char>>) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut i: u8 = 0;
    let mut j: u8;
    let mut part_number_chars: Vec<char> = Vec::new();
    for row in input {
        j = 0;
        for c in row {
            if c.is_digit(10) {
                part_number_chars.push(*c);
            }
            if part_number_chars.len() > 0 && !c.is_digit(10) {
                part_numbers.push(parse_part_number(&part_number_chars, i, j as usize));
                part_number_chars = Vec::new();
            }
            j = j + 1;
        }
        if part_number_chars.len() > 0 {
            part_numbers.push(parse_part_number(&part_number_chars, i, j as usize));
            part_number_chars = Vec::new();
        }
        i = i + 1;
    }
    part_numbers
}

pub fn main() {
    let input = puzzle_input_asarray(3);
    let part_numbers = parse_part_numbers(&input);
    let mut gears: HashMap<(u8, u8), Vec<u32>> = HashMap::new();
    let mut sum: u32 = 0;
    for part_number in &part_numbers {
        for (i, j) in part_number.bordering_cells() {
            if get_char(&input, i, j) != '.' {
                sum = sum + part_number.value;
                break;
            }
        }
    }
    println!("{}", sum);

    for part_number in &part_numbers {
        for (i, j) in part_number.bordering_cells() {
            if get_char(&input, i, j) == '*' {
                gears
                    .entry((i, j).clone())
                    .or_insert(Vec::new())
                    .push(part_number.value);
            }
        }
    }
    let mut sum_gears: u32 = 0;
    for (key, value) in &gears {
        if value.len() == 2 {
            sum_gears = sum_gears + (value.get(0).unwrap() * value.get(1).unwrap());
        }
    }
    println!("{}", sum_gears);
}
