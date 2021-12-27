use std::io;
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Debug)]
enum SnailfishNumber {
    Number(usize),
    Pair(Box<SnailfishNumber>,Box<SnailfishNumber>),
}
impl SnailfishNumber {
    fn parse(s: &str) -> Self {
        let mut pending_pairs = Vec::<Vec<SnailfishNumber>>::new();
        
        for c in s.chars() {
            if c == '[' { pending_pairs.push(Vec::<SnailfishNumber>::new()); }
            else if c == ']' {
                let pair = pending_pairs.pop().unwrap();

                if pending_pairs.len() == 0 { return Self::new_pair(&pair[0],&pair[1]); }
                else {
                    let index = pending_pairs.len()-1;
                    pending_pairs[index].push(Self::new_pair(&pair[0],&pair[1]));
                }
            }
            else if c == ',' { continue; }
            else if c.is_digit(10) {
                let index = pending_pairs.len()-1;
                pending_pairs[index].push(Self::new_number(c as usize - '0' as usize));
            }
        }

        panic!("no snail number parsed!");
    }
    fn new_pair(p1: &Self, p2: &Self) -> Self {
        Self::Pair(Box::new(p1.clone()),Box::new(p2.clone()))
    }
    fn new_number(n: usize) -> Self {
        Self::Number(n)
    }
    fn is_number(&self) -> bool {
        if let Self::Number(_) = self { true } else { false }
    }
    fn is_pair(&self) -> bool {
        if let Self::Pair(_,_) = self { true } else { false }
    }
    fn unwrap(&self) -> usize {
        if let Self::Number(n) = self { *n } else { panic!("can't unwrap a pair!"); }
    }
    fn get_rightmost_node(&self) -> &Self {
        let mut start = self;

        while let Self::Pair(_,ref right) = start {
            start = right;
        }

        start
    }
    fn get_leftmost_node(&self) -> &Self {
        let mut start = self;

        while let Self::Pair(ref left, _) = start {
            start = left;
        }

        start
    }
    fn to_string(&self) -> String {
        if let Self::Pair(p1,p2) = self {
            format!("[{},{}]", p1.to_string(), p2.to_string())
        }
        else {
            format!("{}", self.unwrap())
        }
    }
                
    fn add(&self, new: &Self) -> Self {
        let mut new_pair = Self::new_pair(self, new);

        // println!("add: {}", new.to_string());
        // println!("to: {}", self.to_string());

        while let Some(reduced_pair) = new_pair.reduce() {
            new_pair = reduced_pair;
            // println!("reduced after add: {}", new_pair.to_string());
        }

        new_pair
    }

