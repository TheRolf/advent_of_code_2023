#![allow(dead_code, unused_variables, unused_mut)]

use std::{str::FromStr, collections::HashMap};
use itertools::Itertools; // 0.8.2
use advent_of_code_2023::*;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
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

struct Instruction{
    direction: Direction,
    steps: usize,
    colour: String, 
}

impl Instruction {
    pub fn new(text: String) -> Self {
        let split_text: Vec<&str> = text.split(" ").collect();
        let direction: Direction = split_text[0].parse::<Direction>().unwrap();
        let steps: usize = split_text[1].parse::<usize>().unwrap();
        let colour: String = split_text[2][1..8].to_string();
        Self { direction, steps, colour}
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
    pub fn new(input: Vec<String>) -> Self{
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in input {
            let instruction = Instruction::new(line);
            instructions.push(instruction);
        }
        Self {instructions, ..Default::default()}
    }

    pub fn dig(&mut self){
        for instr in &self.instructions {
            match instr.direction {
                Direction::Down | Direction::Up  => {
                    for i in 0..instr.steps {
                        self.pos_i += if instr.direction == Direction::Down {1} else {-1};
                        self.trench.entry(self.pos_i).or_insert(Vec::new()).push(self.pos_j);
                    }
                }
                Direction::Left | Direction::Right => {
                    for j in 0..instr.steps {
                        self.pos_j += if instr.direction == Direction::Right {1} else {-1};
                        self.trench.entry(self.pos_i).or_insert(Vec::new()).push(self.pos_j);
                    }
                }
            }
        }
        for (i, j_values) in &mut self.trench {
            j_values.sort();
        }
    }

    pub fn area(&self) -> usize {
        let mut total_area = 0;
        for (i, j_values) in self.trench.iter().sorted() {
            
        }
        total_area
    }
}

pub fn main() {
    let input = puzzle_input_aslines(18, true);
    let mut terrain = Terrain::new(input);
    terrain.dig();
    for (i, row) in terrain.trench.iter() {
        println!("{}: {:?}", i, row);
    }
}
