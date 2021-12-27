use std::io;

fn read_crabs() -> Result<Vec<i32>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    if let Ok(_) = stdin.read_line(&mut buffer) {
        Ok(buffer.trim()
           .split(",")
           .map(|x| x.parse().unwrap())
           .collect())
    }
    else { Err(()) }
}

fn fuel_economy_median(crabs: &Vec<i32>) -> u32 {
    let mut sorted = crabs.clone();
    sorted.sort();
    
    let median = sorted[sorted.len()/2] as i32;

    sorted.iter()
        .map(|&x| i32::abs(median-x))
        .sum::<i32>() as u32
}

fn fuel_economy_average(crabs: &Vec<i32>) -> u32 {
    let average: f64 = (crabs.iter().sum::<i32>() as f64)/(crabs.len() as f64);

    u32::min(crabs.iter()
             .map(|&x| i32::abs((average.floor() as i32)-x))
             .map(|x| (1..=x).sum::<i32>())
             .sum::<i32>() as u32,
             
             crabs.iter()
             .map(|&x| i32::abs((average.ceil() as i32)-x))
             .map(|x| (1..=x).sum::<i32>())
             .sum::<i32>() as u32)
}

fn part1() {
    if let Ok(crabs) = read_crabs() {
        println!("{}", fuel_economy_median(&crabs));
    }
    else { println!("couldn't read crabs!"); }
}

fn part2() {
    if let Ok(crabs) = read_crabs() {
        println!("{}", fuel_economy_average(&crabs));
    }
    else { println!("couldn't read crabs!"); }
}

fn main() {
    // part1();
    part2();
}
