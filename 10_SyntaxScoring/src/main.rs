use std::io;
use std::collections::HashMap;

fn read_nav() -> Result<Vec<String>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut lines = Vec::<String>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        lines.push(buffer.trim().to_string());
        buffer.clear();
    }

    if lines.len() == 0 { Err(()) }
    else { Ok(lines) }
}

fn check_syntax(lines: &Vec<String>) -> (u64, u64) {
    let openers: HashMap<char, char> = [
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ].iter().cloned().collect();
    let closers: HashMap<char, char> = openers.iter()
        .map(|(k,v)| (*v,*k))
        .collect();
    let corruption_scoreboard: HashMap<char, u64> = [
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ].iter().cloned().collect();
    let incomplete_scoreboard: HashMap<char, u64> = [
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ].iter().cloned().collect();
    let mut corruption_score = 0u64;
    let mut incomplete_scores = Vec::<u64>::new();
    let mut index = 0usize;

    for line in lines {
        let mut state = Vec::<char>::new();
        let mut incomplete = true;

        for c in line.chars() {
            let opener = openers.get(&c);

            if opener.is_some() { state.push(*opener.unwrap()); continue; }

            let closer = closers.get(&c);

            if closer.is_some() {
                let expected = state.pop().unwrap();

                if expected == c { continue; }
                
                corruption_score += corruption_scoreboard.get(&c).unwrap();
                incomplete = false;
                break;
            }
        }

        if !incomplete { continue; }

        let mut score = 0u64;

        while state.len() > 0 {
            let closer = state.pop().unwrap();
            score = score*5 + incomplete_scoreboard.get(&closer).unwrap();
        }

        incomplete_scores.push(score);
    }

    incomplete_scores.sort();

    (corruption_score, incomplete_scores[incomplete_scores.len()/2])
}

fn part1() {
    if let Ok(lines) = read_nav() {
        println!("{}", check_syntax(&lines).0);
    }
    else {
        panic!("couldn't read nav!");
    }
}

fn part2() {
    if let Ok(lines) = read_nav() {
        println!("{}", check_syntax(&lines).1);
    }
    else {
        panic!("couldn't read nav!");
    }
}

fn main() {
    // part1();
    part2();
}
