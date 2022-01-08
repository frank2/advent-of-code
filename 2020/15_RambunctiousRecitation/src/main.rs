use std::io;
use std::collections::HashMap;

fn read_numbers() -> Result<Vec<usize>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut numbers = Vec::<usize>::new();

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { return Err(()); }

        numbers = buffer.trim()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
    }

    if numbers.len() == 0 { Err(()) }
    else { Ok(numbers) }
}

fn play_game(start: &Vec<usize>, stop: usize) -> usize {
    let mut turn = 1usize;
    let mut priors = HashMap::<usize, (Option<usize>, Option<usize>)>::new();
    let mut last = 0usize;

    for init in start {
        let entry = priors.entry(*init).or_insert((None, None));

        (*entry).0 = Some(turn);
        last = *init;
        turn += 1;
    }

    while turn <= stop {
        let prior = priors.entry(last).or_insert((None, None));
        let new;

        if prior.1.is_none() { new = 0usize; }
        else { new = prior.0.clone().unwrap() - prior.1.clone().unwrap(); }

        let entry = priors.entry(new).or_insert((None, None));

        if entry.0.is_some() { (*entry).1 = entry.0; }
        (*entry).0 = Some(turn);

        last = new;
        turn += 1;
    }

    last
}
    
fn part1() {
    if let Ok(numbers) = read_numbers() {
        println!("{}", play_game(&numbers, 2020));
    }
    else { panic!("couldn't read numbers!"); }
}
    
fn part2() {
    if let Ok(numbers) = read_numbers() {
        println!("{}", play_game(&numbers, 30000000));
    }
    else { panic!("couldn't read numbers!"); }
}

fn main() {
    // part1();
    part2();
}
