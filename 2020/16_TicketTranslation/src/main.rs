use std::io;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::ops::RangeInclusive as Range;

#[derive(Clone, Eq, PartialEq, Debug)]
struct TicketRule {
    label: String,
    ranges: [Range<usize>; 2]
}
impl TicketRule {
    fn from_str(s: &str) -> Self {
        let chunks: Vec<&str> = s.split(": ").collect();
        let label = chunks[0].to_string();
        let range_strings: Vec<&str> = chunks[1].split(" or ").collect();
        let mut ranges = Vec::<Range<usize>>::new();

        for string in range_strings {
            let values: Vec<usize> = string.split("-").map(|x| x.parse::<usize>().unwrap()).collect();
            ranges.push(Range::<usize>::new(values[0], values[1]));
        }

        Self { label: label, ranges: ranges.try_into().unwrap() }
    }
    fn contains(&self, value: usize) -> bool {
        self.ranges[0].contains(&value) || self.ranges[1].contains(&value)
    }
}

type Ticket = Vec<usize>;

fn validate_ticket(rules: &Vec<TicketRule>, ticket: &Ticket) -> Option<usize> {
    if ticket.len() != rules.len() { panic!("bad ticket: {:?}", ticket); }

    for value in ticket {
        let valid = rules.iter()
            .map(|x| x.contains(*value))
            .filter(|&x| x == true)
            .count() > 0;
        
        if !valid { return Some(*value); }
    }

    None
}

fn identify_fields(rules: &Vec<TicketRule>, my_ticket: &Ticket, nearby_tickets: &Vec<Ticket>) -> HashMap<usize, TicketRule> {
    let mut result = HashMap::<usize, TicketRule>::new();
    let mut found_rules = HashSet::<usize>::new();

    loop {
        for index in 0..rules.len() {
            if result.contains_key(&index) { continue; }

            let mut reduction: HashSet<usize> = rules.iter()
                .enumerate()
                .filter(|(i, _)| !found_rules.contains(&i))
                .filter(|(_, x)| x.contains(my_ticket[index]))
                .map(|(i, _)| i)
                .collect();

            for ticket in nearby_tickets {
                reduction = reduction.intersection(&rules.iter()
                                                   .enumerate()
                                                   .filter(|(i, _)| !found_rules.contains(&i))
                                                   .filter(|(_, x)| x.contains(ticket[index]))
                                                   .map(|(i, _)| i)
                                                   .collect())
                    .copied()
                    .collect();
            }

            if reduction.len() == 1 {
                let found_index = *reduction.iter().next().unwrap();
                
                result.insert(index, rules[found_index].clone());
                found_rules.insert(found_index);
            }
        }

        if result.len() == rules.len() { return result; }
    }
}

fn read_ticket_info() -> Result<(Vec<TicketRule>, Ticket, Vec<Ticket>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut rules = Vec::<TicketRule>::new();
    let mut my_ticket = Ticket::new();
    let mut nearby_tickets = Vec::<Ticket>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { break; }

        rules.push(TicketRule::from_str(buffer.trim()));
        buffer.clear();
    }

    if rules.len() == 0 { return Err(()); }

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { break; }
        if buffer.trim() != "your ticket:" {
            my_ticket = buffer.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        }
        buffer.clear();
    }

    if my_ticket.len() == 0 { return Err(()); }

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { break; }
        if buffer.trim() == "nearby tickets:" { buffer.clear(); continue; }
        
        nearby_tickets.push(buffer.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect());
        buffer.clear();
    }

    if nearby_tickets.len() == 0 { Err(()) }
    else { Ok((rules, my_ticket, nearby_tickets)) }
}
    
fn part1() {
    if let Ok((rules, _, nearby_tickets)) = read_ticket_info() {
        let mut sum = 0usize;

        for ticket in &nearby_tickets {
            if let Some(invalid_entry) = validate_ticket(&rules, ticket) {
                println!("{:?}: {}", ticket, invalid_entry);
                sum += invalid_entry;
            }
        }

        println!("{}", sum);
    }
    else { panic!("couldn't read ticket info!"); }
}
    
fn part2() {
    if let Ok((rules, my_ticket, nearby_tickets)) = read_ticket_info() {
        let valid_tickets: Vec<Ticket> = nearby_tickets.iter()
            .filter(|x| validate_ticket(&rules, x).is_none())
            .cloned()
            .collect();
        let fields = identify_fields(&rules, &my_ticket, &valid_tickets);

        println!("{}", fields.iter()
                 .filter(|(i,x)| x.label.contains("departure"))
                 .map(|(i,_)| my_ticket[*i])
                 .product::<usize>());
    }
    else { panic!("couldn't read ticket info!"); }
}

fn main() {
    // part1();
    part2();
}
