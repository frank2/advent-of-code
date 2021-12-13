use std::io;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}
impl Cave {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            _ => {
                if s.chars().filter(|x| x.is_uppercase()).count() > 0 {
                    Self::Big(s.to_string())
                }
                else { Self::Small(s.to_string()) }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Frame {
    node: Cave,
    path: Vec<Cave>,
    visits: HashMap<Cave, usize>,
    small_threshhold: usize,
}
impl Frame {
    fn new(node: &Cave, small_threshhold: usize, prev: Option<&Frame>) -> Self {
        if prev.is_none() {
            Self {
                node: node.clone(),
                path: vec![node.clone()],
                visits: HashMap::<Cave, usize>::new(),
                small_threshhold: small_threshhold,
            }
        }
        else {
            let frame = prev.unwrap();
            let mut path = frame.path.clone();
            path.push(node.clone());
            
            Self {
                node: node.clone(),
                path: path,
                visits: frame.visits.clone(),
                small_threshhold: frame.small_threshhold,
            }
        }
    }
    fn visit(&mut self, cave: &Cave) {
        let visit_count = self.visit_count(cave)+1;
        
        self.visits.insert(cave.clone(), visit_count);

        match cave {
            Cave::Small(_) => { if visit_count >= 2 { self.small_threshhold = 1; } },
            _ => (),
        }
    }
    fn visit_count(&self, cave: &Cave) -> usize {
        if let Some(&visits) = self.visits.get(cave) {
            visits
        }
        else {
            0
        }
    }
    fn visited(&self, cave: &Cave) -> bool {
        let visit_count = self.visit_count(cave);
        
        match cave {
            Cave::Start => visit_count >= 1,
            Cave::End => visit_count >= 1,
            Cave::Big(_) => false,
            Cave::Small(_) => visit_count >= self.small_threshhold,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct CaveSystem {
    nodes: HashMap<Cave, HashSet<Cave>>,
}
impl CaveSystem {
    fn new() -> Self { Self { nodes: HashMap::<Cave, HashSet<Cave>>::new() } }
    fn import_edge(&mut self, s: &str) {
        let caves: Vec<Cave> = s.split("-").map(|x| Cave::from_str(x)).collect();

        self.add_edge(&caves[0], &caves[1]);
        self.add_edge(&caves[1], &caves[0]);
    }
    fn add_edge(&mut self, from: &Cave, to: &Cave) {
        if let Some(edges) = self.nodes.get_mut(&from) {
            edges.insert(to.clone());
        }
        else {
            self.nodes.insert(from.clone(), [to.clone()].iter().cloned().collect());
        }
    }
    fn get_edges(&self, cave: &Cave) -> HashSet<Cave> {
        self.nodes.get(&cave).unwrap().clone()
    }
    
    fn traverse(&self, small_threshhold: usize) -> usize {
        let mut frames: Vec<Frame> = vec![Frame::new(&Cave::Start, small_threshhold, None)];
        let mut paths = 0usize;

        while let Some(mut frame) = frames.pop() {
            let node = frame.node.clone();
            
            match node {
                Cave::End => {
                    // println!("found path: {:?}", frame.path);
                    paths += 1;
                    continue;
                },
                Cave::Small(_) => {
                    if frame.visited(&node) { continue; }
                },
                _ => (),
            }

            frame.visit(&node);
                        
            self.get_edges(&node).iter()
                .filter(|&x| !frame.visited(&x))
                .for_each(|x| frames.push(Frame::new(x, small_threshhold, Some(&frame))));
        }

        paths
    }
}

fn read_system() -> Result<CaveSystem, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut system = CaveSystem::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        system.import_edge(&buffer.trim());
        buffer.clear();
    }

    if system.nodes.len() == 0 { Err(()) }
    else { Ok(system) }
}

fn part1() {
    if let Ok(system) = read_system() {
        println!("{}", system.traverse(1));
    }
    else { panic!("couldn't read system!"); }
}

fn part2() {
    if let Ok(system) = read_system() {
        println!("{}", system.traverse(2));
    }
    else { panic!("couldn't read system!"); }
}

fn main() {
    // part1();
    part2();
}
