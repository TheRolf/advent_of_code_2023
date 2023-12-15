#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

pub fn tilt(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 1..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 'O' {
                for i_new in (0..i).rev() {
                    if input[i_new][j] != '.' {
                        input[i][j] = '.';
                        input[i_new + 1][j] = 'Ö';
                        break;
                    }
                }
                if input[i][j] == 'O' {
                    input[0][j] = 'Ö';
                    input[i][j] = '.';
                }
            }
        }
    }
    input
}

pub fn total_load(input: &Vec<Vec<char>>) -> usize {
    let mut total: usize = 0;
    for (i, row) in input.iter().enumerate() {
        for c in row {
            if *c == 'O' || *c == 'Ö' {
                total += input.len() - i;
            }
        }
    }
    total
}

pub fn main() {
    let mut input: Vec<Vec<char>> = puzzle_input_asarray(14, false);
    print_2d_array(&input);
    input = tilt(input);
    print_2d_array(&input);
    println!("{}", total_load(&input));
}
