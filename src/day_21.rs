use std::collections::{BTreeSet, HashSet};

use advent_of_code_2023::*;

pub fn modulo(dividend: i32, divisor: i32) -> usize {
    (((dividend % divisor) + divisor) % divisor) as usize
}
struct Garden {
    map: Vec<Vec<char>>,
    length: usize,
    width: usize,
    start: (usize, usize),
    visited: HashSet<(usize, usize)>,
}

impl Garden {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        let length = input.len();
        let width = input[0].len();
        let mut start: (usize, usize) = (0, 0);
        for i in 0..length {
            for j in 0..width {
                if input[i][j] == 'S' {
                    start = (i, j);
                }
            }
        }
        Self {
            map: input,
            length,
            width,
            start,
            visited: HashSet::new(),
        }
    }

    pub fn reach_from_start(&mut self, step_count: usize, brute_force: bool) {
        if brute_force {
            self.reach_brute_force(self.start.0 as i32, self.start.1 as i32, step_count)
        } else {
            self.reach(self.start.0 as i32, self.start.1 as i32, step_count)
        }
    }

    pub fn reach_brute_force(&mut self, i: i32, j: i32, step_count: usize) {
        if step_count == 0 {
            self.visited.insert((i as usize, j as usize));
        } else {
            for (i_diff, j_diff) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let i_new = i + i_diff;
                let j_new = j + j_diff;
                if self.map[i_new as usize][j_new as usize] != '#' {
                    self.reach_brute_force(i_new, j_new, step_count - 1);
                }
            }
        }
    }

    pub fn reach(&mut self, i: i32, j: i32, step_count: usize) {
        let mut queue: BTreeSet<(i32, i32, usize)> = BTreeSet::new();
        let mut history: HashSet<(i32, i32, usize)> = HashSet::new();
        queue.insert((i, j, step_count));
        let mut i: i32;
        let mut j: i32;
        let mut step_count: usize;
        while !queue.is_empty() {
            (i, j, step_count) = queue.pop_last().unwrap();
            // println!("{} {} {}", i, j, step_count);
            if step_count == 0 {
                self.visited.insert((i as usize, j as usize));
            } else {
                for (i_diff, j_diff) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    let i_new = i + i_diff;
                    let j_new = j + j_diff;
                    if !history.contains(&(i_new, j_new, step_count - 1))
                        && self.map[modulo(i_new, self.length as i32)][modulo(j_new, self.width as i32)] != '#'
                    {
                        queue.insert((i_new, j_new, step_count - 1));
                        history.insert((i_new, j_new, step_count - 1));
                    }
                }
            }
        }
    }

    pub fn show_visited(&self) {
        for i in 0..self.length {
            for j in 0..self.width {
                if self.visited.contains(&(i, j)) {
                    print!("O");
                } else {
                    print!("{}", self.map[i][j])
                }
            }
            println!();
        }
    }
}

pub fn main() {
    let example = false;
    let input = puzzle_input_asarray(21, example);
    let mut garden: Garden = Garden::new(input);
    garden.reach_from_start(if example { 6 } else { 64 }, false);
    garden.show_visited();
    println!("{}", garden.visited.len());
    garden.visited = HashSet::new();

    garden.reach_from_start(if example { 1000 } else { 26501365 }, false);
    println!("{}", garden.visited.len());
}
