#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashSet;
use advent_of_code_2023::*;

struct Gardening {
    mappings: Vec<GardenMap>
}

impl Gardening {
    pub fn new() -> Self{
        let mut mappings: Vec<GardenMap> = Vec::new();
        Gardening { mappings }
    }

    pub fn add_mapping(&mut self, mapping: GardenMap){
        self.mappings.push(mapping);
    }
}
struct GardenMap {
    mapping: HashSet<(u32, u32, u32)>
}

impl GardenMap {
    pub fn new() -> Self{
        let mut mapping: HashSet<(u32, u32, u32)> = HashSet::new();
        GardenMap { mapping }
    }

    pub fn apply_map(&self, src_value: u32) -> u32{
        for &(dest_start, src_start, range_len) in &self.mapping{
            println!("src: {}-{}, dest: {}-{}", src_start, src_start+range_len - 1, dest_start, dest_start+range_len - 1);
            if src_start <= src_value && src_value <= src_start + range_len - 1{
                return dest_start + (src_value - src_start);
            }
        }
        src_value
    }
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(5);
    let mut gardening: Gardening = Gardening::new();
    let mut current_map: GardenMap = GardenMap::new();
    for line in input{
        if line.chars().next().map_or(false, |c: char| c.is_numeric()){
            let sline: Vec<u32> = line.split(" ").filter_map(|s| s.parse().ok()).collect();
            current_map.mapping.insert((*sline.get(0).unwrap(), *sline.get(1).unwrap(), *sline.get(2).unwrap()));
        } else {
            if !current_map.mapping.is_empty(){
                gardening.add_mapping(current_map);
                current_map = GardenMap::new();
            }
        }
    }
    if !current_map.mapping.is_empty(){
        gardening.add_mapping(current_map);
    }
    for gardenmap in gardening.mappings{
        println!("{:?}", gardenmap.mapping);
    }
    // let mut seed_to_soil: GardenMap = GardenMap::new();
    // seed_to_soil.mapping.insert((50, 98, 2));
    // seed_to_soil.mapping.insert((52, 50, 48));
    // println!("{:?}", seed_to_soil.mapping);
    // println!("{}", seed_to_soil.apply_map(98));
}