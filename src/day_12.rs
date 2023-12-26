#![allow(unused_variables)]

use std::collections::btree_set::{BTreeSet, IntoIter};

use advent_of_code_2023::*;

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

            let (first_element, inner) =
                Self::next_level(&mut unique_elements, elements.clone()).expect("Must have at least one item");

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
        let cells: Vec<char> = split_input[0].chars().collect();
        let config: Vec<usize> = split_input[1].split(",").flat_map(str::parse).collect();
        let mut damaged: Vec<usize> = Vec::new();
        let mut unknown: Vec<usize> = Vec::new();
        for (i, cell) in cells.iter().enumerate() {
            if cell == &'#' {
                damaged.push(i);
            } else if cell == &'?' {
                unknown.push(i);
            }
        }

        Row {
            cells,
            config,
            damaged,
            unknown,
        }
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

    pub fn iterate_brute_force(&self) -> (usize, usize) {
        let mut count: usize = 0;
        let mut count_invalid: usize = 0;
        let unknown_count: usize = self.unknown.len();
        let damaged_count: usize = self.damaged.len();
        let config_count: usize = self.config.iter().sum();
        let mut base: Vec<i32> = vec![1; config_count - damaged_count];
        base.extend(vec![0; unknown_count - base.len()]);
        if base.len() == 0 {
            return (1, 0);
        }
        for (_, perm) in UniquePermutations::new(base).enumerate() {
            let mut new_row = self.clone();
            for (i, x) in perm.iter().enumerate() {
                if *x == 1 {
                    new_row.cells[new_row.unknown[i]] = '#';
                }
            }
            if new_row.is_valid() {
                count += 1;
            } else {
                count_invalid += 1;
            }
        }
        (count, count_invalid)
    }

    pub fn fix_damaged_seq_at_pos(&mut self, seq_len: usize, position: usize) {
        if position > 0 {
            self.cells[position - 1] = '.';
        }
        for i in 0..seq_len {
            self.cells[position + i] = '#';
        }
        if position + seq_len < self.cells.len() {
            self.cells[position + seq_len] = '.';
        }
        self.damaged = Vec::new();
        self.unknown = Vec::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if cell == &'#' {
                self.damaged.push(i);
            } else if cell == &'?' {
                self.unknown.push(i);
            }
        }
    }

    pub fn iterate_heuristic(&mut self) -> (usize, usize) {
        let mut new_row = self.clone();

        // fixing the damaged sequences from the left until they are certain
        let mut cell_index = 0;
        let mut config_index = 0;
        loop {
            if cell_index >= new_row.cells.len() || new_row.cells[cell_index] == '?' {
                break;
            } else if new_row.cells[cell_index] == '#' {
                new_row.fix_damaged_seq_at_pos(new_row.config[config_index], cell_index);
                cell_index += new_row.config[config_index];
                config_index += 1;
            } else {
                cell_index += 1;
            }
        }

        // fixing the damaged sequences from the right until they are certain
        let mut cell_index = new_row.cells.len() - 1;
        let mut config_index = new_row.config.len() - 1;
        loop {
            if cell_index <= 0 || new_row.cells[cell_index] == '?' {
                break;
            } else if new_row.cells[cell_index] == '#' {
                new_row.fix_damaged_seq_at_pos(
                    new_row.config[config_index],
                    cell_index + 1 - new_row.config[config_index],
                );
                if cell_index < new_row.config[config_index] || config_index == 0 {
                    break;
                }
                cell_index -= new_row.config[config_index];
                config_index -= 1;
            } else {
                cell_index -= 1;
            }
        }

        // fixing the damaged sequences from the largest size until they are certain
        let config_desc = new_row.config.clone().sort_by(|a, b| b.cmp(a));

        println!(
            "{} -> {}",
            self.cells.iter().collect::<String>(),
            new_row.cells.iter().collect::<String>()
        );
        new_row.iterate_brute_force()
    }
}

struct SpringRecord {
    rows: Vec<Row>,
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(12, false);
    let mut spring_record: SpringRecord = SpringRecord { rows: Vec::new() };
    let mut count = 0;
    for line in input {
        let row: Row = Row::new(line);
        spring_record.rows.push(row);
    }

    for (i, row) in spring_record.rows.iter_mut().enumerate() {
        print!("{}: ", i + 1);
        let local_count = row.iterate_heuristic();
        print!("{:?} \t", local_count);
        println!();
        count += local_count.0;
    }
    println!("{}", count);
}
