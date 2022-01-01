use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(isize),
}
impl Instruction {
    fn from_string(s: &String) -> Self {
        let chunks: Vec<&str> = s.split(" ").collect();

        match chunks[0] {
            "nop" => Self::Nop(chunks[1].parse().unwrap()),
            "jmp" => Self::Jmp(chunks[1].parse().unwrap()),
            "acc" => Self::Acc(chunks[1].parse().unwrap()),
            _ => panic!("bad instruction: {}", chunks[0]),
        }
    }
}

type Address = usize;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Loop,
    EOF,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Program {
    instructions: Vec<Instruction>,
    address: Address,
    accumulator: isize,
}
impl Program {
    fn new(instructions: &Vec<Instruction>) -> Self {
        Self {
            instructions: instructions.clone(),
            address: 0,
            accumulator: 0,
        }
    }
    fn get_instruction(&self) -> Result<Instruction, ()> {
        if self.address < self.instructions.len() { Ok(self.instructions[self.address]) }
        else { Err(()) }
    }
    fn advance(&mut self) -> Result<usize, ()> {
        self.address += 1;

        if self.address >= self.instructions.len() { Err(()) }
        else { Ok(self.address) }
    }
    fn execute(&mut self) -> State {
        let mut visits = HashMap::<Address, usize>::new();

        loop {
            if self.address >= self.instructions.len() { return State::EOF; }
            
            let visit = visits.entry(self.address).or_insert(0);
            *visit += 1;

            if *visit > 1 { return State::Loop; }
            
            if let Ok(instruction) = self.get_instruction() {
                match instruction {
                    Instruction::Nop(_) => { if self.advance().is_err() { return State::EOF; } },
                    Instruction::Jmp(offset) => {
                        let result = self.address as isize + offset;
                        self.address = result as usize;
                    },
                    Instruction::Acc(offset) => {
                        self.accumulator += offset;
                        if self.advance().is_err() { return State::EOF; }
                    },
                }
            }
            else { return State::EOF; }
        }
    }
    fn patch(&self) -> Program {
        for address in 0..self.instructions.len() {
            let patch_inst;
            
            match self.instructions[address] {
                Instruction::Acc(_) => continue,
                Instruction::Nop(_) => patch_inst = address,
                Instruction::Jmp(_) => patch_inst = address,
            }

            let mut new_instructions = self.instructions.clone();

            match self.instructions[patch_inst] {
                Instruction::Acc(_) => (),
                Instruction::Nop(offset) => new_instructions[address] = Instruction::Jmp(offset),
                Instruction::Jmp(offset) => new_instructions[address] = Instruction::Nop(offset),
            }

            let mut program = Program::new(&new_instructions);

            if program.execute() == State::EOF { return program; }
        }

        panic!("couldn't patch program!");
    }
}

fn read_program() -> Result<Program, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut instructions = Vec::<Instruction>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        instructions.push(Instruction::from_string(&buffer.trim().replace("+","")));
        buffer.clear();
    }

    if instructions.len() == 0 { Err(()) }
    else { Ok(Program::new(&instructions)) }
}

fn part1() {
    if let Ok(mut program) = read_program() {
        program.execute();
        
        println!("{}", program.accumulator);
    }
    else { panic!("couldn't read program!"); }
}

fn part2() {
    if let Ok(mut program) = read_program() {
        let mut patched = program.patch();
        
        println!("{}", patched.accumulator);
    }
    else { panic!("couldn't read program!"); }
}

fn main() {
    // part1();
    part2();
}
