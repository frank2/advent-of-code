use std::io;
use std::collections::HashMap;

type RuleID = usize;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Rule {
    Constant(String),
    Ruleset(Vec<RuleID>),
    Branch(Vec<RuleID>,Vec<RuleID>),
}
impl Rule {
    fn from_str(s: &str) -> (RuleID, Self) {
        let chunks: Vec<&str> = s.split(": ").collect();
        let rule_id: usize = chunks[0].parse().unwrap();

        if chunks[1].contains("\"") {
            return (rule_id, Self::Constant(chunks[1].replace("\"", "")));
        }
        else if chunks[1].contains("|") {
            let rules: Vec<&str> = chunks[1].split(" | ").collect();
            let left_rules: Vec<RuleID> = rules[0].split(" ").map(|x| x.parse().unwrap()).collect();
            let right_rules: Vec<RuleID> = rules[1].split(" ").map(|x| x.parse().unwrap()).collect();

            return (rule_id, Self::Branch(left_rules,right_rules));
        }

        let rules: Vec<RuleID> = chunks[1].split(" ").map(|x| x.parse().unwrap()).collect();

        (rule_id, Self::Ruleset(rules))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Engine {
    rules: HashMap<RuleID, Rule>,
}
impl Engine {
    fn new() -> Self {
        Self { rules: HashMap::<RuleID, Rule>::new() }
    }
    fn add_rule(&mut self, id: RuleID, rule: &Rule) {
        self.rules.insert(id, rule.clone());
    }
    fn get_rule(&self, id: RuleID) -> &Rule {
        self.rules.get(&id).unwrap()
    }
    fn matches(&self, string: &String) -> bool {
        let mut stack = Vec::<(Vec<Vec<RuleID>>, usize)>::new();

        stack.push((vec![vec![0 as RuleID]], 0));

        while let Some((mut rule_stack, mut consumed)) = stack.pop() {
            if consumed >= string.len() && rule_stack.len() > 0 { continue; }
            else if rule_stack.len() == 0 {
                if consumed == string.len() { return true; }
                else { continue; }
            }
            
            let last_rules = rule_stack.len()-1;
            let rules = &mut rule_stack[last_rules];
            let rule_id = rules[0];
            rules.remove(0);

            if rules.len() == 0 { rule_stack.pop(); }

            match self.get_rule(rule_id) {
                Rule::Constant(s) => {
                    if *s != string[consumed..consumed+s.len()] { continue; }

                    consumed += s.len();
                    stack.push((rule_stack, consumed));
                },
                Rule::Ruleset(r) => {
                    rule_stack.push(r.clone());
                    stack.push((rule_stack, consumed));
                },
                Rule::Branch(left,right) => {
                    let mut left_stack = rule_stack.clone();
                    let mut right_stack = rule_stack.clone();

                    left_stack.push(left.clone());
                    right_stack.push(right.clone());

                    stack.push((right_stack, consumed));
                    stack.push((left_stack, consumed));
                },
            }
        }

        false
    }
}

fn read_rules() -> Result<(Engine, Vec<String>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut engine = Engine::new();
    let mut strings = Vec::<String>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { break; }
        
        let (rule_id, rule) = Rule::from_str(buffer.trim());
        engine.add_rule(rule_id, &rule);
        
        buffer.clear();
    }

    if engine.rules.len() == 0 { return Err(()); }

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        strings.push(buffer.trim().to_string());
        
        buffer.clear();
    }

    if strings.len() == 0 { Err(()) }
    else { Ok((engine, strings)) }
}

fn main() {
    if let Ok((engine, strings)) = read_rules() {
        println!("{}", strings.iter().map(|x| engine.matches(x)).filter(|x| *x == true).count());
    }
    else { panic!("couldn't read rules!"); }
}
