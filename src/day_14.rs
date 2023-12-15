#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

struct Platform {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Platform {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        let height: usize = input.len();
        let width: usize = input[0].len();
        Self {
            cells: input,
            height,
            width,
        }
    }

    pub fn tilt(&mut self) {
        for i in 1..self.height {
            for j in 0..self.width {
                if self.cells[i][j] == 'O' {
                    let mut moved: bool = false;
                    for i_new in (0..i).rev() {
                        if self.cells[i_new][j] != '.' {
                            self.cells[i][j] = '.';
                            self.cells[i_new + 1][j] = 'O';
                            moved = true;
                            break;
                        }
                    }
                    if moved == false {
                        self.cells[0][j] = 'O';
                        self.cells[i][j] = '.';
                    }
                }
            }
        }
    }

    pub fn total_load(&self) -> usize {
        let mut total: usize = 0;
        for (i, row) in self.cells.iter().enumerate() {
            for c in row {
                if *c == 'O' {
                    total += self.height - i;
                }
            }
        }
        total
    }
}

pub fn main() {
    let mut input: Vec<Vec<char>> = puzzle_input_asarray(14, true);
    let mut platform: Platform = Platform::new(input);
    print_2d_array(&platform.cells);
    platform.tilt();
    print_2d_array(&platform.cells);
    println!("{}", platform.total_load());
}
