use std::collections::HashMap;

use advent_of_code_2023::*;

const CARD_COUNT: u16 = 202;
struct Card {
    card_no: u16,
    your_numbers: Vec<u16>,
    card_numbers: Vec<u16>,
}

impl Card {
    pub fn new(card_str: &str) -> Card {
        let parts: Vec<&str> = card_str.split(|c| c == ':' || c == '|').collect();
        let header: Vec<&str> = parts.get(0).unwrap().split(" ").collect();
        let your_numbers_str: Vec<&str> = parts.get(1).unwrap().split(" ").collect();
        let card_numbers_str: Vec<&str> = parts.get(2).unwrap().split(" ").collect();
        let card_no: u16 = header.get(header.len() - 1).unwrap().parse::<u16>().unwrap();
        let mut your_numbers: Vec<u16> = Vec::new();
        let mut card_numbers: Vec<u16> = Vec::new();
        for num in your_numbers_str {
            if num != "" {
                your_numbers.push(num.parse::<u16>().unwrap());
            }
        }
        for num in card_numbers_str {
            if num != "" {
                card_numbers.push(num.parse::<u16>().unwrap());
            }
        }
        Card {
            card_no,
            your_numbers,
            card_numbers,
        }
    }

    pub fn scratch(&self) -> u32 {
        let mut value = 0;
        for num in &self.card_numbers {
            if self.your_numbers.contains(&num) {
                if value == 0 {
                    value = 1
                } else {
                    value = value * 2
                }
            }
        }
        value
    }

    pub fn scratch_recurse(&self) -> u16 {
        let mut value = 0;
        for num in &self.card_numbers {
            if self.your_numbers.contains(&num) {
                value += 1;
            }
        }
        value
    }
}

pub fn main() {
    let input = puzzle_input_aslines(4, false);
    let mut sum_a: u32 = 0;
    for line in &input {
        let card: Card = Card::new(line);
        sum_a += card.scratch();
    }
    println!("{}", sum_a);

    let mut sum_b: u32 = 0;
    let mut extra_cards: HashMap<u16, u32> = HashMap::new();
    for i in 0..CARD_COUNT {
        extra_cards.insert(i + 1, 1);
    }
    for line in &input {
        let card: Card = Card::new(line);
        for _ in 0..*extra_cards.get(&card.card_no).unwrap() {
            let result_of_card = card.scratch_recurse();
            for j in 0..result_of_card {
                let card_no_to_modify = card.card_no + j + 1;
                if card_no_to_modify <= CARD_COUNT {
                    extra_cards.entry(card_no_to_modify).and_modify(|v| *v += 1);
                }
            }
        }
    }
    for i in 0..CARD_COUNT {
        sum_b += extra_cards.get(&(i + 1)).unwrap();
    }
    println!("{}", sum_b);
    println!("{:?}", extra_cards);
}
