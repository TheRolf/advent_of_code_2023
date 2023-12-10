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
    boundary: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    current: (usize, usize),
    movement: HashMap<Move, (i16, i16)>,
    allowed_movements_to: HashMap<char, Vec<Move>>,
    allowed_movements_from: HashMap<Move, Vec<char>>,
}

impl PipeMaze {
    pub fn new(input: &Vec<Vec<char>>, boundary: Vec<Vec<bool>>) -> Self {
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
        allowed_movements_to.insert('S', [Move::Up, Move::Down, Move::Left, Move::Right].to_vec());

        let mut allowed_movements_from: HashMap<Move, Vec<char>> = HashMap::new();
        allowed_movements_from.insert(Move::Left, ['-', 'F', 'L', 'S'].to_vec());
        allowed_movements_from.insert(Move::Right, ['-', '7', 'J', 'S'].to_vec());
        allowed_movements_from.insert(Move::Up, ['|', 'F', '7', 'S'].to_vec());
        allowed_movements_from.insert(Move::Down, ['|', 'L', 'J', 'S'].to_vec());

        PipeMaze {
            diagram: input.clone(),
            visited,
            inner,
            boundary,
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

    pub fn step(&mut self, with_fill: bool) -> bool {
        let current_cell: char = self.get_current();
        let mut i: i16 = self.current.0 as i16;
        let mut j: i16 = self.current.1 as i16;
        for direction in self.allowed_movements_to.get(&current_cell).unwrap() {
            let (i_delta, j_delta) = self.movement.get(direction).unwrap();
            if self.step_possible(direction, i + i_delta, j + j_delta) {
                let i_new = (i + i_delta) as usize;
                let j_new = (j + j_delta) as usize;
                self.current = (i_new, j_new);
                self.visited[i_new][j_new] = true;
                if with_fill {
                    i = i_new as i16;
                    j = j_new as i16;
                    if direction == &Move::Right {
                        self.fill_recurse(i + 1, j - 1);
                        self.fill_recurse(i + 1, j);
                    } else if direction == &Move::Down {
                        self.fill_recurse(i - 1, j - 1);
                        self.fill_recurse(i, j - 1);
                    } else if direction == &Move::Left {
                        self.fill_recurse(i - 1, j + 1);
                        self.fill_recurse(i - 1, j);
                    } else if direction == &Move::Up {
                        self.fill_recurse(i + 1, j + 1);
                        self.fill_recurse(i, j + 1);
                    }
                }
                return true;
            }
        }
        false
    }

    pub fn fill_recurse(&mut self, i: i16, j: i16) {
        if i < 0 || i >= self.height as i16 || j < 0 || j >= self.width as i16 {
            return;
        }

        if self.boundary[i as usize][j as usize] || self.inner[i as usize][j as usize] {
            return;
        }
        self.inner[i as usize][j as usize] = true;
        self.fill_recurse(i + 1, j);
        self.fill_recurse(i - 1, j);
        self.fill_recurse(i, j + 1);
        self.fill_recurse(i, j - 1);
    }

    pub fn show_inner(&self) {
        let mut pipe_map: HashMap<char, char> = HashMap::new();
        pipe_map.insert('F', '┌');
        pipe_map.insert('7', '┐');
        pipe_map.insert('J', '┘');
        pipe_map.insert('L', '└');
        for i in 0..self.height {
            for j in 0..self.width {
                print!(
                    "{}",
                    if self.inner[i][j] {
                        '*'
                    } else if self.visited[i][j] {
                        if ['|', '-', 'S'].contains(&self.diagram[i][j]) {
                            self.diagram[i][j]
                        } else {
                            *pipe_map.get(&self.diagram[i][j]).unwrap()
                        }
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    pub fn get_total_inner(&self) -> u16 {
        let mut count: u16 = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.inner[i][j] {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn main() {
    let puzzle_input: Vec<Vec<char>> = puzzle_input_asarray(10, false);
    let mut pipe_maze: PipeMaze = PipeMaze::new(&puzzle_input, Vec::new());
    let mut step_count = 1;
    while pipe_maze.step(false) {
        step_count += 1;
    }
    println!("{}", step_count / 2);

    println!();
    let boundary: Vec<Vec<bool>> = pipe_maze.visited;
    pipe_maze = PipeMaze::new(&puzzle_input, boundary);
    while pipe_maze.step(true) {}
    pipe_maze.show_inner();
    println!("{}", pipe_maze.get_total_inner());
}
