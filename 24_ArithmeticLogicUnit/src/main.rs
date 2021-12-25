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
    fn is_peek_block(&self) -> bool {
        self.instructions.iter()
            .filter(|&x| *x == Instruction {
                opcode: Opcode::Div,
                operands: vec![Operand::Register(Register::Z), Operand::Constant(1)],
            })
            .count() == 1
    }
    fn is_pop_block(&self) -> bool {
        self.instructions.iter()
            .filter(|&x| *x == Instruction {
                opcode: Opcode::Div,
                operands: vec![Operand::Register(Register::Z), Operand::Constant(26)],
            })
            .count() == 1
    }
    fn get_input_offset(&self) -> Option<isize> {
        if !self.is_peek_block() { return None; }
        
        let mut instructions = self.instructions.clone();
        instructions.reverse();

        let mut index = 0;

        for i in 0..instructions.len() {
            index = i;

            let instruction = &instructions[i];
            let target = Instruction {
                opcode: Opcode::Add,
                operands: vec![Operand::Register(Register::Y), Operand::Register(Register::W)]
            };

            if *instruction == target {
                break;
            }
        }

        let target = &instructions[index-1];

        if let Operand::Constant(offset) = target.operands[1] {
            Some(offset)
        }
        else {
            None
        }
    }
    fn get_x_offset(&self) -> Option<isize> {
        if !self.is_pop_block() { return None; }
        
        let mut index = 0;

        for i in 0..self.instructions.len() {
            index = i;

            let instruction = &self.instructions[i];
            let target = Instruction {
                opcode: Opcode::Div,
                operands: vec![Operand::Register(Register::Z), Operand::Constant(26)],
            };
            
            if *instruction == target {
                break;
            }
        }

        let target = &self.instructions[index+1];

        if let Operand::Constant(offset) = target.operands[1] {
            Some(offset)
        }
        else {
            None
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Keygen {
    serial: HashMap<usize, (isize, isize)>,
    links: HashMap<usize, usize>,
}
impl Keygen {
    fn generate(blocks: &Vec<Block>) -> Self {
        let mut serial = HashMap::<usize, (isize, isize)>::new();
        let mut links = HashMap::<usize, usize>::new();
        let mut offset_stack = Vec::<(usize, isize)>::new();

        for block_id in 0..blocks.len() {
            /* if peek block:
             *     push the serial number id and offset onto the stack
             * else: // pop block
             *     pop the id and offset
             *     get the x delta and add it to the offset
             *     get the range of values which equal 1 thru 9 when added to that delta
             *     mark the left-hand id as solved with the left-hand range
             *     mark the right-hand id as solved with the right-hand range
             *     mark left and right ids as linked
             */

            let block = &blocks[block_id];
            
            if block.is_peek_block() {
                let input_offset = block.get_input_offset().unwrap();
                offset_stack.push((block_id, input_offset));
                continue;
            }

            let (popped_id, popped_offset) = offset_stack.pop().unwrap();
            let x_offset = block.get_x_offset().unwrap();
            let offset_result = popped_offset + x_offset;
            let (mut low_end_left, mut low_end_right) = (isize::MAX, isize::MAX);
            let (mut high_end_left, mut high_end_right) = (isize::MIN, isize::MIN);

            for i in 1..=9 {
                let value = i+offset_result;

                if value < 1 || value > 9 { continue; }

                if i < low_end_left { low_end_left = i; }
                if i > high_end_left { high_end_left = i; }

                if value < low_end_right { low_end_right = value; }
                if value > high_end_right { high_end_right = value; }
            }

            serial.insert(popped_id, (low_end_left, high_end_left));
            serial.insert(block_id, (low_end_right, high_end_right));
            
            links.insert(popped_id, block_id);
            links.insert(block_id, popped_id);
        }

        Self { serial, links }
    }
    fn get_lower_serial(&self) -> Vec<isize> {
        let mut result = vec![0isize; 14];

        for (index, range) in &self.serial {
            result[*index] = range.0;
        }

        result
    }
    fn get_upper_serial(&self) -> Vec<isize> {
        let mut result = vec![0isize; 14];

        for (index, range) in &self.serial {
            result[*index] = range.1;
        }

        result
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
        let keygen = Keygen::generate(&blocks);
        let mut state = State::new(&keygen.get_upper_serial());
        keygen.get_upper_serial().iter().for_each(|x| print!("{}", x));
        println!("");

        for block in &blocks {
            block.execute(&mut state);
        }

        assert!(state.get_register(Register::Z) == 0);
    }
    else { panic!("couldn't read instructions!"); }
}

fn part2() {
    if let Ok(blocks) = read_instructions() {
        let keygen = Keygen::generate(&blocks);
        let mut state = State::new(&keygen.get_lower_serial());
        keygen.get_lower_serial().iter().for_each(|x| print!("{}", x));
        println!("");

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
