use std::io;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coordinate(isize, isize);
impl Coordinate {
    fn rotate_right(&self, degrees: usize) -> Coordinate {
        let polarity = [Coordinate(1,1),Coordinate(1,-1),Coordinate(-1,-1),Coordinate(-1,1)];
        let rotations = [self.clone(),Coordinate(self.1,self.0)];
        let index = (degrees/90) % 4;

        rotations[index % 2] * polarity[index]
    }
    fn rotate_left(&self, degrees: usize) -> Coordinate {
        let polarity = [Coordinate(1,1),Coordinate(-1,1),Coordinate(-1,-1),Coordinate(1,-1)];
        let rotations = [self.clone(),Coordinate(self.1,self.0)];
        let index = (degrees/90) % 4;

        rotations[index % 2] * polarity[index]
    }
}
impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0+rhs.0,self.1+rhs.1)
    }
}
impl Mul for Coordinate {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0*rhs.0,self.1*rhs.1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}
impl Heading {
    fn from_int(u: usize) -> Self {
        match u {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("bad int: {}", u),
        }
    }
    fn to_int(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }
    fn rotate_right(&self, degrees: usize) -> Self {
        let rotations = degrees/90;

        Self::from_int((self.to_int() + rotations) % 4)
    }
    fn rotate_left(&self, degrees: usize) -> Self {
        let rotations = (degrees/90) as isize;
        let mut heading = self.to_int() as isize - rotations;

        while heading < 0 { heading += 4; }

        Self::from_int(heading as usize)
    }
    fn polarity(&self) -> Coordinate {
        match self {
            Self::North => Coordinate(0,1),
            Self::East => Coordinate(1,0),
            Self::South => Coordinate(0,-1),
            Self::West => Coordinate(-1,0),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Action {
    Heading(Heading, isize),
    RotateLeft(usize),
    RotateRight(usize),
    Forward(isize),
}
impl Action {
    fn from_str(s: &str) -> Self {
        let mut chars: Vec<char> = s.chars().collect();
        chars.reverse();

        let operand = chars.pop().unwrap();
        let mut value = 0usize;

        while let Some(c) = chars.pop() {
            value *= 10;
            value += c as usize - '0' as usize;
        }

        match operand {
            'N' => Self::Heading(Heading::North, value as isize),
            'S' => Self::Heading(Heading::South, value as isize),
            'E' => Self::Heading(Heading::East, value as isize),
            'W' => Self::Heading(Heading::West, value as isize),
            'L' => Self::RotateLeft(value),
            'R' => Self::RotateRight(value),
            'F' => Self::Forward(value as isize),
            _ => panic!("bad operand: {}", operand)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Ship {
    heading: Heading,
    coord: Coordinate,
    waypoint: Coordinate,
}
impl Ship {
    fn new() -> Self {
        Self { heading: Heading::East, coord: Coordinate(0,0), waypoint: Coordinate(10,1) }
    }
    fn act_p1(&mut self, action: Action) {
        match action {
            Action::Heading(heading, count) => {
                let coord = Coordinate(count, count) * heading.polarity();
                self.coord = self.coord+coord;
            },
            Action::RotateLeft(degrees) => {
                self.heading = self.heading.rotate_left(degrees);
            },
            Action::RotateRight(degrees) => {
                self.heading = self.heading.rotate_right(degrees);
            },
            Action::Forward(amount) => {
                let coord = Coordinate(amount, amount) * self.heading.polarity();
                self.coord = self.coord+coord;
            },
        }
    }
    fn act_p2(&mut self, action: Action) {
        match action {
            Action::Heading(heading, count) => {
                let coord = Coordinate(count, count) * heading.polarity();
                self.waypoint = self.waypoint+coord;
            },
            Action::RotateLeft(degrees) => {
                self.waypoint = self.waypoint.rotate_left(degrees);
            },
            Action::RotateRight(degrees) => {
                self.waypoint = self.waypoint.rotate_right(degrees);
            },
            Action::Forward(amount) => {
                let coord = Coordinate(amount, amount) * self.waypoint;
                self.coord = self.coord+coord;
            },
        }
    }
    fn perform_p1(&mut self, actions: &Vec<Action>) {
        actions.iter().for_each(|x| self.act_p1(*x));
    }
    fn perform_p2(&mut self, actions: &Vec<Action>) {
        actions.iter().for_each(|x| self.act_p2(*x));
    }
    fn manhattan(&self) -> isize {
        isize::abs(self.coord.0) + isize::abs(self.coord.1)
    }
}

fn read_actions() -> Result<Vec<Action>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut actions = Vec::<Action>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        actions.push(Action::from_str(buffer.trim()));
        buffer.clear();
    }

    if actions.len() == 0 { Err(()) }
    else { Ok(actions) }
}

fn part1() {
    if let Ok(actions) = read_actions() {
        let mut ship = Ship::new();
        ship.perform_p1(&actions);

        println!("{}", ship.manhattan());
    }
    else { panic!("couldn't read actions!"); }
}

fn part2() {
    if let Ok(actions) = read_actions() {
        let mut ship = Ship::new();
        ship.perform_p2(&actions);

        println!("{}", ship.manhattan());
    }
    else { panic!("couldn't read actions!"); }
}

fn main() {
    // part1();
    part2();
}
