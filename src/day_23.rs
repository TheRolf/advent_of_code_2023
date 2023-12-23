#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Walk {
    cells: Vec<Cell>,
}

struct Planner {
    map: Vec<Vec<char>>,
    length: usize,
    width: usize,
    walks: Vec<Walk>,
}

impl Planner {
    pub fn new(input: Vec<Vec<char>>) -> Self{
        Self { map: input.clone(), length: input.len(), width: input[0].len(), walks: Vec::new() }
    }

    pub fn plan(&mut self) {
        let mut walks: Vec<Walk> = Vec::new();
        let mut initial_walk = Walk{ cells: Vec::new() };
        initial_walk.cells.push(Cell{ x: 0, y: 1});
        walks.push(initial_walk);        
        while !walks.is_empty() {
            let mut current_walk = walks.pop().unwrap();
            for new_cell in self.next(current_walk.cells.last().unwrap(), &current_walk){
                let mut progressed_walk = current_walk.clone();
                progressed_walk.cells.push(new_cell.clone());
                if (new_cell == Cell{ x: self.length-1, y: self.width-2 }){
                    self.walks.push(progressed_walk);
                    println!("{}", self.longest_walk());
                } else {
                    walks.push(progressed_walk);
                }
            }
        }
    }   

    pub fn next(&self, current_cell: &Cell, current_walk: &Walk) -> Vec<Cell> {
        let mut next_cells = Vec::new();
        let mut x_d: i32;
        let mut y_d: i32;
        for direction in vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            (x_d, y_d) = match direction {
                Direction::Up => (-1, 0),
                Direction::Right => (0, 1),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
            };
            if (current_cell.x as i32) < -x_d || (current_cell.y as i32) < -y_d {
                continue;
            }
            let x_new = (current_cell.x as i32 + x_d) as usize;
            let y_new = (current_cell.y as i32 + y_d) as usize;
            if x_new > self.length-1 || y_new > self.width-1 {
                continue;
            }
            let candidate_cell = Cell{ x: x_new, y: y_new };
            let allowed_slope: char = match direction {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            };
            if ! &current_walk.cells.contains(&candidate_cell) &&
                // (self.map[x_new][y_new] == '.' || self.map[x_new][y_new] == allowed_slope){ // part a
                (self.map[x_new][y_new] != '#'){ // part b
                next_cells.push( candidate_cell );
            }
        }
        next_cells
    }

    pub fn longest_walk(&self) -> usize{
        let mut longest_walk: usize = 0; 
        for walk in &self.walks {
            if walk.cells.len()-1 > longest_walk {
                longest_walk = walk.cells.len()-1;
            }
        }
        longest_walk
    }

}

pub fn main() {
    let input = puzzle_input_asarray(23, false);
    let mut planner = Planner::new(input);
    planner.plan();
    
    println!("{}", planner.longest_walk())
}
