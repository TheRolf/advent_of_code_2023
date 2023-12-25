#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    pub fn new(text: &str) -> Self {
        let coords: Vec<i32> = text.split(",").map(|num| num.trim().parse::<i32>().unwrap()).collect();
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

#[derive(Debug)]
struct Hail {
    position: Point3D,
    velocity: Point3D,
}

impl Hail {
    pub fn new(line: String) -> Self {
        let split_text: Vec<&str> = line.split(" @ ").collect();
        Self {
            position: Point3D::new(split_text[0]),
            velocity: Point3D::new(split_text[1]),
        }
    }
}

struct Weather {
    hails: Vec<Hail>,
}

impl Weather {
    pub fn new(input: Vec<String>) -> Self {
        let mut hails: Vec<Hail> = Vec::new();
        for line in input {
            hails.push(Hail::new(line));
        }
        Self { hails }
    }
}

pub fn main() {
    let input = puzzle_input_aslines(24, true);
    let mut weather = Weather::new(input);
    for hail in weather.hails {
        println!("{:?}", hail);
    }
}
