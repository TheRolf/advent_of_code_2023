use advent_of_code_2023::*;

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Clone)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    pub fn new() -> Self {
        let lenses: Vec<Lens> = Vec::new();
        Self { lenses }
    }

    pub fn index_of(&self, label: &String) -> i32 {
        for (i, lens) in self.lenses.iter().enumerate() {
            if lens.label.eq(label) {
                return i as i32;
            }
        }
        -1
    }

    pub fn add(&mut self, label: String, focal_length: usize) {
        let index: i32 = self.index_of(&label);
        if index < 0 {
            self.lenses.push(Lens {
                label,
                focal_length,
            });
        } else {
            self.lenses[index as usize].focal_length = focal_length;
        }
    }

    pub fn remove(&mut self, label: String) {
        let index: i32 = self.index_of(&label);
        if index >= 0 {
            self.lenses.remove(index as usize);
        }
    }

    pub fn get_focal_length(&self) -> usize {
        let mut focal_length: usize = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            focal_length += (i + 1) * lens.focal_length;
        }
        focal_length
    }
}

pub fn hash(text: &String) -> usize {
    let mut hash_value: usize = 0;
    for c in text.as_bytes() {
        hash_value += *c as usize;
        hash_value *= 17;
        hash_value %= 256;
    }
    hash_value
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(15, false);
    let mut sum_a: usize = 0;
    let mut sum_b: usize = 0;
    let mut boxes: Vec<Box> = vec![Box::new(); 256];

    let steps: Vec<&str> = input[0].split(",").collect();
    for step in steps {
        let last_char: char = step.chars().last().unwrap();
        let label_len: usize;
        if last_char == '-' {
            label_len = step.len() - 1;
        } else {
            label_len = step.len() - 2;
        }
        let label: String = step.to_string()[0..label_len].to_string();
        let hash_of_label: usize = hash(&label);
        if last_char == '-' {
            boxes[hash_of_label].remove(label);
        } else {
            let focal_length: usize = last_char.to_digit(10).unwrap() as usize;
            boxes[hash_of_label].add(label, focal_length);
        }
        sum_a += hash(&step.to_string());
    }
    println!("{}", sum_a);

    for (i, box_) in boxes.iter().enumerate() {
        if box_.lenses.len() > 0 {
            print!("Box {}:", i);
            for lens in &box_.lenses {
                print!(" [{} {}]", lens.label, lens.focal_length);
            }
            println!(" {}", box_.get_focal_length());
            sum_b += (i + 1) * box_.get_focal_length();
        }
    }
    println!("{}", sum_b);
}
