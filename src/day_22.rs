#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashSet;

use advent_of_code_2023::*;

#[derive(Debug, Clone, PartialEq)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    name: String,
    corners: (Point3D, Point3D),
}

impl Brick {
    pub fn new(line: &String, name: String) -> Self {
        let brick_text: Vec<&str> = line.split("~").collect();
        let corner_1: Vec<usize> = brick_text[0].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let corner_2: Vec<usize> = brick_text[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let point1 = Point3D {
            x: corner_1[0],
            y: corner_1[1],
            z: corner_1[2],
        };
        let point2 = Point3D {
            x: corner_2[0],
            y: corner_2[1],
            z: corner_2[2],
        };
        Self {
            name,
            corners: (point1, point2),
        }
    }

    pub fn contains(&self, point: &Point3D) -> bool {
        self.corners.0.x <= point.x
            && point.x <= self.corners.1.x
            && self.corners.0.y <= point.y
            && point.y <= self.corners.1.y
            && self.corners.0.z <= point.z
            && point.z <= self.corners.1.z
    }

    pub fn parts(&self) -> Vec<Point3D> {
        let mut parts_vec: Vec<Point3D> = Vec::new();
        for x in self.corners.0.x..=self.corners.1.x {
            for y in self.corners.0.y..=self.corners.1.y {
                for z in self.corners.0.z..=self.corners.1.z {
                    parts_vec.push(Point3D { x, y, z });
                }
            }
        }
        parts_vec
    }
}

#[derive(Debug, Clone)]
struct Structure {
    bricks: Vec<Brick>,
}

impl Structure {
    pub fn new(input: &Vec<String>) -> Self {
        let mut bricks: Vec<Brick> = Vec::new();
        let mut uppercase_letters = vec![b'A', b'A', b'A'];
        for line in input {
            bricks.push(Brick::new(line, String::from_utf8(uppercase_letters.clone()).unwrap()));
            if uppercase_letters[2] == b'Z' {
                uppercase_letters[2] = b'A';
                if uppercase_letters[1] == b'Z' {
                    uppercase_letters[1] = b'A';
                    uppercase_letters[0] += 1;
                } else {
                    uppercase_letters[1] += 1;
                }
            } else {
                uppercase_letters[2] += 1;
            }
        }
        Self { bricks }
    }

    pub fn fall(&mut self) -> usize {
        let mut changes = true;
        let mut has_fallen: HashSet<usize> = HashSet::new();
        while changes {
            changes = false;
            for i in 0..self.bricks.len() {
                let brick_bottom: usize = self.bricks[i].corners.0.z.min(self.bricks[i].corners.1.z);
                let mut can_fall = true;
                if brick_bottom == 1 {
                    continue;
                }
                for point in self.bricks[i].parts() {
                    let mut fallen_point = point.clone();
                    fallen_point.z -= 1;
                    for j in 0..self.bricks.len() {
                        if i != j && point.z == brick_bottom && self.bricks[j].contains(&fallen_point) {
                            can_fall = false;
                        }
                    }
                }
                if can_fall {
                    has_fallen.insert(i);
                    self.bricks[i].corners.0.z -= 1;
                    self.bricks[i].corners.1.z -= 1;
                    changes = true;
                }
            }
        }
        has_fallen.len()
    }

    pub fn safe_bricks(&self) -> usize {
        let mut count = 0;
        for i_remove in 0..self.bricks.len() {
            println!("{}", self.bricks[i_remove].name);
            let mut structure_altered = self.clone();
            structure_altered.bricks.remove(i_remove);
            let fallen_count = structure_altered.fall();
            if fallen_count == 0 {
                count += 1;
            }
        }
        count
    }
}

pub fn main() {
    let input = puzzle_input_aslines(22, false);
    let mut structure = Structure::new(&input);

    for brick in &structure.bricks {
        println!("{:?}", brick);
    }

    println!("{}", structure.fall());

    println!("{}", structure.safe_bricks());
}
