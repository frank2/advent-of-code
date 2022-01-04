use std::io;
use std::collections::HashMap;

fn jolt_differences(jolts: &Vec<usize>) -> (usize, usize) {
    let mut sorted_jolts = jolts.clone();
    let mut last_joltage = 0usize;
    let mut one_jolt = 0usize;
    let mut three_jolts = 0usize;
    sorted_jolts.sort();

    for jolt in sorted_jolts {
        let delta = jolt - last_joltage;
        last_joltage = jolt;

        if delta == 1 { one_jolt += 1; }
        else if delta == 3 { three_jolts += 1; }
        else { panic!("jolt delta is not 1 or 3"); }
    }

    (one_jolt, three_jolts+1)
}

fn jolt_combos(jolts: &Vec<usize>) -> usize {
    let mut sorted_jolts = jolts.clone();
    let mut last_joltage = 0usize;
    let mut jolt_count = 0usize;
    let mut jolt_counts = Vec::<usize>::new();
    sorted_jolts.sort();

    let combo_map: HashMap<usize, usize> = [(1,1),(2,2),(3,4),(4,7)].iter().copied().collect();

    for jolt in sorted_jolts {
        let delta = jolt - last_joltage;
        last_joltage = jolt;

        if delta == 1 { jolt_count += 1; }
        else if delta == 3 {
            if jolt_count > 0 { jolt_counts.push(*combo_map.get(&jolt_count).unwrap()); }
            jolt_count = 0;
        }
        else { panic!("jolt delta is not 1 or 3"); }
    }

    if jolt_count > 0 { jolt_counts.push(*combo_map.get(&jolt_count).unwrap()); }

    jolt_counts.iter().product()
}

fn read_jolts() -> Result<Vec<usize>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut jolts = Vec::<usize>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        jolts.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    if jolts.len() == 0 { Err(()) }
    else { Ok(jolts) }
}
        
fn part1() {
    if let Ok(jolts) = read_jolts() {
        let (one, three) = jolt_differences(&jolts);
        println!("{}", one*three);
    }
    else { panic!("couldn't read jolts!"); }
}
        
fn part2() {
    if let Ok(jolts) = read_jolts() {
        println!("{}", jolt_combos(&jolts));
    }
    else { panic!("couldn't read jolts!"); }
}

fn main() {
    // part1();
    part2();
}
