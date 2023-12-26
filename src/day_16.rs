use advent_of_code_2023::*;

#[derive(Clone)]
struct Layout {
    cells: Vec<Vec<char>>,
    energised: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Layout {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        let height: usize = input.len();
        let width: usize = input[0].len();
        let cells: Vec<Vec<char>> = input.clone();
        let energised: Vec<Vec<char>> = vec![vec!['.'; width]; height];
        Self {
            cells,
            energised,
            height,
            width,
        }
    }

    pub fn simulate(&mut self, pos_i: i32, pos_j: i32, dir_i: i32, dir_j: i32) {
        if 0 <= pos_i && (pos_i as usize) < self.height && 0 <= pos_j && (pos_j as usize) < self.width {
            let mut i: i32 = pos_i as i32;
            let mut j: i32 = pos_j as i32;
            match self.cells[i as usize][j as usize] {
                '|' => {
                    if self.energised[i as usize][j as usize] != '#' {
                        self.energise(i as usize, j as usize);
                        if dir_j != 0 {
                            self.simulate(i - 1, j, -1, 0);
                            self.simulate(i + 1, j, 1, 0);
                        } else {
                            self.simulate(i + dir_i, j + dir_j, dir_i, dir_j);
                        }
                    }
                }
                '-' => {
                    if self.energised[i as usize][j as usize] != '#' {
                        self.energise(i as usize, j as usize);
                        if dir_i != 0 {
                            self.simulate(i, j - 1, 0, -1);
                            self.simulate(i, j + 1, 0, 1);
                        } else {
                            self.simulate(i + dir_i, j + dir_j, dir_i, dir_j);
                        }
                    }
                }
                '/' => {
                    self.energise(i as usize, j as usize);
                    if dir_i == 0 {
                        self.simulate(i - dir_j, j, -dir_j, 0);
                    } else {
                        self.simulate(i, j - dir_i, 0, -dir_i);
                    }
                }
                '\\' => {
                    self.energise(i as usize, j as usize);
                    if dir_i == 0 {
                        self.simulate(i + dir_j, j, dir_j, -0);
                    } else {
                        self.simulate(i, j + dir_i, 0, dir_i);
                    }
                }
                '.' => {
                    self.energise(i as usize, j as usize);
                    loop {
                        i += dir_i as i32;
                        j += dir_j as i32;
                        if 0 > i || (i as usize) >= self.height || 0 > j || (j as usize) >= self.width {
                            return;
                        } else if self.cells[i as usize][j as usize] == '.' {
                            self.energise(i as usize, j as usize);
                        } else if self.cells[i as usize][j as usize] != '.' {
                            self.simulate(i, j, dir_i, dir_j);
                            return;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn energise(&mut self, i: usize, j: usize) {
        self.energised[i][j] = '#';
    }

    pub fn energised_count(&self) -> usize {
        let mut count: usize = 0;
        for row in &self.energised {
            for cell in row {
                if *cell == '#' {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn main() {
    let input: Vec<Vec<char>> = puzzle_input_asarray(16, false);
    let layout: Layout = Layout::new(input);
    let mut most_energised: usize = 0;

    let mut layout_copy = layout.clone();
    layout_copy.simulate(0, 0, 0, 1);
    println!("{}", layout_copy.energised_count());

    for i in 0..layout.height {
        let mut layout_copy_1: Layout = layout.clone();
        layout_copy_1.simulate(i as i32, 0, 0, 1);
        most_energised = most_energised.max(layout_copy_1.energised_count());
        let mut layout_copy_2: Layout = layout.clone();
        layout_copy_2.simulate(i as i32, layout.width as i32 - 1, 0, -1);
        most_energised = most_energised.max(layout_copy_2.energised_count());
    }
    for j in 0..layout.width {
        let mut layout_copy_1: Layout = layout.clone();
        layout_copy_1.simulate(0, j as i32, 1, 0);
        most_energised = most_energised.max(layout_copy_1.energised_count());
        let mut layout_copy_2: Layout = layout.clone();
        layout_copy_2.simulate(layout.height as i32 - 1, j as i32, -1, 0);
        most_energised = most_energised.max(layout_copy_2.energised_count());
    }

    println!("{}", most_energised);
}
