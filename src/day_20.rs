#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

use advent_of_code_2023::*;

#[derive(Clone, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    Broadcaster {
        name: String,
        destinations: Vec<String>,
    },
    FlipFlop {
        name: String,
        destinations: Vec<String>,
        is_on: bool,
    },
    Conjunction {
        name: String,
        destinations: Vec<String>,
        previous_pulses: HashMap<String, Pulse>,
    },
    NoValue,
}

impl Module {
    pub fn new(line: &String) -> Self {
        let parts: Vec<&str> = line.split(" -> ").collect::<Vec<&str>>();
        let name = parts[0][1..].to_string();
        let destinations: Vec<String> =
            parts[1].split(", ").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
        match line.chars().nth(0).unwrap() {
            'b' => Module::Broadcaster {
                name: "broadcaster".to_string(),
                destinations,
            },
            '%' => Module::FlipFlop {
                name,
                destinations,
                is_on: false,
            },
            '&' => Module::Conjunction {
                name,
                destinations: destinations.clone(),
                previous_pulses: HashMap::new(),
            },
            _ => Module::NoValue,
        }
    }

    pub fn get_name(&self) -> String {
        match &self {
            &Self::Broadcaster { name, .. } => name.clone(),
            &Self::Conjunction { name, .. } => name.clone(),
            &Self::FlipFlop { name, .. } => name.clone(),
            _ => "NO_VALUE".to_string(),
        }
    }

    pub fn get_destinations(&self) -> Vec<String> {
        match &self {
            &Self::Broadcaster { destinations, .. } => destinations.clone(),
            &Self::Conjunction { destinations, .. } => destinations.clone(),
            &Self::FlipFlop { destinations, .. } => destinations.clone(),
            _ => vec![],
        }
    }

    pub fn pulse(&mut self, sender: String, incoming_pulse: Pulse, pulse_queue: &mut Vec<(String, Pulse, String)>) {
        match self {
            Self::Broadcaster { name, destinations, .. } => {
                for succ in destinations {
                    println!("{} -{:?}-> {}", name, incoming_pulse, succ);
                    pulse_queue.push((name.clone(), incoming_pulse.clone(), succ.clone()));
                }
            }
            Self::Conjunction {
                name,
                destinations,
                previous_pulses,
                ..
            } => {
                previous_pulses.entry(sender.clone()).and_modify(|pulse| *pulse = incoming_pulse.clone());
                let mut outgoing_pulse = Pulse::Low;
                for pulse in previous_pulses.values() {
                    if pulse == &Pulse::Low {
                        outgoing_pulse = Pulse::High;
                        break;
                    }
                }
                for succ in destinations {
                    println!("{} -{:?}-> {}", name, outgoing_pulse, succ);
                    pulse_queue.push((name.clone(), outgoing_pulse.clone(), succ.clone()));
                }
            }
            Self::FlipFlop {
                name,
                destinations,
                is_on,
                ..
            } => {
                if incoming_pulse == Pulse::Low {
                    *is_on = !(*is_on);
                    for succ in destinations {
                        println!(
                            "{} -{:?}-> {}",
                            name,
                            if *is_on { Pulse::High } else { Pulse::Low },
                            succ
                        );
                        pulse_queue.push((
                            name.clone(),
                            if *is_on { Pulse::High } else { Pulse::Low },
                            succ.clone(),
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}

struct System {
    modules: HashMap<String, Module>,
    pulse_queue: Vec<(String, Pulse, String)>,
}

impl System {
    pub fn new(input: &Vec<String>) -> Self {
        let mut modules: HashMap<String, Module> = HashMap::new();
        let mut conj_pred: HashMap<String, Vec<String>> = HashMap::new();
        for line in input {
            let mut module = Module::new(line);
            let name = module.get_name();
            for succ in module.get_destinations() {
                conj_pred.entry(succ).or_insert(Vec::new()).push(name.clone());
            }
            match &mut module {
                Module::Conjunction { previous_pulses, .. } => {
                    for prev in conj_pred.entry(name.clone()).or_insert(Vec::new()) {
                        previous_pulses.insert(prev.clone(), Pulse::Low);
                    }
                }
                _ => {}
            }
            modules.insert(name, module);
        }
        System {
            modules,
            pulse_queue: Vec::new(),
        }
    }

    pub fn press_button(&mut self) {
        self.modules
            .entry("broadcaster".to_string())
            .and_modify(|module| module.pulse(" ".to_string(), Pulse::Low, &mut self.pulse_queue));
        while !self.pulse_queue.is_empty() {
            let mut sender: String;
            let mut receiver: String;
            let mut module: &Module;
            let mut pulse: Pulse;
            (sender, pulse, receiver) = self.pulse_queue.remove(0);
            self.modules
                .entry(receiver.clone())
                .and_modify(|module| module.pulse(sender, pulse, &mut self.pulse_queue));
        }
    }
}

pub fn main() {
    let input = puzzle_input_aslines(20, true);
    let mut system = System::new(&input);
    for (name, module) in &system.modules {
        println!("{}: {:?}", name, module);
    }
    system.press_button();
}
