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
    let example = false;
    let times = if example { vec![7, 15, 30] } else { vec![61, 67, 75, 71] };
    let records = if example { vec![9, 40, 200] } else { vec![430, 1036, 1307, 1150] };
    let mut total_ways: u64 = 1;
    for i in 0..times.len() {
        let beating = beating_record(times[i], records[i]);
        total_ways *= beating;
    }
    println!("{}", total_ways);

    let (time, record) = if example { (71530, 940200) } else { (61677571_u64, 430103613071150_u64) };
    println!("{}", beating_record(time, record));
}