    fn reduce(&self) -> Option<Self> {
        unsafe {
            let mut reduced = self.clone();
            let mut did_reduction = false;

            let mut node = &mut reduced as *mut SnailfishNumber;
            let mut last_visited = None;
            let mut stack = Vec::<*mut SnailfishNumber>::new();

            // first, process explosions
            while stack.len() > 0 || (*node).is_pair() {
                if (*node).is_pair() { 
                    stack.push(node);
                    
                    if let Self::Pair(ref mut p1, _) = *node {
                        node = &mut **p1 as *mut SnailfishNumber;
                        continue;
                    }
                }

                let peek = stack[stack.len()-1];
                
                if let Self::Pair(_, ref mut p2) = *peek {
                    let p2_ptr = &mut **p2 as *mut SnailfishNumber;

                    if (*p2_ptr).is_pair() && last_visited != Some(p2_ptr) {
                        node = p2_ptr;
                        continue;
                    }
                }
                else { // we're only interested in visiting pairs
                    stack.pop();
                    continue;
                }

                if stack.len() < 5 {
                    last_visited = Some(stack.pop().unwrap());
                    continue;
                }
                    
                let exploder = stack.pop().unwrap();
                
                // println!("explode: {}", (*exploder).to_string());

                let parent = stack[stack.len()-1];
                let mut left_target = None;
                let mut right_target = None;
                let mut replacement = parent;

                if let Self::Pair(ref mut left, ref mut right) = *parent {
                    if **left == *exploder {
                        right_target = Some(&mut **right as *mut SnailfishNumber);
                        replacement = &mut **left as *mut SnailfishNumber;
                    }
                    else {
                        left_target = Some(&mut **left as *mut SnailfishNumber);
                        replacement = &mut **right as *mut SnailfishNumber;
                    }
                }

                let mut known_element = exploder;

                while let Some(search_pair) = stack.pop() {
                    if let Self::Pair(ref mut left, ref mut right) = *search_pair {
                        if left_target.is_none() && **left != *known_element {
                            left_target = Some(&mut **left as *mut SnailfishNumber);
                            break;
                        }
                        else if right_target.is_none() && **right != *known_element {
                            right_target = Some(&mut **right as *mut SnailfishNumber);
                            break;
                        }
                        else {
                            known_element = search_pair;
                        }
                    }
                }

                if let Self::Pair(ex_left,ex_right) = &*exploder {
                    if let Some(left_node) = left_target {
                        if (*left_node).is_pair() {
                            let rightmost = (*left_node).get_rightmost_node() as *const SnailfishNumber as *mut SnailfishNumber;
                            *rightmost = Self::new_number((*rightmost).unwrap()+ex_left.unwrap());
                        }
                        else {
                            *left_node = Self::new_number((*left_node).unwrap()+ex_left.unwrap());
                        }
                    }

                    if let Some(right_node) = right_target {
                        if (*right_node).is_pair() {
                            let leftmost = (*right_node).get_leftmost_node() as *const SnailfishNumber as *mut SnailfishNumber;
                            *leftmost = Self::new_number((*leftmost).unwrap()+ex_right.unwrap());
                        }
                        else {
                            *right_node = Self::new_number((*right_node).unwrap()+ex_right.unwrap());
                        }
                    }
                }

                *replacement = Self::new_number(0);
                did_reduction = true;
                
                break;
            }

            if did_reduction { return Some(reduced); }

            node = &mut reduced as *mut SnailfishNumber;

            // next, process splits
            loop {
                if (*node).is_pair() {
                    stack.push(node);

                    if let Self::Pair(ref mut p1,_) = *node {
                        node = &mut **p1 as *mut SnailfishNumber;
                    }

                    continue;
                }

                let prev_node = stack.pop();
                
                if (*node).is_number() && (*node).unwrap() >= 10 {
                    // println!("split: {}", (*node).to_string());
                        
                    let value = (*node).unwrap();
                    let left = value/2;
                    let right = left+value%2;

                    *node = Self::new_pair(&Self::new_number(left),&Self::new_number(right));
                    did_reduction = true;
                    break;
                }

                if let Some(popped_node) = prev_node {
                    if (*popped_node).is_pair() {
                        if let Self::Pair(_,ref mut p2) = *popped_node {
                            node = &mut **p2 as *mut SnailfishNumber;
                            continue;
                        }
                    }
                }

                break;
            }

            if did_reduction { Some(reduced) }
            else { None }
        }
    }

    fn magnitude(&self) -> usize {
        if let Self::Pair(p1,p2) = self {
            3*p1.magnitude() + 2*p2.magnitude()
        }
        else {
            self.unwrap()
        }
    }
}

fn read_numbers() -> Result<Vec<SnailfishNumber>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut numbers = Vec::<SnailfishNumber>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        numbers.push(SnailfishNumber::parse(buffer.trim()));
        buffer.clear();
    }

    if numbers.len() == 0 { Err(()) }
    else { Ok(numbers) }
}

fn part1() {
    if let Ok(mut numbers) = read_numbers() {
        numbers.reverse();
        
        let mut result = numbers.pop().unwrap();

        while let Some(reduced) = result.reduce() {
            result = reduced;
        }

        while let Some(next) = numbers.pop() {
            result = result.add(&next);
        }

        println!("{}", result.magnitude());
    }
    else { panic!("couldn't read numbers!"); }
}

fn part2() {
    if let Ok(numbers) = read_numbers() {
        let mut visited = HashSet::<(usize, usize)>::new();
        let mut max_magnitude = 0usize;
        
        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i == j { continue; }
                
                let node = (i,j);

                if visited.contains(&node) { continue; }
                visited.insert(node);

                let left = numbers[i].clone();
                let right = numbers[j].clone();

                let left_add = left.add(&right);
                let right_add = right.add(&left);

                let left_magnitude = left_add.magnitude();
                let right_magnitude = right_add.magnitude();

                if left_magnitude > max_magnitude {
                    max_magnitude = left_magnitude;
                }

                if right_magnitude > max_magnitude {
                    max_magnitude = right_magnitude;
                }
            }
        }

        println!("{}", max_magnitude);
    }
    else { panic!("couldn't read numbers!"); }
}

fn main() {
    // part1();
    part2();
}
