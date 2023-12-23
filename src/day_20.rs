#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

use advent_of_code_2023::*;

enum Pulse {
    Low,
    High,
}

//
// Module
//

enum Module {
    Broadcaster{destinations: Vec<String>},
    FlipFlop {
        name: String,
        destinations: Vec<String>,
        is_on: bool,
    },
    Conjunction {
        name: String,
        destinations: Vec<String>,
        previous_pulses: Vec<Pulse>,
    },
}

impl Module {
    pub fn new(line: &String) -> Self {
        let mut module = match line.chars().nth(0).unwrap() {
            'b' => {},
            '%' => {},
            '&' => {},
            _ => None,
        };
    }
    
    pub fn pulse(&self, other_modules:&HashMap<String, Module>){}
}

fn read_modules(input: &Vec<String>) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in input {

    }
    modules
}

pub fn main() {
    let input = puzzle_input_aslines(20, true);
    
}
