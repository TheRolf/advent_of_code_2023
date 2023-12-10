#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

use advent_of_code_2023::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

struct PipeMaze {
    diagram: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    inner: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    current: (usize, usize),
    movement: HashMap<Move, (i16, i16)>,
    allowed_movements_to: HashMap<char, Vec<Move>>,
    allowed_movements_from: HashMap<Move, Vec<char>>,
}

impl PipeMaze {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        let width: usize = input[0].len();
        let height: usize = input.len();
        let mut visited: Vec<Vec<bool>> = Vec::new();
        let mut inner: Vec<Vec<bool>> = Vec::new();
        let mut current: (usize, usize) = (0, 0);
        for i in 0..height {
            visited.push(Vec::new());
            inner.push(Vec::new());
            for j in 0..width {
                inner[i].push(false);
                if input[i][j] == 'S' {
                    current = (i, j);
                    visited[i].push(true);
                } else {
                    visited[i].push(false);
                }
            }
        }
        let mut movement: HashMap<Move, (i16, i16)> = HashMap::new();
        movement.insert(Move::Left, (0, -1));
        movement.insert(Move::Right, (0, 1));
        movement.insert(Move::Up, (-1, 0));
        movement.insert(Move::Down, (1, 0));

        let mut allowed_movements_to: HashMap<char, Vec<Move>> = HashMap::new();
        allowed_movements_to.insert('-', [Move::Left, Move::Right].to_vec());
        allowed_movements_to.insert('|', [Move::Up, Move::Down].to_vec());
        allowed_movements_to.insert('F', [Move::Right, Move::Down].to_vec());
        allowed_movements_to.insert('7', [Move::Left, Move::Down].to_vec());
        allowed_movements_to.insert('L', [Move::Right, Move::Up].to_vec());
        allowed_movements_to.insert('J', [Move::Left, Move::Up].to_vec());
        allowed_movements_to.insert('S', [Move::Left, Move::Right, Move::Down, Move::Up].to_vec());

        let mut allowed_movements_from: HashMap<Move, Vec<char>> = HashMap::new();
        allowed_movements_from.insert(Move::Left, ['-', 'F', 'L', 'S'].to_vec());
        allowed_movements_from.insert(Move::Right, ['-', '7', 'J', 'S'].to_vec());
        allowed_movements_from.insert(Move::Up, ['|', 'F', '7', 'S'].to_vec());
        allowed_movements_from.insert(Move::Down, ['|', 'L', 'J', 'S'].to_vec());

        PipeMaze {
            diagram: input,
            visited,
            inner,
            width,
            height,
            current,
            movement,
            allowed_movements_to,
            allowed_movements_from,
        }
    }

    pub fn get_current(&self) -> char {
        let (i, j) = self.current;
        return self.diagram[i][j];
    }

    pub fn step_possible(&self, arrival: &Move, i: i16, j: i16) -> bool {
        if i < 0 || i >= self.height as i16 || j < 0 || j >= self.width as i16 {
            return false;
        }
        if self.visited[i as usize][j as usize] {
            return false;
        }
        let value: &char = &self.diagram[i as usize][j as usize];
        if self.allowed_movements_from.get(arrival).unwrap().contains(value) {
            return true;
        }
        false
    }

    pub fn step(&mut self) -> bool {
        let current_cell: char = self.get_current();
        let i = self.current.0 as i16;
        let j = self.current.1 as i16;
        for direction in self.allowed_movements_to.get(&current_cell).unwrap() {
            let (i_delta, j_delta) = self.movement.get(direction).unwrap();
            if self.step_possible(direction, i + i_delta, j + j_delta) {
                let i_new = (i + i_delta) as usize;
                let j_new = (j + j_delta) as usize;
                self.current = (i_new, j_new);
                self.visited[i_new][j_new] = true;
                return true;
            }
        }
        false
    }
}

pub fn main() {
    let puzzle_input = puzzle_input_asarray(10, false);
    let mut pipe_maze: PipeMaze = PipeMaze::new(puzzle_input);
    let mut step_count = 1;
    while pipe_maze.step() {
        step_count += 1;
    }
    println!("{}", step_count / 2);
}
