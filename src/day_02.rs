#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;
use std::collections::HashMap;

const COLORS: &[&str] = &["red", "green", "blue"];
struct Round {
    rgb: HashMap<String, u16>,
}

impl Round {
    pub fn new(round_str: &str) -> Self {
        let mut rgb: HashMap<String, u16> = HashMap::new();
        let cubes_str: Vec<&str> = round_str.split(",").collect();
        for cube_str in cubes_str {
            let cube: Vec<&str> = cube_str.split(" ").collect();
            let colour: String = cube.get(2).unwrap().to_string();
            let cube_count: u16 = cube.get(1).unwrap().parse::<u16>().unwrap();
            rgb.insert(colour, cube_count);
        }
        Round { rgb }
    }
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(game_str: &str) -> Self {
        let mut rounds: Vec<Round> = Vec::new();
        let rounds_str: Vec<&str> = game_str.split(";").collect();
        for round_str in rounds_str {
            let round = Round::new(round_str);
            rounds.push(round);
        }
        Self { rounds }
    }
    pub fn check(&self, red: u16, green: u16, blue: u16) -> bool {
        for round in &self.rounds {
            if round.rgb.contains_key("red") && round.rgb.get("red").unwrap() > &red {
                return false;
            }
            if round.rgb.contains_key("green") && round.rgb.get("green").unwrap() > &green {
                return false;
            }
            if round.rgb.contains_key("blue") && round.rgb.get("blue").unwrap() > &blue {
                return false;
            }
        }
        true
    }

    pub fn fewest(&self) -> Round {
        let mut rgb: HashMap<String, u16> = HashMap::new();
        rgb.insert("red".to_string(), 0);
        rgb.insert("green".to_string(), 0);
        rgb.insert("blue".to_string(), 0);
        for round in &self.rounds {
            for color in COLORS {
                if round.rgb.contains_key(*color)
                    && round.rgb.get(*color).unwrap() > rgb.get(*color).unwrap()
                {
                    rgb.insert((*color).to_string(), *round.rgb.get(*color).unwrap());
                }
            }
        }

        Round { rgb }
    }
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(2);
    let mut sum: u16 = 0;
    let mut prodsum: u32 = 0;
    for line in input {
        let game_strings: Vec<&str> = line.split(":").collect();
        let game_no: u16 = game_strings.get(0).unwrap().parse::<u16>().unwrap();
        let game_str: &str = game_strings.get(1).unwrap();
        let game: Game = Game::new(game_str);
        if game.check(12, 13, 14) {
            sum += game_no;
        }
        let fewest: Round = game.fewest();
        prodsum += (fewest.rgb.get("red").unwrap() * fewest.rgb.get("green").unwrap() * fewest.rgb.get("blue").unwrap()) as u32;
        // println!("{:?}", fewest.rgb);
    }
    println!("{}", sum);
    println!("{}", prodsum);
}
