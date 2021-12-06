use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Frequency(u32, u32);
impl Frequency {
    pub fn new() -> Self { Self(0,0) }
    pub fn gamma(&self) -> u8 { if self.0 > self.1 { 0 } else { 1 } }
    pub fn epsilon(&self) -> u8 { if self.0 > self.1 { 1 } else { 0 } }
}

fn read_bitvec() -> Result<Vec<u8>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let result = stdin.read_line(&mut buffer);

    if result.is_err() { return Err(()); }

    // convert the string into an array of bit values
    let bitvec: Vec<u8> = buffer.trim()
        .chars()
        .map(|x| x as u8 - '0' as u8)
        .collect();

    if bitvec.len() == 0 { Err(()) }
    else { Ok(bitvec) }
}

fn bitvec_frequency(freq: &mut Vec::<Frequency>, bitvec: &Vec<u8>) {
    if freq.len() == 0 {
        for _ in 0..bitvec.len() { freq.push(Frequency::new()) }
    }

    for i in 0..bitvec.len() {
        let value = bitvec[i];

        match value {
            0 => freq[i].0 += 1,
            1 => freq[i].1 += 1,
            _ => (),
        }
    }
}

fn bits_to_u32(v: &Vec<u8>) -> u32 {
    v.iter().fold(0, |acc, x| (acc << 1) | (*x as u32))
}

fn life_support_rating(bitvecs: &Vec<Vec<u8>>, freq: &Vec<Frequency>, rating: bool) -> Result<u32, ()> {
    let mut selections = bitvecs.clone();
    let mut reduced_freq = freq.clone();
    let mut index = 0usize;

    loop {
        let check;
        
        if rating { check = reduced_freq[index].gamma(); }
        else { check = reduced_freq[index].epsilon(); }

        selections = selections.iter()
            .filter(|x| x[index] == check)
            .cloned()
            .collect();

        if selections.len() == 1 { return Ok(bits_to_u32(&selections[0])); }
        else {
            index += 1;
            if index == freq.len() { return Err(()); }
        }

        reduced_freq = Vec::<Frequency>::new();

        for i in 0..selections.len() {
            bitvec_frequency(&mut reduced_freq, &selections[i]);
        }
    }
}

fn part1() {
    let mut freq = Vec::<Frequency>::new();

    loop {
        let result = read_bitvec();

        if result.is_err() { break; }

        let bitvec = result.unwrap();
        
        bitvec_frequency(&mut freq, &bitvec);
    }

    let gamma_value = bits_to_u32(&freq.iter().map(|x| x.gamma()).collect());
    let epsilon_value = bits_to_u32(&freq.iter().map(|x| x.epsilon()).collect());

    println!("{}", gamma_value * epsilon_value);
}

fn part2() {
    let mut freq = Vec::<Frequency>::new();
    let mut bitvecs = Vec::<Vec<u8>>::new();

    loop {
        let result = read_bitvec();

        if result.is_err() { break; }

        let bitvec = result.unwrap();
        
        bitvec_frequency(&mut freq, &bitvec);
        bitvecs.push(bitvec);
    }

    let gamma_value = life_support_rating(&bitvecs, &freq, true).unwrap();
    let epsilon_value = life_support_rating(&bitvecs, &freq, false).unwrap();

    println!("{}", gamma_value * epsilon_value);
}

fn main() {
    // part1();
    part2();
}
