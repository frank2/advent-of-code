use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pattern {
    segments: String
}
impl Pattern {
    pub fn from_str(s: &str) -> Self {
        let mut segments: Vec<u8> = s.as_bytes().iter().cloned().collect();
        segments.sort();
        
        Self { segments: std::str::from_utf8(&segments).unwrap().to_string() } 
    }
    pub fn as_set(&self) -> HashSet<u8> {
        self.segments.as_str().as_bytes().iter().cloned().collect()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Signal {
    patterns: Vec<Pattern>,
    output: Vec<Pattern>,
}
impl Signal {
    pub fn pattern_map(&self) -> HashMap<String, u32> {
        let mut digits = HashMap::<u32, Pattern>::new();
        let mut five_segments = Vec::<Pattern>::new();
        let mut six_segments = Vec::<Pattern>::new();

        // first, get the known patterns based on length
        for pattern in &self.patterns {
            match pattern.segments.len() {
                2 => { digits.insert(1, pattern.clone()); },
                3 => { digits.insert(7, pattern.clone()); },
                4 => { digits.insert(4, pattern.clone()); },
                7 => { digits.insert(8, pattern.clone()); },
                5 => { five_segments.push(pattern.clone()); },
                6 => { six_segments.push(pattern.clone()); },
                _ => (),
            }
        }

        let one_set = digits.get(&1).unwrap().as_set();
        let four_set = digits.get(&4).unwrap().as_set();

        // infer the six-segment displays
        for pattern in six_segments {
            let target_set = pattern.as_set();
            let four_intersection: HashSet<u8> = target_set.intersection(&four_set).map(|&x| x).collect();
            let one_intersection: HashSet<u8> = target_set.intersection(&one_set).map(|&x| x).collect();

            if four_intersection.len() == four_set.len() {
                digits.insert(9, pattern.clone());
            }
            else if one_intersection.len() == one_set.len() && four_intersection.len() != four_set.len() {
                digits.insert(0, pattern.clone());
            }
            else {
                digits.insert(6, pattern.clone());
            }
        }

        let six_set = digits.get(&6).unwrap().as_set();

        // infer the five-segment displays
        for pattern in five_segments {
            let target_set = pattern.as_set();
            let six_intersection: HashSet<u8> = target_set.intersection(&six_set).map(|&x| x).collect();
            let one_intersection: HashSet<u8> = target_set.intersection(&one_set).map(|&x| x).collect();

            if six_intersection.len() == target_set.len() {
                digits.insert(5, pattern.clone());
            }
            else if one_intersection.len() == one_set.len() {
                digits.insert(3, pattern.clone());
            }
            else {
                digits.insert(2, pattern.clone());
            }
        }

        digits.iter().map(|(k,v)| (v.segments.clone(), *k)).collect()
    }
    pub fn readout(&self) -> u32 {
        let pattern_map = self.pattern_map();
        let mut result = 0u32;

        for digit in &self.output {
            result *= 10;
            result += pattern_map.get(&digit.segments).unwrap();
        }

        result
    }
}

fn read_signals() -> Result<Vec<Signal>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut signals = Vec::<Signal>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        let signal_data: Vec<&str> = buffer.trim().split(" | ").collect();
        let pattern_segments: Vec<Pattern> = signal_data[0].split(" ")
            .map(|x| Pattern::from_str(x))
            .collect();
        let output_segments: Vec<Pattern> = signal_data[1].split(" ")
            .map(|x| Pattern::from_str(x))
            .collect();

        signals.push(Signal { patterns: pattern_segments, output: output_segments });

        buffer.clear();
    }

    if signals.len() == 0 { return Err(()) }
    else { Ok(signals) }
}

fn part1() {
    if let Ok(signals) = read_signals() {
        let lengths: HashSet<usize> = [2usize, 3, 4, 7].iter().cloned().collect();
        let mut count = 0u32;

        for signal in &signals {
            for pattern in &signal.output {
                if lengths.contains(&pattern.segments.len()) { count += 1; }
            }
        }

        println!("{}", count);
    }
}
    
fn part2() {
    if let Ok(signals) = read_signals() {
        println!("{}", signals.iter().map(|x| x.readout()).sum::<u32>());
    }
    else { panic!("couldn't read signals!"); }
}

fn main() {
    // part1();
    part2();
}
