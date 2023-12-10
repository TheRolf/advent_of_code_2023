use std::fs::{self, File};
use std::io::{Read, Write};

pub fn get_puzzle_input(day: u8, example: bool) -> String {
    let file_path: String = format!("data/day_{:02}{}.txt", day, if example { "_example" } else { "" });
    let mut file: File = File::open(&file_path).unwrap();
    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn puzzle_input_asarray(day: u8, example: bool) -> Vec<Vec<char>> {
    let content: String = get_puzzle_input(day, example);
    content.lines().map(|line: &str| line.chars().collect()).collect()
}

pub fn puzzle_input_aslines(day: u8, example: bool) -> Vec<String> {
    let content: String = get_puzzle_input(day, example);
    content.lines().map(String::from).collect()
}

pub fn print_2d_array(two_d_array: &Vec<Vec<char>>) {
    for row in two_d_array {
        let row_string: String = row.iter().collect();
        println!("{}", row_string);
    }
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
