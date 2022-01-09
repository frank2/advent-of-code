use std::io;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    coords: Vec<isize>
}
impl Coordinate {
    fn new(dimensions: usize) -> Self {
        Self { coords: vec![0isize; dimensions] }
    }
    fn from_vec(&mut self, v: &Vec<isize>) {
        if v.len() > self.coords.len() { self.coords = v.clone(); }
        else { (0..v.len()).for_each(|i| self.coords[i] = v[i]); }
    }
    fn dimensions(&self) -> usize {
        self.coords.len()
    }
    fn get_dimension(&self, d: usize) -> isize {
        self.coords[d]
    }
    fn set_dimension(&mut self, d: usize, v: isize) {
        self.coords[d] = v;
    }
    fn neighbors(&self) -> Vec<Coordinate> {
        let mut neighbors = Vec::<Coordinate>::new();
        let offsets = vec![RangeInclusive::<isize>::new(-1,1); self.dimensions()];
        let mut iterators = offsets.clone();

        loop {
            let coord_offset: Vec<isize> = iterators.iter().map(|x| x.start()).copied().collect();

            if coord_offset.iter().filter(|&x| *x==0).count() != self.dimensions() {
                let mut new_coord = self.clone();

                for dimension in 0..self.dimensions() {
                    let shifted = new_coord.coords[dimension] + coord_offset[dimension];
                    new_coord.coords[dimension] = shifted;
                }

                neighbors.push(new_coord);
            }

            let mut iterator = iterators.len();
            let mut next = None;

            while next.is_none() {
                if iterator == 0 { return neighbors; }
                iterator -= 1;

                next = iterators[iterator].next();
                
                if let Some(value) = next {
                    if value == *iterators[iterator].end() {
                        next = None;
                        continue;
                    }
                }

                iterator += 1;

                while iterator < iterators.len() {
                    iterators[iterator] = offsets[iterator].clone();
                    iterator += 1;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cube {
    Active,
    Inactive,
}
impl Cube {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Active,
            '.' => Self::Inactive,
            _ => panic!("bad cube: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Active => '#',
            Self::Inactive => '.',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PocketDimension {
    cubes: HashMap<Coordinate, Cube>,
    min: Coordinate,
    max: Coordinate,
}
impl PocketDimension {
    fn new(dimensions: usize) -> Self {
        Self { cubes: HashMap::<Coordinate, Cube>::new(), min: Coordinate::new(dimensions), max: Coordinate::new(dimensions) }
    }
    fn set_cube(&mut self, coord: &Coordinate, cube: Cube) {
        self.cubes.insert(coord.clone(), cube);

        for dimension in 0..coord.dimensions() {
            if coord.get_dimension(dimension) < self.min.get_dimension(dimension) {
                self.min.set_dimension(dimension, coord.get_dimension(dimension));
            }
            
            if coord.get_dimension(dimension)+1 > self.max.get_dimension(dimension) {
                self.max.set_dimension(dimension, coord.get_dimension(dimension)+1);
            }
        }
    }
    fn get_cube(&self, coord: &Coordinate) -> Cube {
        if let Some(cube) = self.cubes.get(coord) { *cube }
        else { Cube::Inactive }
    }
    fn step(&self) -> Self {
        let mut new_dimension = self.clone();
        let active_cubes: Vec<Coordinate> = self.cubes.iter()
            .filter(|(_, &c)| c == Cube::Active)
            .map(|(c, _)| c)
            .cloned()
            .collect();
        let mut affected_cubes = HashSet::<Coordinate>::new();

        for coord in &active_cubes {
            affected_cubes.insert(coord.clone());
            for neighbor in coord.neighbors() { affected_cubes.insert(neighbor); }
        }

        for coord in &affected_cubes {
            let cube = self.get_cube(coord);
            let active = coord.neighbors().iter()
                .map(|x| self.get_cube(x))
                .filter(|&x| x == Cube::Active)
                .count();

            match cube {
                Cube::Active => { if active != 2 && active != 3 { new_dimension.set_cube(coord, Cube::Inactive); } },
                Cube::Inactive => { if active == 3 { new_dimension.set_cube(coord, Cube::Active); } },
            }
        }

        new_dimension
    }
}

fn read_cubes(dimensions: usize) -> Result<PocketDimension, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut dimension = PocketDimension::new(dimensions);
    let mut y = 0isize;

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        for (i,x) in buffer.trim().chars().enumerate() {
            let mut new_coord = Coordinate::new(dimensions);
            new_coord.from_vec(&[i as isize, y].to_vec());

            dimension.set_cube(&new_coord, Cube::from_char(x));
        }

        y += 1;
        buffer.clear();
    }

    if dimension.cubes.len() == 0 { Err(()) }
    else { Ok(dimension) }
}

fn part1() {
    if let Ok(mut dimension) = read_cubes(3) {
        (0..6).for_each(|_| dimension = dimension.step());
        
        println!("{}", dimension.cubes.values().filter(|&x| *x == Cube::Active).count());
    }
    else { panic!("couldn't read cubes!"); }
}

fn part2() {
    if let Ok(mut dimension) = read_cubes(4) {
        (0..6).for_each(|_| dimension = dimension.step());
        
        println!("{}", dimension.cubes.values().filter(|&x| *x == Cube::Active).count());
    }
    else { panic!("couldn't read cubes!"); }
}

fn main() {
    // part1();
    part2();
}
