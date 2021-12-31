use std::io;
use std::collections::{HashMap, HashSet};

type BagId = usize;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bag {
    label: String,
    parents: Vec<BagId>,
    contents: Vec<(BagId, usize)>,
}
impl Bag {
    fn new(label: String) -> Self {
        Self { label: label.clone(), parents: Vec::<BagId>::new(), contents: Vec::<(BagId, usize)>::new() }
    }
    fn add_parent(&mut self, parent: BagId) {
        if !self.parents.contains(&parent) { self.parents.push(parent); }
    }
    fn add_content(&mut self, bag: BagId, count: usize) {
        if !self.contents.contains(&(bag, count)) { self.contents.push((bag, count)); }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bags {
    bags: Vec<Bag>,
}
impl Bags {
    fn new() -> Self {
        Self { bags: Vec::<Bag>::new() }
    }
    fn add_bag(&mut self, label: String) -> BagId {
        self.bags.push(Bag::new(label));

        self.bags.len()-1
    }
    fn get_bag(&self, id: BagId) -> &Bag {
        &self.bags[id]
    }
    fn get_bag_mut(&mut self, id: BagId) -> &mut Bag {
        &mut self.bags[id]
    }
    fn bag_map(&self) -> HashMap<String, BagId> {
        self.bags.iter().enumerate().map(|(i, x)| (x.label.clone(), i)).collect()
    }
    fn register_bag(&mut self, label: &String, contents: &Vec<(String, usize)>) {
        let parent_bag_id;

        if let Some(bag_id) = self.bag_map().get(label) { parent_bag_id = *bag_id; }
        else { parent_bag_id = self.add_bag(label.clone()); }

        for (content_label, amount) in contents {
            let content_id;

            if let Some(bag_id) = self.bag_map().get(content_label) { content_id = *bag_id; }
            else { content_id = self.add_bag(content_label.clone()); }
            
            let content_bag = self.get_bag_mut(content_id);
            content_bag.add_parent(parent_bag_id);
            
            let parent_bag = self.get_bag_mut(parent_bag_id);
            parent_bag.add_content(content_id, *amount);
        }
    }
    fn find_containers(&self, label: &String) -> HashSet<BagId> {
        let mut containers = HashSet::<BagId>::new();

        if let Some(bag_id) = self.bag_map().get(label) {
            let bag = self.get_bag(*bag_id);
            let mut stack = bag.parents.clone();

            while let Some(new_node) = stack.pop() {
                if containers.contains(&new_node) { continue; }
                containers.insert(new_node);

                let node = self.get_bag(new_node);
                stack.append(&mut node.parents.clone());
            }
        }

        containers
    }
    fn count_bags(&self, label: &String) -> usize {
        let bag_id;

        if let Some(found_bag_id) = self.bag_map().get(label) {
            bag_id = *found_bag_id;
        }
        else { return 0; }
        
        let mut stack: Vec<(BagId, usize)> = [(bag_id, 1)].iter().copied().collect();
        let mut sum = 0usize;

        while let Some((bag_id, bag_product)) = stack.pop() {
            let bag = self.get_bag(bag_id);
            let bags: usize = bag.contents.iter().map(|(_, count)| count).sum();

            sum += bags * bag_product;
            bag.contents.iter().for_each(|(id, count)| stack.push((*id, bag_product*count)));
        }

        sum
    }
}

fn read_bags() -> Result<Bags, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut bags = Bags::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        let chunks: Vec<&str> = buffer.trim().split(" bags contain ").collect();

        let label = chunks[0].to_string();
        
        if chunks[1] == "no other bags." {
            bags.register_bag(&label, &Vec::<(String, usize)>::new());
            buffer.clear();
            continue;
        }

        let contents: Vec<(String, usize)> = chunks[1].replace(" bags","")
            .replace(" bag","")
            .replace(".","")
            .split(", ")
            .map(|x| {
                if let Some((amount, bag)) = x.split_once(" ") {
                    (bag.to_string(), amount.parse::<usize>().unwrap())
                } else { panic!("bad chunk: {}", chunks[1]); }
            })
            .collect();
        
        bags.register_bag(&label, &contents);
        buffer.clear();
    }

    if bags.bags.len() == 0 { Err(()) }
    else { Ok(bags) }
}

fn part1() {
    if let Ok(bags) = read_bags() {
        println!("{}", bags.find_containers(&"shiny gold".to_string()).len());
    }
    else { panic!("couldn't read bags!"); }
}

fn part2() {
    if let Ok(bags) = read_bags() {
        println!("{}", bags.count_bags(&"shiny gold".to_string()));
    }
    else { panic!("couldn't read bags!"); }
}

fn main() {
    // part1();
    part2();
}
