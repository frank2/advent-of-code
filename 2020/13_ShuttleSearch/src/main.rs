use std::io;
use std::collections::HashMap;
use std::ops::Range;

fn read_schedule() -> Result<(usize, Vec<usize>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut timestamp = 0usize;
    let mut schedule = Vec::<usize>::new();

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { return Err(()); }

        timestamp = buffer.trim().parse().unwrap();
        buffer.clear();
    }
    else { return Err(()); }

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { return Err(()); }

        schedule = buffer.trim()
            .split(",")
            .map(|x| if x == "x" { 0 } else { x.parse::<usize>().unwrap() })
            .collect();
    }
    else { return Err(()); }

    if schedule.len() == 0 { Err(()) }
    else { Ok((timestamp, schedule)) }
}

fn part1() {
    if let Ok((timestamp, schedule)) = read_schedule() {
        let mut best_bus = 0usize;
        let mut best_time = usize::MAX;

        for bus_id in schedule {
            if bus_id == 0 { continue; }
        
            let bus_time = (0..timestamp+bus_id).step_by(bus_id).last().unwrap();
            if bus_time < best_time { best_time = bus_time; best_bus = bus_id; }
        }

        println!("{}", (best_time - timestamp) * best_bus);
    }
    else { panic!("couldn't read schedule!"); }
}

fn part2() {
    if let Ok((_, schedule)) = read_schedule() {
        let mut timestamp = 0;
        let mut period = 1;

        for phase in 0..schedule.len() {
            if schedule[phase] == 0 { continue; }
            
            while (timestamp+phase) % schedule[phase] != 0 {
                timestamp += period;
            }

            period *= schedule[phase];
        }

        println!("{}", timestamp);
    }
    else { panic!("couldn't read schedule!"); }
}

fn main() {
    // part1();
    part2();
}
