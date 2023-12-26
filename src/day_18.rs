#![allow(dead_code)]

use advent_of_code_2023::*;
use itertools::Itertools; // 0.8.2
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().ok_or(())?;
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    steps: usize,
    colour: String,
}

impl Instruction {
    pub fn new(text: &String) -> Self {
        let split_text: Vec<&str> = text.split(" ").collect();
        let direction: Direction = split_text[0].parse::<Direction>().unwrap();
        let steps: usize = split_text[1].parse::<usize>().unwrap();
        let colour: String = split_text[2][1..8].to_string();
        Self {
            direction,
            steps,
            colour,
        }
    }
    pub fn new2(text: &String) -> Self {
        let split_text: Vec<&str> = text.split(" ").collect();
        let direction: Direction = match split_text[2][1..8].chars().nth(6).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Right,
            _ => Direction::Up,
        };

        let steps: usize = i64::from_str_radix(&split_text[2][1..8][1..6], 16).unwrap() as usize;
        let colour: String = " ".to_string();
        Self {
            direction,
            steps,
            colour,
        }
    }
}

#[derive(Default)]
struct Terrain {
    instructions: Vec<Instruction>,
    trench: HashMap<i32, Vec<i32>>,
    length: usize,
    width: usize,
    pos_i: i32,
    pos_j: i32,
}

impl Terrain {
    pub fn new(input: &Vec<String>) -> Self {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in input {
            let instruction = Instruction::new(line);
            instructions.push(instruction);
        }
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn new2(input: &Vec<String>) -> Self {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in input {
            let instruction = Instruction::new2(line);
            instructions.push(instruction);
        }
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn print(&self) {
        for (i, j_values) in self.trench.iter().sorted() {
            let mut j: i32 = 0;
            let mut found: usize = 0;
            while found < j_values.len() {
                if j_values.contains(&j) {
                    print!("#");
                    found += 1;
                } else {
                    print!(".");
                }
                j += 1;
            }
            println!(" {}", i);
        }
        println!();
    }

    pub fn dig(&mut self) {
        for instr in &self.instructions {
            match instr.direction {
                Direction::Down | Direction::Up => {
                    for _ in 0..instr.steps {
                        self.pos_i += if instr.direction == Direction::Down { 1 } else { -1 };
                        self.trench.entry(self.pos_i).or_insert(Vec::new()).push(self.pos_j);
                    }
                }
                Direction::Left | Direction::Right => {
                    for _ in 0..instr.steps {
                        self.pos_j += if instr.direction == Direction::Right { 1 } else { -1 };
                        self.trench.entry(self.pos_i).or_insert(Vec::new()).push(self.pos_j);
                    }
                }
            }
        }
        for (_, j_values) in &mut self.trench {
            j_values.sort();
        }
    }

    // trial 1, incorrect
    pub fn area_with_print(&self) -> usize {
        let mut total_area: usize = 0;
        for (_, j_values) in self.trench.iter().sorted() {
            let mut j: i32 = 0;
            let mut found: usize = 0;
            while found < j_values.len() {
                if j_values.contains(&j) {
                    print!("#");
                    found += 1;
                } else {
                    print!(".");
                }
                j += 1;
            }

            let mut prev_j: i32 = j_values[0];
            let mut inside: bool = true;
            let mut prev_jump: bool = false;
            print!(" ({})", j_values.len());
            total_area += j_values.len();
            for j in &j_values[1..] {
                if *j - prev_j > 1 {
                    if prev_jump {
                        inside = !inside;
                    }
                    if inside {
                        print!(" ({})", (*j - prev_j - 1));
                        total_area += (*j - prev_j - 1) as usize;
                        prev_jump = true;
                    }
                } else {
                    prev_jump = false;
                }
                prev_j = *j;
            }
            println!(" {}", total_area);
        }
        total_area
    }

    // trial 2, stack overflow (all variations)
    pub fn fill_recurse(&mut self, i: i32, j: i32, rev: bool) {
        println!("{} {}", i, j);
        let choices = if rev {
            vec![(0, 1), (0, -1)]
        } else {
            vec![(1, 0), (-1, 0)]
        };
        for (d_i, d_j) in choices {
            let mut m: i32 = 1;
            let mut new_i = i + m * d_i;
            let mut new_j = j + m * d_j;
            while self.trench.contains_key(&new_i) && !self.trench.get(&new_i).unwrap().contains(&new_j) {
                self.trench.entry(new_i).or_insert(Vec::new()).push(new_j);
                self.fill_recurse(new_i, new_j, !rev);
                m += 1;
                new_i = i + m * d_i;
                new_j = j + m * d_j;
            }
        }
    }

    // trial 3, incorrect
    pub fn fill(&mut self, (i, j): (i32, i32)) {
        let preference = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut i = i;
        let mut j = j;
        let mut d_i;
        let mut d_j;
        let mut arrow: i32 = 1;
        'outer: loop {
            self.trench.entry(i).or_insert(Vec::new()).push(j);
            for try_move in (arrow - 1)..(arrow + 3) {
                (d_i, d_j) = preference[((try_move + 4) % 4) as usize];
                if !self.trench.get(&(i + d_i)).unwrap().contains(&(j + d_j)) {
                    arrow = try_move;
                    i += d_i;
                    j += d_j;
                    continue 'outer;
                }
            }
            break;
        }
    }

    // trial 4: correct!
    pub fn fill_recurse_iter(&mut self, (i_start, j_start): (i32, i32)) {
        let mut queue: Vec<(i32, i32)> = Vec::new();
        queue.push((i_start, j_start));
        let mut i: i32;
        let mut j: i32;
        while queue.len() > 0 {
            (i, j) = queue.pop().unwrap();
            self.trench.entry(i).or_insert(Vec::new()).push(j);
            for (d_i, d_j) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_i = i + d_i;
                let new_j = j + d_j;
                if self.trench.contains_key(&new_i)
                    && !self.trench.get(&new_i).unwrap().contains(&new_j)
                    && !queue.contains(&(new_i, new_j))
                {
                    queue.push((new_i, new_j));
                }
            }
        }
    }

    pub fn total(&self) -> usize {
        self.trench.values().map(|x| x.len()).sum()
    }
}

pub fn main() {
    let example = false;
    let input = puzzle_input_aslines(18, example);

    let mut terrain = Terrain::new(&input);
    if example {
        for x in &terrain.instructions {
            println!("{:?} {}", x.direction, x.steps);
        }
    }
    terrain.dig();
    let start_node = if example { (1, 1) } else { (-84, 11) };
    terrain.fill_recurse_iter(start_node);
    println!("Area: {}", terrain.total());
    println!();

    let mut terrain = Terrain::new2(&input);
    if example {
        for x in &terrain.instructions {
            println!("{:?} {}", x.direction, x.steps);
        }
    }
    terrain.dig();
    let start_node = if example { (1, 1) } else { (-84, 11) };
    terrain.fill_recurse_iter(start_node);
    println!("Area: {}", terrain.total());
}
