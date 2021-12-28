use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Range(usize,usize);
impl Range {
    fn from_str(s: &str) -> Self {
        let chunks = s.split("-").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();

        Self(chunks[0],chunks[1])
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PolicyCheck {
    range: Range,
    character: char,
    password: String,
}
impl PolicyCheck {
    fn from_str(s: &str) -> Self {
        let first_split: Vec<&str> = s.split(": ").collect();
        let second_split: Vec<&str> = first_split[0].split(" ").collect();

        let range = Range::from_str(second_split[0]);
        let character = second_split[1].chars().next().unwrap();
        let password = first_split[1].to_string();

        Self { range, character, password }
    }
    fn check_p1(&self) -> bool {
        let mut char_map = HashMap::<char,usize>::new();

        self.password.chars().for_each(|x| *char_map.entry(x).or_insert(0) += 1);
        let count = char_map.get(&self.character);

        if count.is_none() { false }
        else { let result = *count.unwrap(); self.range.0 <= result && result <= self.range.1 }
    }
    fn check_p2(&self) -> bool {
        let charset: Vec<char> = self.password.chars().collect();
        let (o1, o2) = (self.range.0-1,self.range.1-1);

        (charset[o1] == self.character || charset[o2] == self.character) && charset[o1] != charset[o2]
    }
}

fn read_policies() -> Result<Vec<PolicyCheck>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut policies = Vec::<PolicyCheck>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        policies.push(PolicyCheck::from_str(buffer.trim()));
        buffer.clear();
    }

    if policies.len() == 0 { Err(()) }
    else { Ok(policies) }
}

fn part1() {
    if let Ok(policies) = read_policies() {
        println!("{}", policies.iter().filter(|x| x.check_p1()).count());
    }
    else { panic!("couldn't read policies!"); }
}

fn part2() {
    if let Ok(policies) = read_policies() {
        println!("{}", policies.iter().filter(|x| x.check_p2()).count());
    }
    else { panic!("couldn't read policies!"); }
}

fn main() {
    // part1();
    part2();
}
