#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

use advent_of_code_2023::*;

#[derive(Debug)]
struct Part {
    rating: HashMap<char, usize>,
}

impl Part {
    pub fn new(text: String) -> Self {
        let mut rating: HashMap<char, usize> = HashMap::new();
        let entries: Vec<&str> = text[1..text.len() - 1].split(",").collect();
        for entry in entries {
            let var: char = entry.chars().nth(0).unwrap();
            let value: usize = entry[2..].parse::<usize>().unwrap();
            rating.insert(var, value);
        }
        Self { rating }
    }
    pub fn value(&self) -> usize {
        self.rating.values().sum()
    }
}

#[derive(Debug)]
struct Rule {
    var: char,
    cond: char,
    value: usize,
    state: String,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    or_else: String,
}

impl Workflow {
    pub fn new(text: String) -> Self {
        let mut rules: Vec<Rule> = Vec::new();
        let mut or_else: String = "".to_string();
        let split_text: Vec<&str> = text.split(",").collect();
        for cmd in split_text {
            if cmd.contains(':') {
                let var = cmd.chars().nth(0).unwrap();
                let cond = cmd.chars().nth(1).unwrap();
                let second_half: Vec<&str> = cmd[2..].split(":").collect();
                let value = second_half[0].parse::<usize>().unwrap();
                let state = second_half[1].to_string();
                rules.push(Rule {
                    var,
                    cond,
                    value,
                    state,
                });
            } else {
                or_else = cmd.to_string();
            }
        }
        Self { rules, or_else }
    }

    pub fn process(&self, part: &Part) -> &String {
        for rule in &self.rules {
            match rule.cond {
                '<' => {
                    if part.rating.get(&rule.var).unwrap() < &rule.value {
                        return &rule.state;
                    }
                }
                '>' => {
                    if part.rating.get(&rule.var).unwrap() > &rule.value {
                        return &rule.state;
                    }
                }
                _ => {}
            }
        }
        &self.or_else
    }
}

#[derive(Clone)]
struct Config {
    bounds: HashMap<char, (usize, usize)>,
    next_state: String,
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn count(&self) -> usize {
        let mut count: usize = 1;
        for (min_x, max_x) in self.bounds.values() {
            count *= max_x - min_x + 1;
        }
        count
    }
}

impl Default for Config {
    fn default() -> Self {
        let next_state: String = "in".to_string();
        let mut bounds: HashMap<char, (usize, usize)> = HashMap::new();
        for c in vec!['x', 'm', 'a', 's'] {
            bounds.insert(c, (1, 4000));
        }
        Self { next_state, bounds }
    }
}
struct System {
    workflows: HashMap<String, Workflow>,
}

impl System {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn add(&mut self, text: String) {
        let split_text: Vec<&str> = text.split("{").collect();
        let workflow: Workflow =
            Workflow::new(split_text[1][..(split_text[1].len() - 1)].to_string());
        self.workflows.insert(split_text[0].to_string(), workflow);
    }

    pub fn accepts(&self, part: &Part) -> bool {
        let mut label: &String = &"in".to_string();
        while label != &"A".to_string() && label != &"R".to_string() {
            label = self.workflows.get(label).unwrap().process(&part);
        }
        label.as_str() == "A"
    }

    pub fn process(&self, part: &Part) -> usize {
        if self.accepts(part) {
            return part.value();
        }
        0
    }

    pub fn accept_count_brute_force(&self) -> usize {
        let mut sum: usize = 0;
        for x in 1..4001 {
            for m in 1..4001 {
                println!("{} {} {}", x, m, sum);
                for a in 1..4001 {
                    for s in 1..4001 {
                        let mut rating: HashMap<char, usize> = HashMap::new();
                        rating.insert('x', x);
                        rating.insert('m', m);
                        rating.insert('a', a);
                        rating.insert('s', s);
                        let part: Part = Part { rating };
                        if self.accepts(&part) {
                            sum += 1;
                        }
                    }
                }
            }
        }
        sum
    }

    pub fn find_accept_bounds(&self) -> usize {
        let mut to_explore: Vec<Config> = Vec::new();
        let mut accept_bounds: Vec<Config> = Vec::new();
        to_explore.push(Config::new());
        while !to_explore.is_empty() {
            let mut config: Config = to_explore.pop().unwrap();
            match config.next_state.as_str() {
                "A" => accept_bounds.push(config),
                "R" => continue,
                _ => {
                    for rule in &self.workflows.get(&config.next_state).unwrap().rules {
                        let mut new_config = config.clone();
                        new_config.next_state = rule.state.clone();
                        if rule.cond == '<' {
                            new_config.bounds.entry(rule.var).or_insert((1, 4000)).1 =
                                rule.value - 1;
                            config.bounds.entry(rule.var).or_insert((1, 4000)).0 = rule.value;
                        } else {
                            new_config.bounds.entry(rule.var).or_insert((1, 4000)).0 =
                                rule.value + 1;
                            config.bounds.entry(rule.var).or_insert((1, 4000)).1 = rule.value;
                        }
                        to_explore.push(new_config);
                    }
                    config.next_state = self
                        .workflows
                        .get(&config.next_state)
                        .unwrap()
                        .or_else
                        .clone();
                    to_explore.push(config);
                }
            }
        }
        let mut total: usize = 0;
        for config in accept_bounds {
            let config_count = config.count();
            // println!("{:?} {}", config.bounds,config_count);
            total += config_count;
        }
        total
    }
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(19, false);
    let mut system: System = System::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut rules: bool = true;
    for line in input {
        if line.is_empty() {
            rules = false;
        } else if rules {
            system.add(line)
        } else {
            parts.push(Part::new(line));
        }
    }

    let mut sum_a: usize = 0;
    for part in &parts {
        sum_a += &system.process(part);
    }
    println!("{}", sum_a);
    println!("{}", system.find_accept_bounds());
}
