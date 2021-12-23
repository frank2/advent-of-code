use std::io;
use std::collections::BTreeSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Off,
    On,
}
impl State {
    fn from_str(s: &str) -> Self {
        match s {
            "off" => Self::Off,
            "on" => Self::On,
            _ => panic!("invalid switch value"),
        }
    }
    fn flip(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Range(isize,isize);
impl Range {
    fn from_str(s: &str) -> Self {
        let chunks = s.split("..").collect::<Vec<&str>>();
        Self(chunks[0].parse::<isize>().unwrap(),chunks[1].parse::<isize>().unwrap())
    }
    fn contains(&self, r: Self) -> bool {
        r.0 >= self.0 && r.1 <= self.1
    }
    fn intersects(&self, r: Self) -> bool {
        r.0 <= self.1 && r.1 >= self.0
    }
    fn intersection(&self, r: Self) -> Option<Self> {
        if !self.intersects(r) { None }
        else { Some(Self(isize::max(self.0,r.0),isize::min(self.1,r.1))) }
    }
    fn len(&self) -> isize {
        let min = isize::min(self.0,self.1);
        let max = isize::max(self.0,self.1);

        (max-min)+1
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Cuboid(Range,Range,Range);
impl Cuboid {
    fn from_str(s: &str) -> Self {
        let chunks = s.split(",").collect::<Vec<&str>>();
        let mut ranges = Vec::<Range>::new();

        for chunk in chunks {
            let range_data = chunk.split("=").collect::<Vec<&str>>();
            ranges.push(Range::from_str(range_data[1]));
        }

        Self(ranges[0],ranges[1],ranges[2])
    }
    fn contains(&self, c: Self) -> bool {
        self.0.contains(c.0) && self.1.contains(c.1) && self.2.contains(c.2)
    }
    fn intersects(&self, c: Self) -> bool {
        self.0.intersects(c.0) && self.1.intersects(c.1) && self.2.intersects(c.2)
    }
    fn intersection(&self, c: Self) -> Option<Self> {
        if !self.intersects(c) { return None; }
        
        let x = self.0.intersection(c.0);
        let y = self.1.intersection(c.1);
        let z = self.2.intersection(c.2);
        
        Some(Self(x.unwrap(),y.unwrap(),z.unwrap()))
    }
    fn volume(&self) -> isize {
        self.0.len() * self.1.len() * self.2.len()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Reboot {
    state: State,
    cuboid: Cuboid,
}
impl Reboot {
    fn from_str(s: &str) -> Self {
        let chunks = s.split(" ").collect::<Vec<&str>>();

        Self { state: State::from_str(chunks[0]), cuboid: Cuboid::from_str(chunks[1]) }
    }
}

fn read_cuboids() -> Result<Vec<Reboot>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut result = Vec::<Reboot>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        result.push(Reboot::from_str(buffer.trim()));
        buffer.clear();
    }

    if result.len() == 0 { Err(()) }
    else { Ok(result) }
}

fn reboot(sequence: &Vec<Reboot>) -> isize {
    let mut on = 0isize;
    let mut result = Vec::<Reboot>::new();

    for i in 0..sequence.len() {
        let step = &sequence[i];

        for processed in &result.clone() {
            if !step.cuboid.intersects(processed.cuboid) {
                continue;
            }

            let intersection = step.cuboid.intersection(processed.cuboid).unwrap();
            let insertion = Reboot { state: processed.state.flip(), cuboid: intersection };
            
            result.push(insertion);
        }

        if step.state == State::On {
            result.push(*step);
        }
    }

    for sequence in result {
        match sequence.state {
            State::On => on += sequence.cuboid.volume(),
            State::Off => on -= sequence.cuboid.volume(),
        }
    }

    on
}

fn part1() {
    if let Ok(sequence) = read_cuboids() {
        let range = Cuboid(Range(-50,50),Range(-50,50),Range(-50,50));
        let filtered = sequence.iter().filter(|x| range.contains(x.cuboid)).copied().collect::<Vec<Reboot>>();
        println!("{}", reboot(&filtered));
    }
    else { panic!("couldn't read cuboids!"); }
}

fn part2() {
    if let Ok(sequence) = read_cuboids() {
        println!("{}", reboot(&sequence));
    }
    else { panic!("couldn't read cuboids!"); }
}

fn main() {
    // part1();
    part2();
}
