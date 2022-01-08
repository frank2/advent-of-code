use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Bitmask {
    mask: usize,
    value: usize,
    address: usize,
}
impl Bitmask {
    fn new() -> Self {
        Self { mask: 0, value: 0, address: 0 }
    }
    fn from_str(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();

        let mut mask = 0usize;
        let mut value = 0usize;
        let mut address = 0usize;

        for i in 0..chars.len() {
            mask <<= 1;
            value <<= 1;
            address <<= 1;
            
            match chars[i] {
                '0' => { mask |= 1; },
                '1' => { value |= 1; mask |= 1; },
                'X' => { address |= 1; },
                _ => panic!("bad bitmask character: {}", chars[i]),
            }
        }

        Self { mask, value, address }
    }
    fn mask(&self, v: usize) -> usize {
        (v & !self.mask) | self.value
    }
    
    fn address_offsets(&self) -> Vec<usize> {
        let mut result = Vec::<usize>::new();
        let mut address = self.address;
        let mut offset = 0usize;

        while address > 0 {
            if address & 1 == 1 { result.push(offset); }
            
            address >>= 1;
            offset += 1;
        }

        result
    }
    fn address_max(&self) -> usize {
        2usize.pow(self.address_offsets().len() as u32)
    }
    fn address_index(&self, value: usize) -> usize {
        let max = self.address_max();

        if value >= max { panic!("value out of range: {}", value); }

        let mut offsets = self.address_offsets();
        let mut bits = value;
        let mut result = 0usize;
        
        offsets.reverse();

        while let Some(index) = offsets.pop() {
            result |= (bits & 1) << index;
            bits >>= 1;
        }

        result
    }
    fn write(&self, address: usize, value: usize, memory: &mut HashMap<usize, usize>) {
        let base_address = (address | self.value) & !self.address;

        for index in 0..self.address_max() {
            memory.insert(base_address | self.address_index(index), value);
        }
    }
}

fn read_program_p1() -> Result<(Bitmask, HashMap<String, usize>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut bitmask = Bitmask::new();
    let mut memory = HashMap::<String, usize>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        let chunks: Vec<&str> = buffer.trim().split(" = ").collect();

        if chunks[0] == "mask" { bitmask = Bitmask::from_str(chunks[1]); }
        else { memory.insert(chunks[0].to_string(), bitmask.mask(chunks[1].parse().unwrap())); }

        buffer.clear();
    }

    if memory.len() == 0 { Err(()) }
    else { Ok((bitmask, memory)) }
}

fn read_program_p2() -> Result<(Bitmask, HashMap<usize, usize>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut bitmask = Bitmask::new();
    let mut memory = HashMap::<usize, usize>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        let chunks: Vec<&str> = buffer.trim().split(" = ").collect();

        if chunks[0] == "mask" { bitmask = Bitmask::from_str(chunks[1]); }
        else {
            let address = chunks[0].replace("mem[", "").replace("]","").parse::<usize>().unwrap();
            bitmask.write(address, chunks[1].parse().unwrap(), &mut memory);
        }
            
        buffer.clear();
    }

    if memory.len() == 0 { Err(()) }
    else { Ok((bitmask, memory)) }
}

fn part1() {
    if let Ok((bitmask, memory)) = read_program_p1() {
        println!("{}", memory.values().sum::<usize>());
    }
    else { panic!("couldn't read program!"); }
}

fn part2() {
    if let Ok((bitmask, memory)) = read_program_p2() {
        println!("{}", memory.values().sum::<usize>());
    }
    else { panic!("couldn't read program!"); }
}

fn main() {
    // part1();
    part2();
}
