use advent_of_code_2023::*;

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Platform {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
    history: Vec<usize>,
    shift: usize,
    base: Vec<usize>,
}

impl Platform {
    pub fn new(input: &Vec<Vec<char>>) -> Self {
        let height: usize = input.len();
        let width: usize = input[0].len();
        Self {
            cells: input.clone(),
            height,
            width,
            history: Vec::new(),
            shift: 0,
            base: Vec::new(),
        }
    }

    pub fn tilt(&mut self, direction: Direction) {
        let first_range: Vec<usize> = match direction {
            Direction::North => (1..self.height).collect(),
            Direction::South => (0..self.height - 1).rev().collect(),
            Direction::West => (1..self.width).collect(),
            Direction::East => (0..self.width - 1).rev().collect(),
        };
        let second_range: Vec<usize> = match direction {
            Direction::North | Direction::South => (0..self.width).collect(),
            Direction::West | Direction::East => (0..self.height).collect(),
        };
        for i in first_range {
            for j in second_range.clone() {
                if (vec![Direction::North, Direction::South].contains(&direction)
                    && self.cells[i][j] == 'O')
                    || (vec![Direction::West, Direction::East].contains(&direction)
                        && self.cells[j][i] == 'O')
                {
                    match direction {
                        Direction::North => {
                            for i_new in (-1..i as i32).rev() {
                                if i_new < 0 || self.cells[i_new as usize][j] != '.' {
                                    self.cells[i][j] = '.';
                                    self.cells[(i_new + 1) as usize][j] = 'O';
                                    break;
                                }
                            }
                        }
                        Direction::South => {
                            for i_new in i + 1..self.height + 1 {
                                if i_new == self.height || self.cells[i_new][j] != '.' {
                                    self.cells[i][j] = '.';
                                    self.cells[(i_new - 1) as usize][j] = 'O';
                                    break;
                                }
                            }
                        }
                        Direction::West => {
                            for i_new in (-1..i as i32).rev() {
                                if i_new < 0 || self.cells[j][i_new as usize] != '.' {
                                    self.cells[j][i] = '.';
                                    self.cells[j][(i_new + 1) as usize] = 'O';
                                    break;
                                }
                            }
                        }
                        Direction::East => {
                            for i_new in i + 1..self.width + 1 {
                                if i_new == self.width || self.cells[j][i_new] != '.' {
                                    self.cells[j][i] = '.';
                                    self.cells[j][(i_new - 1) as usize] = 'O';
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn cycle(&mut self, number_of_cycles: usize, minimum_base_length: usize) {
        for i in 0..number_of_cycles {
            self.tilt(Direction::North);
            self.tilt(Direction::West);
            self.tilt(Direction::South);
            self.tilt(Direction::East);
            self.history.push(self.total_load());
            if self.history.len() >= minimum_base_length * 2 && self.repetition() {
                println!("Fixed point reached after {} cycles.", i);
                println!("{} {:?}.", self.shift, self.base);
                let result = self.base[(number_of_cycles - self.shift - 1) % self.base.len()];
                println!("Load after {} cycles would be {}", number_of_cycles, result);
                return;
            }
        }
    }

    pub fn repetition(&mut self) -> bool {
        for left_start in 0..self.history.len() / 2 {
            let right_start: usize = left_start + (self.history.len() - left_start) / 2;
            let mut i: usize = 0;
            while left_start + i < right_start {
                if self.history[left_start + i] != self.history[right_start + i] {
                    break;
                }
                i += 1;
            }
            if left_start + i == right_start {
                self.shift = left_start;
                self.base = self.history[left_start..right_start].to_vec();
                return true;
            }
        }
        false
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
    let input: Vec<Vec<char>> = puzzle_input_asarray(14, false);
    let mut platform: Platform = Platform::new(&input);
    platform.tilt(Direction::North);
    println!("{}", platform.total_load());

    platform = Platform::new(&input);
    platform.cycle(1000000000, 5);
}
