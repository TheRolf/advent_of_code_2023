#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

#[derive(Debug)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D {
    pub fn new(text: &str) -> Self {
        let coords: Vec<f32> = text.split(",").map(|num| num.trim().parse::<f32>().unwrap()).collect();
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

    pub fn at(&self, t: f32) -> Point3D {
        Point3D {
            x: self.position.x + t * self.velocity.x,
            y: self.position.y + t * self.velocity.y,
            z: 0.0,
        }
    }

    pub fn intersection(&self, other: &Hail) -> Option<Point3D> {
        let (a1, a2) = (-other.velocity.x, -other.velocity.y);
        let (b1, b2) = (self.velocity.x, self.velocity.y);
        let (c1, c2) = (other.position.x - self.position.x, other.position.y - self.position.y);
        let denominator = a1 * b2 - a2 * b1;
        let (s, t) = ((c1 * b2 - c2 * b1) / denominator, (a1 * c2 - a2 * c1) / denominator);
        if denominator == 0.0 || t < 0.0 || s < 0.0 {
            None
        } else {
            Some(self.at(t))
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
    let example = false;
    let input = puzzle_input_aslines(24, example);
    let mut weather = Weather::new(input);
    for hail in &weather.hails {
        println!("{:?}", hail);
    }
    let boundaries = if example {
        (7.0, 27.0)
    } else {
        (200_000_000_000_000.0, 400_000_000_000_000.0)
    };
    let mut count_intersection = 0;
    for i in 0..weather.hails.len() {
        for j in i + 1..weather.hails.len() {
            match weather.hails[i].intersection(&weather.hails[j]) {
                Some(point) => {
                    if point.x >= boundaries.0
                        && point.x <= boundaries.1
                        && point.y >= boundaries.0
                        && point.y <= boundaries.1
                    {
                        count_intersection += 1;
                    }
                }
                _ => {}
            }
        }
    }
    println!("{}", count_intersection);

    // b: smaller than 983_620_716_335_753
    // 983_620_716_335_751
}
