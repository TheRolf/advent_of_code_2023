#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    Nothing = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

impl fmt::Display for HandValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [char; 5],
    bid: u32,
    line: String,
}

impl Hand {
    pub fn new(line: String) -> Self {
        let parts: Vec<&str> = line.split(" ").collect();
        let mut cards: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        for i in 0..5 {
            cards[i] = parts.get(0).unwrap().chars().nth(i).unwrap();
        }
        let bid: u32 = parts.get(1).unwrap().parse::<u32>().unwrap();
        Hand { cards, bid, line }
    }

    pub fn println(&self) {
        println!("{:?}: {}  {}", self.cards, self.bid, self.value());
    }

    pub fn value(&self) -> HandValue {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for &card in &self.cards {
            *counter.entry(card).or_insert(0) += 1;
        }
        if counter.values().any(|&v| v == 5) {
            return HandValue::FiveKind;
        }
        if counter.values().any(|&v| v == 4) {
            return HandValue::FourKind;
        }
        if counter.values().any(|&v| v == 3) {
            if counter.values().any(|&v| v == 2) {
                return HandValue::FullHouse;
            } else {
                return HandValue::ThreeKind;
            }
        }
        if counter.values().any(|&v| v == 2) {
            if counter.len() == 3 {
                return HandValue::TwoPair;
            } else {
                return HandValue::OnePair;
            }
        }
        HandValue::Nothing
    }

    pub fn card_values(&self) -> [u8; 5] {
        let mut values: [u8; 5] = [0, 0, 0, 0, 0];
        for i in 0..5 {
            values[i] = self.card_value(self.cards[i]);
        }
        values
    }

    pub fn card_value(&self, card: char) -> u8 {
        if card.is_digit(10) {
            return card.to_digit(10).unwrap() as u8;
        }
        if card == 'T' {
            return 10;
        }
        if card == 'J' {
            return 1;
        }
        if card == 'Q' {
            return 12;
        }
        if card == 'K' {
            return 13;
        }
        if card == 'A' {
            return 14;
        }
        return 0;
    }

    pub fn substitute_joker(&self) -> HandValue {
        let mut counter: HashMap<char, usize> = HashMap::new();
        let mut substituted_hand: Hand = Hand::new(self.line.clone());
        for &card in &substituted_hand.cards {
            *counter.entry(card).or_insert(0) += 1;
        }
        if counter.contains_key(&'J') {
            let mut frequent_card: char = ' ';
            let mut frequency: usize = 0;
            for (card, freq) in counter {
                if card != 'J' && freq >= frequency {
                    if freq > frequency
                        || substituted_hand.card_value(card)
                            > substituted_hand.card_value(frequent_card)
                    {
                        frequent_card = card;
                        frequency = freq;
                    }
                }
            }
            for i in 0..5 {
                if substituted_hand.cards[i] == 'J' {
                    substituted_hand.cards[i] = frequent_card;
                }
            }
        }
        return substituted_hand.value();
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self
            .substitute_joker()
            .partial_cmp(&other.substitute_joker())
        {
            Some(Ordering::Equal) => {
                // Compare based on the array values
                Some(self.card_values().cmp(&other.card_values()))
            }
            result => result,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(7);
    let mut hands: Vec<Hand> = Vec::new();
    for line in input {
        let mut new_hand: Hand = Hand::new(line);
        println!("{:?}", new_hand.cards);
        hands.push(new_hand);
    }

    hands.sort();

    let mut winnings = 0;
    let mut i = 1;
    for mut hand in hands {
        hand.println();
        hand.value();
        winnings += i * hand.bid;
        i += 1;
    }
    println!("{}", winnings);
}
