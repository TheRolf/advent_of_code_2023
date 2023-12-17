use std::collections::btree_set::{BTreeSet, IntoIter};
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{Read, Write};

pub fn get_puzzle_input(day: u8, example: bool) -> String {
    let file_path: String = format!("data/day_{:02}{}.txt", day, if example { "_example" } else { "" });
    let mut file: File = File::open(&file_path).unwrap();
    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn puzzle_input_aslines(day: u8, example: bool) -> Vec<String> {
    let content: String = get_puzzle_input(day, example);
    content.lines().map(String::from).collect()
}

pub fn puzzle_input_asarray(day: u8, example: bool) -> Vec<Vec<char>> {
    let content: String = get_puzzle_input(day, example);
    content.lines().map(|line: &str| line.chars().collect()).collect()
}

pub fn print_2d_array<T: Display>(two_d_array: &Vec<Vec<T>>) {
    for row in two_d_array {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

pub fn prepare_day(day_number: usize) {
    let puzzle_input: String = format!("data/day_{:02}.txt", day_number);
    let puzzle_example: String = format!("data/day_{:02}_example.txt", day_number);
    let puzzle_code: String = format!("src/day_{:02}.rs", day_number);
    File::create(puzzle_input).unwrap().write_all(b"").unwrap();
    File::create(puzzle_example).unwrap().write_all(b"").unwrap();
    if !fs::metadata(&puzzle_code).is_ok() {
        fs::copy("src/day_template.rs", &puzzle_code).unwrap();
        let mut toml_file: File = fs::OpenOptions::new().append(true).open("Cargo.toml").unwrap();
        writeln!(toml_file, "").unwrap();
        writeln!(toml_file, "[[bin]]").unwrap();
        writeln!(toml_file, "name = \"{}\"", day_number).unwrap();
        writeln!(toml_file, "path = \"src/day_{:02}.rs\"", day_number).unwrap();
    }
}

enum UniquePermutations {
    Leaf {
        elements: Option<Vec<i32>>,
    },
    Stem {
        elements: Vec<i32>,
        unique_elements: IntoIter<i32>,
        first_element: i32,
        inner: Box<Self>,
    },
}

impl UniquePermutations {
    fn new(elements: Vec<i32>) -> Self {
        if elements.len() == 1 {
            let elements = Some(elements);
            Self::Leaf { elements }
        } else {
            let mut unique_elements = elements.clone().into_iter().collect::<BTreeSet<_>>().into_iter();

            let (first_element, inner) = Self::next_level(&mut unique_elements, elements.clone()).expect("Must have at least one item");

            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }

    fn next_level(mut unique_elements: impl Iterator<Item = i32>, elements: Vec<i32>) -> Option<(i32, Box<Self>)> {
        let first_element = unique_elements.next()?;

        let mut remaining_elements = elements;

        if let Some(idx) = remaining_elements.iter().position(|&i| i == first_element) {
            remaining_elements.remove(idx);
        }

        let inner = Box::new(Self::new(remaining_elements));

        Some((first_element, inner))
    }
}

impl Iterator for UniquePermutations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Leaf { elements } => elements.take(),
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            } => loop {
                match inner.next() {
                    Some(mut v) => {
                        v.insert(0, *first_element);
                        return Some(v);
                    }
                    None => {
                        let (next_fe, next_i) = Self::next_level(&mut *unique_elements, elements.clone())?;
                        *first_element = next_fe;
                        *inner = next_i;
                    }
                }
            },
        }
    }
}
