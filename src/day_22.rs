#![allow(dead_code, unused_variables, unused_mut)]

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
    pub fn new(line: String, name: String) -> Self {
        let brick_text: Vec<&str> = line.split("~").collect();
        let corner_1: Vec<usize> = brick_text[0].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let corner_2: Vec<usize> = brick_text[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let point1 = Point3D{ x: corner_1[0], y: corner_1[1], z: corner_1[2] };
        let point2 = Point3D{ x: corner_2[0], y: corner_2[1], z: corner_2[2] };
        Self { name, corners: (point1, point2) }
    }

    pub fn contains(&self, point: &Point3D) -> bool{
        self.corners.0.x <= point.x && point.x <= self.corners.1.x &&
        self.corners.0.y <= point.y && point.y <= self.corners.1.y &&
        self.corners.0.z <= point.z && point.z <= self.corners.1.z
    }

    pub fn parts(&self) -> Vec<Point3D>{
        let mut parts_vec: Vec<Point3D> = Vec::new();
        for x in self.corners.0.x..=self.corners.1.x {
            for y in self.corners.0.y..=self.corners.1.y {
                for z in self.corners.0.z..=self.corners.1.z {
                    parts_vec.push(Point3D{ x, y, z });
                }
            }
        }
        parts_vec
    }

}

pub fn main() {
    let input = puzzle_input_aslines(22, false);
    let mut bricks: Vec<Brick> = Vec::new();
    let mut uppercase_letters = vec![b'A', b'A', b'A'];
    for line in input{
        bricks.push(Brick::new(line, String::from_utf8(uppercase_letters.clone()).unwrap()));
        if uppercase_letters[2] == b'Z' {
            uppercase_letters[1] += 1;
            uppercase_letters[2] = b'A';
        }
        if uppercase_letters[1] == b'Z' {
            uppercase_letters[0] += 1;
            uppercase_letters[1] = b'A';
        }
        uppercase_letters[2] += 1;
    }

    for brick in &bricks {
        println!("{:?}", brick);
    }

    println!("{}", bricks[0].contains(&Point3D{x: 1, y: 3, z: 1}));

    let mut changes = true;
    while changes {
        changes = false;
        for i in 0..bricks.len() {
            let brick_bottom: usize = bricks[i].corners.0.z.min(bricks[i].corners.1.z);
            let mut can_fall = true;
            if brick_bottom == 1 {
                continue;
            }
            for point in bricks[i].parts() {
                let mut fallen_point = point.clone();
                fallen_point.z -= 1;
                for j in 0..bricks.len() {
                    if i != j && point.z == brick_bottom && bricks[j].contains(&fallen_point) {
                        can_fall = false;
                    }

                }
            }
            if can_fall {
                bricks[i].corners.0.z -= 1;
                bricks[i].corners.1.z -= 1;
                changes = true;
            }
        }
    }

    for brick in &bricks {
        println!("{:?}", brick);
    }

    let mut count = 0;
    for i_remove in 0..bricks.len(){
        let mut bricks_altered = bricks.clone();
        bricks_altered.remove(i_remove);
        let mut any_fall = false;
        for i in 0..bricks_altered.len() {
            let brick_bottom: usize = bricks_altered[i].corners.0.z.min(bricks_altered[i].corners.1.z);
            let mut can_fall = true;
            if brick_bottom == 1 {
                continue;
            }
            for point in bricks_altered[i].parts() {
                let mut fallen_point = point.clone();
                fallen_point.z -= 1;
                for j in 0..bricks_altered.len() {
                    if i != j && point.z == brick_bottom && bricks_altered[j].contains(&fallen_point) {
                        can_fall = false;
                    }
                }
            }
            if can_fall {
                any_fall = true;
            }

        }
        if !any_fall {
            count += 1;
        }
    }

    println!("{}", count);

}
