use std::collections::HashMap;

use advent_of_code_2023::*;

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
            let colour: String = cube[2].to_string();
            let cube_count: u16 = cube[1].parse::<u16>().unwrap();
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

    pub fn is_possible(&self, other_round: &Round) -> bool {
        for round in &self.rounds {
            for &color in COLORS {
                if round.rgb.contains_key(color)
                    && round.rgb.get(color).unwrap() > &other_round.rgb.get(color).unwrap()
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn fewest(&self) -> Round {
        let mut fewest: Round = Round::new(" 0 red, 0 green, 0 blue");
        for round in &self.rounds {
            for &color in COLORS {
                if round.rgb.contains_key(color) {
                    let color_count: &u16 = round.rgb.get(color).unwrap();
                    if color_count > fewest.rgb.get(color).unwrap() {
                        fewest.rgb.insert(color.to_string(), *color_count);
                    }
                }
            }
        }
        fewest
    }
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(2, false);
    let mut sum: u16 = 0;
    let mut prodsum: u32 = 0;
    let elf_check: Round = Round::new(" 12 red, 13 green, 14 blue");
    for line in input {
        let game_strings: Vec<&str> = line.split(":").collect();
        let game_no: u16 = game_strings[0].parse::<u16>().unwrap();
        let game_str: &str = game_strings[1];
        let game: Game = Game::new(game_str);
        if game.is_possible(&elf_check) {
            sum += game_no;
        }
        let fewest: Round = game.fewest();
        prodsum += fewest.rgb.values().fold(1, |acc: u16, x: &u16| acc * x) as u32;
    }
    println!("{}", sum);
    println!("{}", prodsum);
}
