use std::io;

fn simulate(fish: &Vec<u8>, steps: u16) -> u64 {
    let mut breedable = [0u64; 7];
    let mut available = [0u64; 7];

    for f in fish {
        breedable[*f as usize] += 1;
        available[*f as usize] += 1;
    }
    for s in 0..steps {
        let modulo = (s % 7) as usize;
        let delay = ((s + 2) % 7) as usize;

        available[delay] = available[delay] + breedable[modulo];
        breedable[modulo] = available[modulo];
    }

    available.iter().sum()
}

fn read_fish() -> Result<Vec<u8>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    if let Ok(size) = stdin.read_line(&mut buffer) {
        let mut fish: Vec<u8> = buffer.trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(fish)
    }
    else { Err(()) }
}

fn part1() {
    if let Ok(mut fish) = read_fish() {
        println!("{}", simulate(&fish, 80));
    }
    else { panic!("couldn't read fish!"); }
}

fn part2() {
    if let Ok(mut fish) = read_fish() {
        println!("{}", simulate(&fish, 256));
    }
    else { panic!("couldn't read fish!"); }
}

fn main() {
    // part1();
    part2();
}
