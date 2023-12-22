#![allow(dead_code, unused_variables, unused_mut)]

pub fn distance_travelled(hold_time: u64, total_time: u64) -> u64 {
    hold_time * (total_time - hold_time)
}

pub fn beating_record(total_time: u64, record_time: u64) -> u64 {
    let mut beating: u64 = 0;
    for i in 0..(total_time + 1) {
        if distance_travelled(i, total_time) > record_time {
            beating += 1;
        }
    }
    beating
}

pub fn main() {
    // let time = [7, 15, 30];
    // let record = [9, 40, 200];
    let time = [61, 67, 75, 71];
    let record = [430, 1036, 1307, 1150];
    let mut total_ways: u64 = 1;
    for i in 0..4 {
        let beating = beating_record(time[i], record[i]);
        println!("{}", beating);
        total_ways *= beating;
    }
    println!("{}", total_ways);

    // println!("{}", beating_record(71530, 940200));
    println!("{}", beating_record(61677571, 430103613071150));
}
