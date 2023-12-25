#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::{HashMap, HashSet};

use advent_of_code_2023::*;
use graph::prelude::{CsrLayout, GraphBuilder, UndirectedCsrGraph};

pub fn main() {
    let input = puzzle_input_aslines(25, true);
    // let mut builder: GraphBuilder<String> = GraphBuilder::edges(self, edges)
    let mut nodes_str: HashSet<String> = HashSet::new();
    let mut edges_str: Vec<(String, String)> = Vec::new();
    for line in input {
        let split_text: Vec<&str> = line.split(": ").collect();
        let root = split_text[0];
        let connections: Vec<&str> = split_text[1].split(" ").collect();
        nodes_str.insert(root.to_string());
        for conn in connections {
            nodes_str.insert(conn.to_string());
            edges_str.push((root.to_string(), conn.to_string()));
        }
    }
    let mut nodes = Vec::from_iter(nodes_str.clone());
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (edge_a, edge_b) in edges_str {
        let edge_i = nodes_str.iter().position(|e| *e == edge_a).unwrap();
        let edge_j = nodes_str.iter().position(|e| *e == edge_b).unwrap();
        edges.push((edge_i, edge_j));
    }
    let g: UndirectedCsrGraph<usize> = GraphBuilder::new().csr_layout(CsrLayout::Sorted).edges(edges).build();
}
