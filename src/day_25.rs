#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::{HashMap, HashSet};

use advent_of_code_2023::*;

#[derive(Debug)]
struct Graph {
    nodes: HashSet<String>,
    adjacent_nodes: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new(input: &Vec<String>) -> Self {
        let mut nodes = HashSet::new();
        let mut adjacent_nodes = HashMap::new();

        for line in input {
            let split_line: Vec<&str> = line.split(": ").collect();
            let root = split_line[0].to_string();
            let adjacents: Vec<&str> = split_line[1].split(" ").collect();
            nodes.insert(root.clone());
            for conn in adjacents {
                nodes.insert(conn.to_string());
                adjacent_nodes.entry(root.to_string()).or_insert(HashSet::new()).insert(conn.to_string());
                adjacent_nodes.entry(conn.to_string()).or_insert(HashSet::new()).insert(root.to_string());
            }
        }

        Self { nodes, adjacent_nodes }
    }
}

pub fn main() {
    let input = puzzle_input_aslines(25, true);
    let mut graph: Graph = Graph::new(&input);
    println!("{:#?}", graph);
}
