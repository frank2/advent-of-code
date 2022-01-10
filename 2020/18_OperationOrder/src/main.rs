use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Operand {
    Constant(usize),
    Expression(Box<Expression>),
}
impl Operand {
    fn eval_p1(&self) -> usize {
        match self {
            Self::Constant(u) => *u,
            Self::Expression(e) => e.eval_p1(),
        }
    }
    fn eval_p2(&self) -> usize {
        match self {
            Self::Constant(u) => *u,
            Self::Expression(e) => e.eval_p2(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Expression {
    operands: Vec<Operand>,
    operators: Vec<Operator>,
}
impl Expression {
    fn new() -> Self {
        Self { operands: Vec::<Operand>::new(), operators: Vec::<Operator>::new(), }
    }
    fn from_str(s: &str) -> Self {
        let mut tokens: Vec<char> = s.chars().rev().collect();
        let mut expression_stack = Vec::<Expression>::new();

        expression_stack.push(Expression::new());

        while let Some(token) = tokens.pop() {
            let mut expression_id = expression_stack.len()-1;
                     
            if token == ' ' {
                continue;
            }
            else if token == '+' || token == '*' {
                let expression = &mut expression_stack[expression_id];
                
                if token == '+' { expression.operators.push(Operator::Addition); }
                else if token == '*' { expression.operators.push(Operator::Multiplication); }
            }
            else if token == '(' {
                expression_stack.push(Expression::new());
            }
            else if token == ')' {
                let subexpression = expression_stack.pop().unwrap();
                expression_id = expression_stack.len()-1;
                
                let expression = &mut expression_stack[expression_id];
                expression.operands.push(Operand::Expression(Box::new(subexpression.clone())));
            }
            else if token as usize >= '0' as usize && token as usize <= '9' as usize {
                let digit = token as usize - '0' as usize;
                let operand = Operand::Constant(digit);
                let expression = &mut expression_stack[expression_id];

                expression.operands.push(operand);
            }
            else { panic!("bad token: {}", token); }
        }

        expression_stack.pop().unwrap()
    }
    fn eval_p1(&self) -> usize {
        let mut result = 0usize;
        if self.operands.len() == 0 { return result; }

        result = self.operands[0].eval_p1();

        for i in 0..self.operators.len() {
            match self.operators[i] {
                Operator::Addition => { result += self.operands[i+1].eval_p1(); },
                Operator::Multiplication => { result *= self.operands[i+1].eval_p1(); },
            }
        }

        result
    }
    fn eval_p2(&self) -> usize {
        if self.operands.len() == 0 { return 0; }

        let mut operands = self.operands.clone();
        let mut operators = self.operators.clone();

        while operators.contains(&Operator::Addition) {
            let index = (0..operators.len()).find(|&x| operators[x] == Operator::Addition).unwrap();

            let op1 = operands[index].eval_p2();
            let op2 = operands[index+1].eval_p2();
            let result = op1+op2;

            operands.remove(index);
            operands.remove(index);
            operators.remove(index);

            operands.insert(index, Operand::Constant(result));
        }
        
        operands.iter().map(|x| x.eval_p2()).product()
    }
}

fn read_expressions() -> Result<Vec<Expression>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut expressions = Vec::<Expression>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        expressions.push(Expression::from_str(buffer.trim()));
        buffer.clear();
    }

    if expressions.len() == 0 { Err(()) }
    else { Ok(expressions) }
}
        
fn part1() {
    if let Ok(expressions) = read_expressions() {
        println!("{}", expressions.iter().map(|x| x.eval_p1()).sum::<usize>());
    }
    else { panic!("couldn't read expressions!"); }
}

fn part2() {
    if let Ok(expressions) = read_expressions() {
        println!("{}", expressions.iter().map(|x| x.eval_p2()).sum::<usize>());
    }
    else { panic!("couldn't read expressions!"); }
}

fn main() {
    // part1();
    part2();
}
