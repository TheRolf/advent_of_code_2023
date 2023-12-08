
#![allow(dead_code, unused_variables, unused_mut)]

use num_integer::lcm;
use std::collections::HashMap;
use advent_of_code_2023::*;

pub fn lcm_vec(numbers: Vec<usize>) -> usize {
    numbers.iter().fold(1, |acc, &x| lcm(acc, x))
}
struct Step {
    left: String,
    right: String,
}

impl Step {
    pub fn new(left: String, right: String) -> Self {
        Step { left, right }
    }
}

struct Network {
    current: String,
    currents: Vec<String>,
    nodes: HashMap<String, Step>,
}

impl Network {
    pub fn new() -> Self {
        Network { current: "JMA".to_string(), currents: Vec::new(), nodes: HashMap::new() }
    }

    pub fn add(&mut self, line: String){
        let result: Vec<&str> = line.split(|c| ['(', ')', ' ', ',', '='].contains(&c)).filter(|s| !s.is_empty()).collect();
        let node: Step = Step::new(result.get(1).unwrap().to_string(), result.get(2).unwrap().to_string());
        self.nodes.insert(result.get(0).unwrap().to_string(), node);
        if result.get(0).unwrap().chars().nth(2).unwrap() == 'A' {
            self.currents.push(result.get(0).unwrap().to_string());
        }
    }

    pub fn step(&mut self, index: usize, step_type: char) {
        let current: String = self.currents[index].clone();
        if step_type == 'L' { 
            self.currents[index] = self.nodes.get(&current).map(|own_type| own_type.left.clone()).unwrap();
        }
        if step_type == 'R' { 
            self.currents[index] = self.nodes.get(&current).map(|own_type| own_type.right.clone()).unwrap();
        }    
    }

    pub fn is_done(&self, index: usize) -> bool {
        self.currents[index].chars().nth(2).unwrap() == 'Z'
    }

}

pub fn main() {
    // let instructions = "LR";
    let instructions = "LRLRLLRRLRRRLRLRRLRLLRRLRRRLRLRLRLRRLRLLRRRLRRRLLRRLRRLRLRRRLLLRRLRLRLRLRLRLLRRRLRLRRRLRRRLRRRLRRRLRRRLRRRLRRRLRRLRRRLLRLLRRLRRLRRLRRRLLRLRRLRLRLRRLLRLRRRLRRLLRLRLRRRLRRLRRLRRLRLLRLRRRLLLRRRLLLLRRLRRRLLLRRLLRLRLRLLLRRRLLRRRLLLRLRRLLRRRLRRRLRLLRRRLRLRLRLLRRLLRRLRRRLRLRRRLRRLRLRRLRRRR";

    let input: Vec<String> = puzzle_input_aslines(8);
    let mut network: Network = Network::new();
    for line in input {
        network.add(line);
    }

    println!("{:?}", network.currents);
    
    let mut step_counts: Vec<usize> = Vec::new();
    for index in 0..network.currents.len(){
        let mut step_count: usize = 0;
        let mut step_type: char;
        let start_node = network.currents[index].clone();
        while !network.is_done(index) {
            step_type = instructions.chars().nth(step_count % instructions.len()).unwrap();
            network.step(index, step_type);
            step_count += 1;
        }
        println!("Steps taken from {} to {}: {}", start_node, network.currents[index], step_count);
        step_counts.push(step_count);
    }
    println!("LCM of step counts is: {}", lcm_vec(step_counts))
}