#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

#[derive(Debug, Clone)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {

}

#[derive(Debug, Clone)]
struct Brick {
    corners: (Point3D, Point3D),
}

impl Brick {
    pub fn new(line: String) -> Self {
        let brick_text: Vec<&str> = line.split("~").collect();
        let corner_1: Vec<usize> = brick_text[0].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let corner_2: Vec<usize> = brick_text[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let point1 = Point3D{ x: corner_1[0], y: corner_1[1], z: corner_1[2] };
        let point2 = Point3D{ x: corner_2[0], y: corner_2[1], z: corner_2[2] };
        Self { corners: (point1, point2) }
    }

    pub fn contains(&self, point: Point3D) -> bool{
        self.corners.0.x <= point.x && point.x <= self.corners.1.x &&
        self.corners.0.y <= point.y && point.y <= self.corners.1.y &&
        self.corners.0.z <= point.z && point.z <= self.corners.1.z
    }
}

pub fn main() {
    let input = puzzle_input_aslines(22, true);
    let mut bricks: Vec<Brick> = Vec::new();
    for line in input{
        bricks.push(Brick::new(line));
    }

    for brick in &bricks {
        println!("{:?}", brick);
    }

    println!("{}", bricks[0].contains(Point3D{x: 1, y: 3, z: 1}))
}
