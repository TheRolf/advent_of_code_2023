#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

use std::collections::btree_set::{BTreeSet, IntoIter};

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
            let mut unique_elements = elements
                .clone()
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter();

            let (first_element, inner) = Self::next_level(&mut unique_elements, elements.clone())
                .expect("Must have at least one item");

            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }

    fn next_level(
        mut unique_elements: impl Iterator<Item = i32>,
        elements: Vec<i32>,
    ) -> Option<(i32, Box<Self>)> {
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
                        let (next_fe, next_i) =
                            Self::next_level(&mut *unique_elements, elements.clone())?;
                        *first_element = next_fe;
                        *inner = next_i;
                    }
                }
            },
        }
    }
}

#[derive(Clone)]
struct Row {
    cells: Vec<char>,
    config: Vec<usize>,
    damaged: Vec<usize>,
    unknown: Vec<usize>,
}

impl Row {
    pub fn new(line: String) -> Self {
        let split_input: Vec<&str> = line.split(" ").collect();
        let mut cells: Vec<char> = split_input[0].chars().collect();
        let mut config: Vec<usize> = split_input[1].split(",").flat_map(str::parse).collect();
        let mut damaged: Vec<usize> = Vec::new();
        let mut unknown: Vec<usize> = Vec::new();
        for (i, cell) in cells.iter().enumerate() {
            if cell == &'#' {
                damaged.push(i);
            } else if cell == &'?' {
                unknown.push(i);
            }
        }


        Row { cells, config, damaged, unknown }
    }

    pub fn is_valid(&self) -> bool {
        let mut actual_counts: Vec<usize> = Vec::new();
        let mut actual_count: usize = 0;
        for cell in &self.cells {
            if cell == &'#' {
                actual_count += 1;
            } else if actual_count > 0 {
                actual_counts.push(actual_count);
                actual_count = 0;
            }
        }
        if actual_count > 0 {
            actual_counts.push(actual_count);
        }
        self.config.iter().eq(actual_counts.iter())
    }

    pub fn iterate(&self) -> usize {
        let mut count: usize = 0;
        let unknown_count: usize = self.unknown.len();
        let damaged_count: usize = self.damaged.len();
        let config_count: usize = self.config.iter().sum();
        let mut base: Vec<i32> = vec![1; config_count - damaged_count];
        base.extend(vec![0; unknown_count - base.len()]);
        // for (j, perm) in base.iter().permutations(base.len()).unique().into_iter().enumerate() {
        for (j, perm) in UniquePermutations::new(base).enumerate() {
            // print!("{}   ", j);
            let mut new_row = self.clone();
            for (i, x) in perm.iter().enumerate() {
                if *x == 1 {
                    new_row.cells[ new_row.unknown[i] ] = '#';
                }
            }
            // println!("{:?} {:?} {:?}", new_row.cells, new_row.config, new_row.is_valid());
            if new_row.is_valid() {
                count += 1;
            }
        }
        count
    }
}



struct SpringRecord {
    rows: Vec<Row>
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(12, true);
    let mut spring_record: SpringRecord = SpringRecord { rows: Vec::new() };
    let mut count = 0;
    for line in input {
        let mut row: Row = Row::new(line);
        spring_record.rows.push(row);
    }

    for (i, row) in spring_record.rows.iter().enumerate() {
        // println!("{:?} {:?} {:?}", row.cells, row.config, row.is_valid());
        let local_count = row.iterate();
        println!("{} {}", i+1, local_count);
        count += local_count;
    }

    // let mut row = Row::new(".####??????.##### 1,2,3,2,1".to_string());
    // count = row.iterate();
    
    println!("{}", count);
}
