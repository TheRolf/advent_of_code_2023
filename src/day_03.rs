use std::collections::HashMap;

use advent_of_code_2023::*;

const N_ROW: usize = 140;
const N_COL: usize = 140;
struct PartNumber {
    value: u32,
    row_index: usize,
    col_indices: Vec<usize>,
}

impl PartNumber {
    fn bordering_cells(&self) -> Vec<(usize, usize)> {
        let mut cells: Vec<(usize, usize)> = Vec::new();

        let first_pos: usize = self.col_indices[0];
        if first_pos > 0 {
            cells.push((self.row_index, first_pos - 1))
        }

        let last_pos: usize = *self.col_indices.last().unwrap();
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
}

fn parse_part_number(digits: &Vec<char>, row: usize, column_after: usize) -> PartNumber {
    let last_col: usize = column_after - 1;
    let digits_str: String = digits.iter().collect();
    PartNumber {
        value: digits_str.parse::<u32>().unwrap(),
        row_index: row,
        col_indices: (last_col + 1 - digits.len()..=last_col).collect(),
    }
}

fn parse_part_numbers(input: &Vec<Vec<char>>) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut part_number_chars: Vec<char> = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                part_number_chars.push(*c);
            }
            if part_number_chars.len() > 0 && !c.is_digit(10) {
                part_numbers.push(parse_part_number(&part_number_chars, i, j));
                part_number_chars = Vec::new();
            }
        }
        if part_number_chars.len() > 0 {
            part_numbers.push(parse_part_number(&part_number_chars, i, N_COL));
            part_number_chars = Vec::new();
        }
    }
    part_numbers
}

pub fn main() {
    let input: Vec<Vec<char>> = puzzle_input_asarray(3, false);
    let part_numbers: Vec<PartNumber> = parse_part_numbers(&input);
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut sum: u32 = 0;
    let mut sum_gears: u32 = 0;
    for part_number in &part_numbers {
        for (i, j) in part_number.bordering_cells() {
            if input[i][j] != '.' {
                sum = sum + part_number.value;
            }
            if input[i][j] == '*' {
                gears
                    .entry((i, j))
                    .or_insert(Vec::new())
                    .push(part_number.value);
            }
        }
    }
    println!("{}", sum);

    for gear_part_numbers in gears.values() {
        if gear_part_numbers.len() == 2 {
            sum_gears += gear_part_numbers[0] * gear_part_numbers[1];
        }
    }
    println!("{}", sum_gears);
}
