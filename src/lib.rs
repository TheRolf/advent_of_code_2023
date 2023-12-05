use std::fs::File;
use std::io::Read;

pub fn get_puzzle_input(day: u8) -> String {
    let file_path: String = format!("data/day_{:02}.txt", day);
    let mut file: File = File::open(&file_path).unwrap();
    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn puzzle_input_asarray(day: u8) -> Vec<Vec<char>> {
    let content: String = get_puzzle_input(day);
    content
        .lines()
        .map(|line: &str| line.chars().collect())
        .collect()
}

pub fn puzzle_input_aslines(day: u8) -> Vec<String> {
    let content: String = get_puzzle_input(day);
    content.lines().map(String::from).collect()
}

pub fn print_2d_array(two_d_array: &Vec<Vec<char>>) {
    for row in two_d_array {
        let row_string: String = row.iter().collect();
        println!("{}", row_string);
    }
}

pub fn get_char(matrix: &Vec<Vec<char>>, i: u8, j: u8) -> char {
    matrix
        .get(i as usize)
        .and_then(|row| row.get(j as usize).cloned())
        .unwrap()
}
