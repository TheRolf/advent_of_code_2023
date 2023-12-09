#![allow(dead_code, unused_variables, unused_mut)]

use advent_of_code_2023::*;

struct Series {
    series: Vec<Vec<i32>>,
}

impl Series {
    pub fn new(line: String) -> Self {
        let mut series: Vec<Vec<i32>> = Vec::new();
        series.push(line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect());
        let mut index: usize = 0;
        loop {
            let diff: Vec<i32> = Series::calc_diff(series.get(index).unwrap());
            let sum: i32 = diff.iter().sum();
            let all_zeroes = diff.iter().all(|&x| x == 0);
            series.push(diff);
            if all_zeroes {
                break;
            }
            index += 1;
        }
        Series { series }
    }

    pub fn calc_diff(numbers: &Vec<i32>) -> Vec<i32> {
        let mut diff: Vec<i32> = Vec::new();
        for i in 0..numbers.len() - 1 {
            diff.push(numbers[i + 1] - numbers[i]);
        }
        diff
    }

    pub fn extrapolate(&mut self) -> i32 {
        for depth in (0..self.series.len()).rev() {
            if depth == self.series.len() - 1 {
                self.series[depth].push(0);
            } else {
                let extra_value: i32 =
                    self.series[depth].last().unwrap() + self.series[depth + 1].last().unwrap();
                self.series[depth].push(extra_value);
            }
        }
        *self.series[0].last().unwrap()
    }

    pub fn extrapolate_backwards(&mut self) -> i32 {
        for depth in (0..self.series.len()).rev() {
            if depth == self.series.len() - 1 {
                self.series[depth].insert(0, 0);
            } else {
                let extra_value: i32 = self.series[depth][0] - self.series[depth + 1][0];
                self.series[depth].insert(0, extra_value);
            }
        }
        *self.series[0].last().unwrap()
    }

    pub fn println(&self) {
        for series in &self.series {
            println!("{:?}", series);
        }
    }
}

pub fn main() {
    let input: Vec<String> = puzzle_input_aslines(9, false);
    let mut sequences: Vec<Series> = Vec::new();
    for line in input {
        sequences.push(Series::new(line));
    }
    let mut sum_a: i32 = 0;
    let mut sum_b: i32 = 0;
    for mut sequence in sequences {
        sequence.extrapolate();
        sequence.extrapolate_backwards();
        sum_a += sequence.series[0].last().unwrap();
        sum_b += sequence.series[0][0];
    }
    println!("{}", sum_a);
    println!("{}", sum_b);
}
