use std::io;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Window {
    size: usize,
    values: Vec<usize>,
    sums: HashMap<usize, usize>,
}
impl Window {
    fn new(size: usize) -> Self {
        Self { size: size, values: Vec::<usize>::new(), sums: HashMap::<usize, usize>::new() }
    }
    fn validate(&self, value: usize) -> bool {
        self.values.len() != self.size || self.sums.get(&value).is_some()
    }
    fn push(&mut self, value: usize) {
        if self.values.len() == self.size {
            let first = self.values[0];

            for index in 1..self.values.len() {
                let second = self.values[index];
                let sum = first+second;
                
                let entry = self.sums.entry(sum).or_insert(0);
                *entry -= 1;

                if *entry == 0 { self.sums.remove(&sum); }
            }

            self.values.remove(0);
        }

        for index in 0..self.values.len() {
            let second = self.values[index];
            let sum = value+second;

            let entry = self.sums.entry(sum).or_insert(0);
            *entry += 1;
        }

        self.values.push(value);
    }
}

fn find_sum(values: &Vec<usize>, target: usize) -> usize {
    let mut low_end = 0usize;
    let mut high_end = 1usize;

    while high_end <= values.len() {
        let mut slice: Vec<usize> = values[low_end..high_end].iter().copied().collect();
        let sum: usize = slice.iter().sum();
            
        if sum == target {
            slice.sort();
            return slice[0] + slice[slice.len()-1];
        }
        else if sum > target {
            low_end += 1;
        }
        else if sum < target {
            high_end += 1;
        }
    }

    panic!("couldn't find sum!");
}

fn read_values() -> Result<Vec<usize>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut values = Vec::<usize>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        values.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    if values.len() == 0 { Err(()) }
    else { Ok(values) }
}
                    
fn part1() {
    if let Ok(values) = read_values() {
        let mut window = Window::new(25);
        
        for value in values {
            if !window.validate(value) { println!("{}", value); break; }
            window.push(value);
        }
    }
    else { panic!("couldn't read values!"); }
}
                    
fn part2() {
    if let Ok(values) = read_values() {
        let mut window = Window::new(25);
        
        for value in &values {
            if !window.validate(*value) { println!("{}", find_sum(&values, *value)); break; }
            window.push(*value);
        }
    }
    else { panic!("couldn't read values!"); }
}

fn main() {
    // part1();
    part2();
}
