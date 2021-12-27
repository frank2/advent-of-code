use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(usize,usize);
impl Coordinate {
    fn eastbound(&self, grid: Coordinate) -> Self {
        Self((self.0+1) % grid.0, self.1)
    }
    fn southbound(&self, grid: Coordinate) -> Self {
        Self(self.0, (self.1+1) % grid.1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum TilePiece {
    Empty,
    East,
    South,
}
impl TilePiece {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '>' => Self::East,
            'v' => Self::South,
            _ => panic!("bad tile piece"),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::East => '>',
            Self::South => 'v',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct SeaFloor {
    map: HashMap<Coordinate, TilePiece>,
    tiles: HashMap<TilePiece, HashSet<Coordinate>>,
    grid: Coordinate,
}
impl SeaFloor {
    fn new() -> Self {
        Self { map: HashMap::<Coordinate, TilePiece>::new(), tiles: HashMap::<TilePiece, HashSet<Coordinate>>::new(), grid: Coordinate(0,0) }
    }
    fn get(&self, coord: Coordinate) -> TilePiece {
        *self.map.get(&coord).unwrap()
    }
    fn add(&mut self, coord: Coordinate, tile: TilePiece) {
        self.map.insert(coord, tile);

        if let Some(tiles) = self.tiles.get_mut(&tile) {
            tiles.insert(coord);
        }
        else {
            self.tiles.insert(tile, [coord].iter().copied().collect());
        }

        if coord.0+1 > self.grid.0 { self.grid.0 = coord.0+1; }
        if coord.1+1 > self.grid.1 { self.grid.1 = coord.1+1; }
    }
    fn remove(&mut self, coord: Coordinate) {
        let removed = self.map.remove(&coord);

        if removed.is_none() { return; }

        if let Some(tiles) = self.tiles.get_mut(&removed.unwrap()) {
            tiles.remove(&coord);
        }
    }
    fn swap(&mut self, c1: Coordinate, c2: Coordinate) {
        let c1_tile = self.get(c1);
        let c2_tile = self.get(c2);
        
        self.remove(c1);
        self.remove(c2);

        self.add(c2, c1_tile);
        self.add(c1, c2_tile);
    }
    fn migrate(&mut self) -> usize {
        let mut migrations = Vec::<(Coordinate, Coordinate)>::new();
        let mut total_migrations = 0usize;

        if let Some(eastbound_pieces) = self.tiles.get(&TilePiece::East) {
            for coord in eastbound_pieces {
                let eastbound = coord.eastbound(self.grid);
                if self.get(eastbound) == TilePiece::Empty { migrations.push((*coord, eastbound)); }
            }
        }

        total_migrations += migrations.len();

        while let Some((coord,eastbound)) = migrations.pop() {
            self.swap(coord,eastbound);
        }
            
        if let Some(southbound_pieces) = self.tiles.get(&TilePiece::South) {
            for coord in southbound_pieces {
                let southbound = coord.southbound(self.grid);
                if self.get(southbound) == TilePiece::Empty { migrations.push((*coord, southbound)); }
            }
        }

        total_migrations += migrations.len();

        while let Some((coord,southbound)) = migrations.pop() {
            self.swap(coord,southbound);
        }

        total_migrations
    }
    fn print(&self) {
        for y in 0..self.grid.1 {
            for x in 0..self.grid.0 {
                let coord = Coordinate(x,y);

                print!("{}", self.get(coord).to_char());
            }

            println!("");
        }

        println!("");
    }
}

fn migrate_seafloor(seafloor: &mut SeaFloor) -> usize {
    let mut step = 0usize;

    loop {
        step += 1;
        let migration_check = seafloor.migrate();

        // println!("step {}:", step);
        // seafloor.print();

        // if step == 30 { panic!("break"); }

        if migration_check == 0 { return step; }
    }
}

fn read_seafloor() -> Result<SeaFloor, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut seafloor = SeaFloor::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
        
        let y = seafloor.grid.1;
        let mut x = 0usize;
        
        for c in buffer.trim().chars() {
            seafloor.add(Coordinate(x,y), TilePiece::from_char(c));
            x += 1;
        }

        buffer.clear();
    }

    if seafloor.map.len() == 0 { Err(()) }
    else { Ok(seafloor) }
}
            
fn main() {
    if let Ok(mut seafloor) = read_seafloor() {
        seafloor.print();
        println!("{}", migrate_seafloor(&mut seafloor));
    }
    else { panic!("couldn't read seafloor!"); }
}
