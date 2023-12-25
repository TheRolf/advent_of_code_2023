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

#[derive(Debug)]
struct MultiGraph {
    nodes: HashSet<String>,
    adjacent_nodes: HashMap<String, HashMap<String, usize>>,
    partition: Vec<String>,
}

impl MultiGraph {
    pub fn new(graph: &Graph) -> Self {
        let nodes = graph.nodes.clone();
        let mut adjacent_nodes = HashMap::new();
        for (node, adjacents) in &graph.adjacent_nodes {
            let adjacents_new: HashMap<String, usize> = adjacents.iter().fold(HashMap::new(), |mut acc, s| {
                acc.insert(s.clone(), 1);
                acc
            });
            adjacent_nodes.insert(node.clone(), adjacents_new);
        }
        MultiGraph {
            nodes,
            adjacent_nodes,
            partition: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("Nodes: {:?}", self.nodes);
        println!("Adjacency:");
        for (node, adjacents) in &self.adjacent_nodes {
            println!("{}: {:?}", node, adjacents);
        }
        println!("Min-cut: {:?}", self.partition);
    }

    pub fn merge_nodes(&mut self, node_1: &String, node_2: &String) {
        let new_node = format!("{}_{}", node_1, node_2);

        self.nodes.insert(new_node.clone());
        self.nodes.remove(node_1);
        self.nodes.remove(node_2);

        for (node, adjacents) in self.adjacent_nodes.iter_mut() {
            if node == node_1 || node == node_2 {
                continue;
            }
            let mut total_multiplicity = 0;
            if adjacents.contains_key(node_1) {
                total_multiplicity += adjacents.remove(node_1).unwrap();
            }
            if adjacents.contains_key(node_2) {
                total_multiplicity += adjacents.remove(node_2).unwrap();
            }
            if total_multiplicity > 0 {
                adjacents.insert(new_node.clone(), total_multiplicity);
            }
        }
        let mut new_adjacents: HashMap<String, usize> = HashMap::new();

        for (node, multiplicity) in self.adjacent_nodes.get(node_1).unwrap() {
            if node == node_2 {
                continue;
            }
            *new_adjacents.entry(node.clone()).or_insert(0) += multiplicity;
        }
        for (node, multiplicity) in self.adjacent_nodes.get(node_2).unwrap() {
            if node == node_1 {
                continue;
            }
            *new_adjacents.entry(node.clone()).or_insert(0) += multiplicity;
        }
        self.adjacent_nodes.insert(new_node, new_adjacents);
        self.adjacent_nodes.remove(node_1);
        self.adjacent_nodes.remove(node_2);
    }

    pub fn min_cut_phase(&mut self, start_node: &String) -> (String, String, usize) {
        let mut cut_of_the_phase: usize = 0;
        let mut nodes_checked: Vec<String> = Vec::new();
        nodes_checked.push(start_node.clone());
        while nodes_checked.len() < self.nodes.len() {
            let (node, value) = self.tightly_connected(&nodes_checked);
            nodes_checked.push(node);
            cut_of_the_phase = value;
        }
        let (merge_1, merge_2) = (nodes_checked.pop().unwrap(), nodes_checked.pop().unwrap());
        self.merge_nodes(&merge_1, &merge_2);
        (merge_1, merge_2, cut_of_the_phase)
    }

    pub fn min_cut(&mut self, start_node: String) -> usize {
        let mut min_cut_value = usize::MAX;
        while self.nodes.len() > 1 {
            println!("{}", self.nodes.len());
            let (merge_1, merge_2, cut_of_the_phase) = self.min_cut_phase(&start_node);
            if cut_of_the_phase < min_cut_value {
                min_cut_value = cut_of_the_phase;
                self.partition = {
                    if merge_1.len() > 3 {
                        merge_1
                    } else {
                        merge_2
                    }
                }
                .split("_")
                .map(|x| x.to_string())
                .collect();
            }
        }
        min_cut_value
    }

    pub fn tightly_connected(&self, component: &Vec<String>) -> (String, usize) {
        let mut best_node: String = "".to_string();
        let mut best_cut: usize = 0;
        for node in self.nodes.difference(&component.iter().cloned().collect::<HashSet<_>>()) {
            let cut_value = self.cut_between_node_and_component(&node, &component);
            if cut_value > best_cut {
                best_cut = cut_value;
                best_node = node.clone();
            }
        }
        (best_node, best_cut)
    }

    pub fn cut_between_node_and_component(&self, node: &String, component: &Vec<String>) -> usize {
        self.adjacent_nodes
            .get(node)
            .unwrap()
            .iter()
            .filter(|(other_node, _)| component.contains(*other_node))
            .fold(0, |acc, (_, multiplicity)| acc + *multiplicity)
    }
}

pub fn main() {
    let input = puzzle_input_aslines(25, false);
    let graph: Graph = Graph::new(&input);
    let mut mgraph: MultiGraph = MultiGraph::new(&graph);
    println!("Min-cut value: {}", mgraph.min_cut("rsh".to_string()));
    mgraph.print();
    println!(
        "{}",
        mgraph.partition.len() * (graph.nodes.len() - mgraph.partition.len())
    );
}
