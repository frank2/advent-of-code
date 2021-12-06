use std::io;

fn read_u32() -> Result<u32, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let input = stdin.read_line(&mut buffer);

    if input.is_err() { return Err(()); }

    let result = buffer.trim().parse::<u32>();

    if result.is_err() { return Err(()); }
    
    Ok(result.unwrap())
}

fn part1() {
    let mut prev = read_u32();
    let mut increase = 0u32;

    if prev.is_ok() {
        loop {
            let depth = read_u32();

            if depth.is_err() { break; }

            let d_unwrapped = depth.unwrap();
            let p_unwrapped = prev.unwrap();
            
            if d_unwrapped > p_unwrapped { increase += 1; }

            prev = depth;
        }
    }

    println!("{}", increase);
}

fn get_window(w: &mut Vec<u32>) -> Result<(), ()> {
    while w.len() >= 3 { w.remove(0); }

    while w.len() < 3 {
        let measurement = read_u32();

        if measurement.is_err() { return Err(()); }

        w.push(measurement.unwrap());
    }

    Ok(())
}

fn part2() {
    let mut window = Vec::<u32>::new();
    let mut prev: u32;
    let mut increase = 0u32;

    if get_window(&mut window).is_ok() {
        prev = window.iter().sum();

        loop {
            if get_window(&mut window).is_err() { break; }

            let sum = window.iter().sum();

            if sum > prev { increase += 1; }

            prev = sum;
        }
    }

    println!("{}", increase);
}

fn main() {
    part1();
    part2();
}
