#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

#[derive(Clone)]
struct Pattern {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Pattern {
    pub fn new() -> Self {
        Self { cells: Vec::new(), width: 0, height: 0 }
    }

    pub fn print(&self) {
        for row in &self.cells {
            for c in row {
                print!("{} ", c);
            }
            println!();
        }
    }

    pub fn row_equal(&self, row_1: usize, row_2: usize) -> bool {
        for j in 0..self.width {
            if self.cells[row_1][j] != self.cells[row_2][j] {
                return false;
            }
        }
        true
    }

    pub fn col_equal(&self, col_1: usize, col_2: usize) -> bool {
        for i in 0..self.height {
            if self.cells[i][col_1] != self.cells[i][col_2] {
                return false;
            }
        }
        true
    }


    pub fn mirror_at_row(&self, row_index: usize) -> bool {
        let mut i: usize = 0;
        loop {
            let row_1: i32 = (row_index as i32 - i as i32 - 1) as i32;
            let row_2 = row_index + i;
            if row_1 < 0 || row_2 > self.height - 1 {
                break;
            }
            if !self.row_equal(row_1 as usize, row_2) {
                return false;
            }
            i += 1;
        }
        true
    }


    pub fn mirror_at_col(&self, col_index: usize) -> bool {
        let mut j: usize = 0;
        loop {
            let col_1: i32 = (col_index as i32 - j as i32 - 1) as i32;
            let col_2 = col_index + j;
            if col_1 < 0 || col_2 > self.width - 1 {
                break;
            }
            if !self.col_equal(col_1 as usize, col_2) {
                return false;
            }
            j += 1;
        }
        true
    }

    pub fn lines_before_mirror(&self, original_value: i32) -> usize {
        let mut reflection_number: usize = 0;
        for row_index in 1..self.height {
            if original_value < 0 || row_index != (original_value/100) as usize {
                if self.mirror_at_row(row_index) {
                    return 100*row_index;
                }
            }
        }
        for col_index in 1..self.width {
            if original_value < 0 || col_index != original_value as usize {
                if self.mirror_at_col(col_index){
                    return col_index;
                }
            }
        } 
        return reflection_number;
    } 

    pub fn smudge(&self, row: usize, col: usize) -> Self {
        let mut smudged_pattern: Pattern = self.clone();
        if smudged_pattern.cells[row][col] == '.' {
            smudged_pattern.cells[row][col] = '#';
        } else if smudged_pattern.cells[row][col] == '#' {
            smudged_pattern.cells[row][col] = '.';
        }
        smudged_pattern
    }

    pub fn fix_smudge(&self) -> usize {
        let original_value: usize = self.lines_before_mirror(-1);
        for row in 0..self.height {
            for col in 0..self.width {
                let value = self.smudge(row, col).lines_before_mirror(original_value as i32);
                if value != original_value && value > 0 {
                    return value;
                }
            }
        }
        0
    }

}



pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(13, false);
    let mut valley: Vec<Pattern> = Vec::new();
    let mut pattern: Pattern = Pattern::new();
    let mut sum_a: usize = 0;
    let mut sum_b: usize = 0;
    for line in input {
        if line.len() == 0 {
            valley.push(pattern);
            pattern = Pattern::new();
        } else {
            pattern.cells.push(line.chars().collect());
            pattern.width = line.len();
            pattern.height += 1;
        }
    }
    if pattern.height > 0 {
        valley.push(pattern);
    } 

    for (i, pattern) in valley.iter().enumerate() {
        let value_a: usize = pattern.lines_before_mirror(-1);
        let value_b: usize = pattern.fix_smudge();
        sum_a += value_a;
        sum_b += value_b;
    }
    println!("{}", sum_a);
    println!("{}", sum_b);

}
