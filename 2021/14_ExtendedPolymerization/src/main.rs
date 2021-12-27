use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PolymerPair(char,char);
impl PolymerPair {
    fn from_str(s: &str) -> Self {
        let cv: Vec<char> = s.chars().collect();

        Self(cv[0],cv[1])
    }
    fn insert(&self, c: char) -> (PolymerPair, PolymerPair) {
        (Self(self.0, c), Self(c, self.1))
    }
}

fn read_polymers() -> Result<(HashMap<PolymerPair, usize>, HashMap<PolymerPair, char>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut polymer_pairs = Vec::<PolymerPair>::new();
    let mut polymer_count = HashMap::<PolymerPair, usize>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { break; }
        
        let char_vec: Vec<char> = buffer.trim().chars().collect();

        for i in 0..char_vec.len()-1 {
            polymer_pairs.push(PolymerPair(char_vec[i],char_vec[i+1]));
        }

        buffer.clear();
    }

    if polymer_pairs.len() == 0 { return Err(()); }

    for polymer in polymer_pairs {
        let value = polymer_count.get(&polymer);

        if value.is_none() { polymer_count.insert(polymer, 1); }
        else { polymer_count.insert(polymer, value.unwrap()+1); }
    }

    let mut polymer_rules = HashMap::<PolymerPair, char>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        let rule_pair: Vec<&str> = buffer.trim().split(" -> ").collect();
        let polymer = PolymerPair::from_str(rule_pair[0]);
        let rule = rule_pair[1].chars().next().unwrap();

        polymer_rules.insert(polymer, rule);
        buffer.clear();
    }

    Ok((polymer_count, polymer_rules))
}

fn evolve(rules: &HashMap<PolymerPair, char>, count: &HashMap<PolymerPair, usize>) -> HashMap<PolymerPair, usize> {
    let mut new_count = HashMap::<PolymerPair, usize>::new();
        
    for (polymer, occurences) in count {
        let (left, right) = polymer.insert(*rules.get(&polymer).unwrap());

        let left_value = new_count.get(&left);

        if left_value.is_none() { new_count.insert(left, *occurences); }
        else { new_count.insert(left, left_value.unwrap()+*occurences); }

        let right_value = new_count.get(&right);
        
        if right_value.is_none() { new_count.insert(right, *occurences); }
        else { new_count.insert(right, right_value.unwrap()+*occurences); }
    }

    new_count
}

fn iterate(rules: &HashMap<PolymerPair, char>, count: &HashMap<PolymerPair, usize>, steps: usize) -> usize {
    let mut result = count.clone();

    for _ in 0..steps {
        result = evolve(rules, &result);
    }

    let mut letter_count = HashMap::<char, usize>::new();

    for (pair, count) in result {
        let value = letter_count.get(&pair.1);

        if value.is_none() { letter_count.insert(pair.1, count); }
        else { letter_count.insert(pair.1, value.unwrap()+count); }
    }

    let mut min = usize::MAX;
    let mut max = 0;

    for (_, count) in letter_count {
        if count <= min { min = count; }
        if count >= max { max = count; }
    }

    (max - min)+1
}

fn part1() {
    if let Ok((count, rules)) = read_polymers() {
        println!("{}", iterate(&rules, &count, 10));
    }
    else { panic!("couldn't read polymers!"); }
}

fn part2() {
    if let Ok((count, rules)) = read_polymers() {
        println!("{}", iterate(&rules, &count, 40));
    }
    else { panic!("couldn't read polymers!"); }
}

fn main() {
    // part1();
    part2();
}
