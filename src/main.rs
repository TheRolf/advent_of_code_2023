use std::env;

use advent_of_code_2023::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let day_number: usize = args[1].parse().unwrap();
        prepare_day(day_number);
    }
}
