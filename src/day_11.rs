#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

struct Universe {
    galaxies: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn new(input: &Vec<Vec<char>>) -> Self {
        let mut galaxies: Vec<(usize, usize)> = Vec::new();
        let height = input.len();
        let width = input[0].len();
        for i in 0..height {
            for j in 0..width {
                if input[i][j] == '#' {
                    galaxies.push((i, j));
                }
            }
        }
        Universe {
            galaxies,
            width,
            height,
        }
    }

    pub fn expand(&mut self) {
        let mut i: usize = 0;
        while i < self.height {
            if self.galaxies_in_row(i) == 0 {
                self.shift_from_row(i);
                self.height += 999999;
                i += 999999;
            }
            i += 1;
        }
        let mut j: usize = 0;
        while j < self.width {
            if self.galaxies_in_col(j) == 0 {
                self.shift_from_col(j);
                self.width += 999999;
                j += 999999;
            }
            j += 1;
        }
    }

    pub fn shift_from_row(&mut self, i: usize) {
        for mut galaxy in &mut self.galaxies {
            if galaxy.0 > i {
                galaxy.0 += 999999;
            }
        }
    }

    pub fn shift_from_col(&mut self, j: usize) {
        for mut galaxy in &mut self.galaxies {
            if galaxy.1 > j {
                galaxy.1 += 999999;
            }
        }
    }

    pub fn galaxies_in_row(&self, i: usize) -> usize {
        let mut count: usize = 0;
        for galaxy in &self.galaxies {
            if galaxy.0 == i {
                count += 1;
            }
        }
        count
    }

    pub fn galaxies_in_col(&self, j: usize) -> usize {
        let mut count: usize = 0;
        for galaxy in &self.galaxies {
            if galaxy.1 == j {
                count += 1;
            }
        }
        count
    }

    pub fn sum_of_distances(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.galaxies.len() {
            for j in 0..i {
                sum += self.distance(i, j);
            }
        }
        sum
    }

    pub fn distance(&self, i: usize, j: usize) -> usize {
        ((self.galaxies[i].0 as i32 - self.galaxies[j].0 as i32).abs()
            + (self.galaxies[i].1 as i32 - self.galaxies[j].1 as i32).abs()) as usize
    }
}

pub fn main() {
    let input: Vec<Vec<char>> = puzzle_input_asarray(11, false);
    let mut universe = Universe::new(&input);
    // println!("{:?}", universe.galaxies);
    universe.expand();
    // println!("{:?}", universe.galaxies);
    println!("{}", universe.sum_of_distances());
}
