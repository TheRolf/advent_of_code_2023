use std::collections::HashSet;

use advent_of_code_2023::*;

struct Gardening {
    mappings: Vec<GardenMap>,
}

impl Gardening {
    pub fn new() -> Self {
        let mappings: Vec<GardenMap> = Vec::new();
        Gardening { mappings }
    }

    pub fn add_mapping(&mut self, mapping: GardenMap) {
        self.mappings.push(mapping);
    }

    pub fn get_result(&self, src_value: u64) -> u64 {
        let mut new_src_value = src_value;
        for mapping in &self.mappings {
            new_src_value = mapping.apply_map(new_src_value);
        }
        new_src_value
    }
}
struct GardenMap {
    mapping: HashSet<(u64, u64, u64)>,
}

impl GardenMap {
    pub fn new() -> Self {
        let mapping: HashSet<(u64, u64, u64)> = HashSet::new();
        GardenMap { mapping }
    }

    pub fn apply_map(&self, src_value: u64) -> u64 {
        for &(dest_start, src_start, range_len) in &self.mapping {
            if src_start <= src_value && src_value <= src_start + range_len - 1 {
                return dest_start + (src_value - src_start);
            }
        }
        src_value
    }
}

pub fn main() {
    let example = false;
    let input: Vec<String> = puzzle_input_aslines(5, example);
    let mut gardening: Gardening = Gardening::new();
    let mut current_map: GardenMap = GardenMap::new();
    let mut smallest: u64 = u64::MAX;
    for line in input {
        if line.chars().next().map_or(false, |c: char| c.is_numeric()) {
            let sline: Vec<u64> = line.split(" ").filter_map(|s| s.parse().ok()).collect();
            current_map.mapping.insert((*sline.get(0).unwrap(), *sline.get(1).unwrap(), *sline.get(2).unwrap()));
        } else {
            if !current_map.mapping.is_empty() {
                gardening.add_mapping(current_map);
                current_map = GardenMap::new();
            }
        }
    }
    if !current_map.mapping.is_empty() {
        gardening.add_mapping(current_map);
    }

    let numbers = if example {
        vec![79, 14, 55, 13]
    } else {
        vec![
            1347397244, 12212989, 2916488878, 1034516675, 2821376423, 8776260, 2240804122, 368941186, 824872000,
            124877531, 1597965637, 36057332, 4091290431, 159289722, 1875817275, 106230212, 998513229, 159131132,
            2671581775, 4213184,
        ]
    };
    let mut num: u64;
    let mut range: u64;
    let mut result: u64;
    for i in 0..numbers.len() / 2 {
        num = numbers[2 * i];
        range = numbers[2 * i + 1];
        for j in 0..range {
            result = gardening.get_result(num + j);
            if result < smallest {
                smallest = result;
            }
        }
    }
    println!("{}", smallest);
}
