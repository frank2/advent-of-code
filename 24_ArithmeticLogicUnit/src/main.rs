use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Opcode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}
impl Opcode {
    fn from_str(s: &str) -> Self {
        match s {
            "inp" => Self::Inp,
            "add" => Self::Add,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "mod" => Self::Mod,
            "eql" => Self::Eql,
            _ => panic!("bad opcode"),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Inp => "inp".to_string(),
            Self::Add => "add".to_string(),
            Self::Mul => "mul".to_string(),
            Self::Div => "div".to_string(),
            Self::Mod => "mod".to_string(),
            Self::Eql => "eql".to_string(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Register {
    W,
    X,
    Y,
    Z,
}
impl Register {
    fn from_str(s: &str) -> Self {
        match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => panic!("bad register"),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::W => "w".to_string(),
            Self::X => "x".to_string(),
            Self::Y => "y".to_string(),
            Self::Z => "z".to_string(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    registers: HashMap<Register, isize>,
    tape: Vec<isize>,
}
impl State {
    fn new(tape: &Vec<isize>) -> Self {
        Self {
            registers: [
            (Register::W, 0),
            (Register::X, 0),
            (Register::Y, 0),
            (Register::Z, 0),
            ].iter().copied().collect(),
            
            tape: tape.clone()
        }
    }
    fn to_string(&self) -> String {
        format!("[w={},x={},y={},z={},tape={:?}]",
                self.get_register(Register::W),
                self.get_register(Register::X),
                self.get_register(Register::Y),
                self.get_register(Register::Z),
                self.tape.clone()
        )
    }
    fn get_register(&self, r: Register) -> isize {
        *self.registers.get(&r).unwrap()
    }
    fn set_register(&mut self, r: Register, v: isize) {
        self.registers.insert(r, v);
    }
    fn read_tape(&mut self) -> Option<isize> {
        if self.tape.len() == 0 { None }
        else {
            let result = self.tape[0];
            self.tape.remove(0);

            Some(result)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Operand {
    Constant(isize),
    Register(Register),
}
impl Operand {
    fn from_str(s: &str) -> Self {
        if let Ok(value) = s.parse::<isize>() { Self::Constant(value) }
        else { Self::Register(Register::from_str(s)) }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Constant(s) => format!("{}", s),
            Self::Register(r) => r.to_string(),
        }
    }
    fn get_value(&self, state: &State) -> isize {
        match self {
            Self::Constant(s) => *s,
            Self::Register(r) => state.get_register(*r),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Instruction {
    opcode: Opcode,
    operands: Vec<Operand>,
}
impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut chunks: Vec<&str> = s.split(" ").collect();
        let opcode = Opcode::from_str(chunks[0]);
        let mut operands = Vec::<Operand>::new();

        chunks.remove(0);

        for chunk in &chunks {
            operands.push(Operand::from_str(chunk));
        }

        Self { opcode, operands }
    }
    fn to_string(&self) -> String {
        if self.operands.len() == 1 {
            format!("{} {}", self.opcode.to_string(), self.operands[0].to_string())
        }
        else {
            format!("{} {},{}", self.opcode.to_string(), self.operands[0].to_string(), self.operands[1].to_string())
        }
    }
    fn execute(&self, state: &mut State) {
        match self.opcode {
            Opcode::Inp => {
                if let Operand::Register(reg) = self.operands[0] {
                    if let Some(tape) = state.read_tape() {
                        state.set_register(reg, tape);
                    }
                    else { panic!("no input to read!"); }
                }
                else { panic!("can't get input of constant"); }
            },
            Opcode::Add => {
                if let Operand::Register(reg) = self.operands[0] {
                    let right_side = self.operands[1].get_value(state);
                    state.set_register(reg, state.get_register(reg)+right_side);
                }
                else { panic!("cannot store in constant"); }
            },
            Opcode::Mul => {
                if let Operand::Register(reg) = self.operands[0] {
                    let right_side = self.operands[1].get_value(state);
                    state.set_register(reg, state.get_register(reg)*right_side);
                }
                else { panic!("cannot store in constant"); }
            },
            Opcode::Div => {
                if let Operand::Register(reg) = self.operands[0] {
                    let right_side = self.operands[1].get_value(state);
                    state.set_register(reg, state.get_register(reg)/right_side);
                }
                else { panic!("cannot store in constant"); }
            },
            Opcode::Mod => {
                if let Operand::Register(reg) = self.operands[0] {
                    let right_side = self.operands[1].get_value(state);
                    state.set_register(reg, state.get_register(reg)%right_side);
                }
                else { panic!("cannot store in constant"); }
            },
            Opcode::Eql => {
                if let Operand::Register(reg) = self.operands[0] {
                    let right_side = self.operands[1].get_value(state);
                    state.set_register(reg, (state.get_register(reg)==right_side) as isize);
                }
                else { panic!("cannot store in constant!"); }
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Block {
    instructions: Vec<Instruction>,
}
impl Block {
    fn new() -> Self { Self { instructions: Vec::<Instruction>::new() } }
    fn push(&mut self, instruction: &Instruction) {
        self.instructions.push(instruction.clone());
    }
    fn execute(&self, state: &mut State) {
        self.instructions.iter().for_each(|x| x.execute(state));
        
        /*
        for instruction in &self.instructions {
            let inst = instruction.to_string();
            
            instruction.execute(state);

            let st = state.to_string();
            
            println!("{}{}{}",
                     inst,
                     std::str::from_utf8(&vec![' ' as u8; 10-inst.len()]).unwrap(),
                     st);
        }

        println!("");
         */
    }
}

fn read_instructions() -> Result<Vec<Block>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut blocks = Vec::<Block>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        let instruction = Instruction::from_str(buffer.trim());

        if instruction.opcode == Opcode::Inp {
            blocks.push(Block::new());
        }

        let index = blocks.len()-1;
        blocks[index].push(&instruction);
        
        buffer.clear();
    }

    if blocks.len() == 0 { Err(()) }
    else { Ok(blocks) }
}

fn part1() {
    if let Ok(blocks) = read_instructions() {
        let mut state = State::new(&vec![9,6,9,2,9,9,9,4,2,9,3,9,9,6]);

        for block in &blocks {
            block.execute(&mut state);
        }

        assert!(state.get_register(Register::Z) == 0);
    }
    else { panic!("couldn't read instructions!"); }
}

fn part2() {
    if let Ok(blocks) = read_instructions() {
        let mut state = State::new(&vec![4,1,8,1,1,7,6,1,1,8,1,1,4,1]);

        for block in &blocks {
            block.execute(&mut state);
        }

        assert!(state.get_register(Register::Z) == 0);
    }
    else { panic!("couldn't read instructions!"); }
}

fn main() {
    // part1();
    part2();
}
