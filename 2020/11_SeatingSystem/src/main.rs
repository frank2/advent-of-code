use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(isize, isize);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Floor,
    Unoccupied,
    Occupied,
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Unoccupied,
            '#' => Self::Occupied,
            _ => panic!("bad tile character"),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Floor => '.',
            Self::Unoccupied => 'L',
            Self::Occupied => '#',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Layout {
    tiles: HashMap<Coordinate, Tile>,
    size: Coordinate,
    seats: HashSet<Coordinate>,
}
impl Layout {
    fn new() -> Self {
        Self { tiles: HashMap::<Coordinate, Tile>::new(), size: Coordinate(0,0), seats: HashSet::<Coordinate>::new() }
    }
    fn add_tile(&mut self, coord: Coordinate, tile: Tile) {
        self.tiles.insert(coord, tile);

        if coord.0+1 > self.size.0 { self.size.0 = coord.0+1; }
        if coord.1+1 > self.size.1 { self.size.1 = coord.1+1; }

        if tile != Tile::Floor {
            self.seats.insert(coord);
        }
    }
    
    fn neighbors_p1(&self, base_coord: Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::<Coordinate>::new();

        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 { continue; }

                let coord = Coordinate(base_coord.0+x, base_coord.1+y);

                if self.tiles.get(&coord).is_none() { continue; }

                neighbors.push(coord);
            }
        }

        neighbors
    }
    
    fn neighbors_p2(&self, base_coord: Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::<Coordinate>::new();

        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 { continue; }

                let mut coord = Coordinate(base_coord.0+x, base_coord.1+y);
                let mut multiplier = 1isize;

                while let Some(tile) = self.tiles.get(&coord) {
                    if *tile != Tile::Floor {
                        neighbors.push(coord);
                        break;
                    }

                    multiplier += 1;
                    coord = Coordinate(base_coord.0+(x*multiplier), base_coord.1+(y*multiplier));
                }
            }
        }

        neighbors
    }
        
    fn step_p1(&self) -> (usize, Self) {
        let mut new_layout = self.clone();
        let mut changes = 0usize;

        for coord in &self.seats {
            let occupied = self.neighbors_p1(*coord).iter()
                .map(|x| self.tiles.get(x).unwrap())
                .filter(|&x| *x == Tile::Occupied)
                .count();
            
            match self.tiles.get(coord).unwrap() {
                Tile::Floor => (),
                Tile::Unoccupied => {
                    if occupied == 0 {
                        new_layout.add_tile(*coord, Tile::Occupied);
                        changes += 1;
                    }
                },
                Tile::Occupied => {
                    if occupied >= 4 {
                        new_layout.add_tile(*coord, Tile::Unoccupied);
                        changes += 1;
                    }
                },
            }
        }

        (changes, new_layout)
    }
        
    fn step_p2(&self) -> (usize, Self) {
        let mut new_layout = self.clone();
        let mut changes = 0usize;

        for coord in &self.seats {
            let occupied = self.neighbors_p2(*coord).iter()
                .map(|x| self.tiles.get(x).unwrap())
                .filter(|&x| *x == Tile::Occupied)
                .count();
            
            match self.tiles.get(coord).unwrap() {
                Tile::Floor => (),
                Tile::Unoccupied => {
                    if occupied == 0 {
                        new_layout.add_tile(*coord, Tile::Occupied);
                        changes += 1;
                    }
                },
                Tile::Occupied => {
                    if occupied >= 5 {
                        new_layout.add_tile(*coord, Tile::Unoccupied);
                        changes += 1;
                    }
                },
            }
        }

        (changes, new_layout)
    }

    fn print(&self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let coord = Coordinate(x,y);
                let tile = self.tiles.get(&coord).unwrap();

                print!("{}", tile.to_char());
            }

            println!("");
        }
    }
}

fn read_layout() -> Result<Layout, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut layout = Layout::new();
    let mut y = 0isize;

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        buffer.trim()
            .chars()
            .enumerate()
            .for_each(|(i,x)| layout.add_tile(Coordinate(i as isize,y), Tile::from_char(x)));
        
        y += 1;
        buffer.clear();
    }

    if layout.tiles.len() == 0 { Err(()) }
    else { Ok(layout) }
}

fn count_seats(part: bool, layout: &Layout) -> usize {
    let mut layout_state = layout.clone();

    loop {
        let (changes, new_layout);

        if part { let state = layout_state.step_p2(); changes = state.0; new_layout = state.1; }
        else { let state = layout_state.step_p1(); changes = state.0; new_layout = state.1; }

        if changes == 0 { return layout_state.seats.iter()
                          .map(|x| new_layout.tiles.get(x).unwrap())
                          .filter(|&x| *x == Tile::Occupied)
                          .count(); }
        
        layout_state = new_layout;
    }
}
                    
fn part1() {
    if let Ok(layout) = read_layout() {
        println!("{}", count_seats(false, &layout));
    }
    else { panic!("couldn't read layout!"); }
}
            
fn part2() {
    if let Ok(layout) = read_layout() {
        println!("{}", count_seats(true, &layout));
    }
    else { panic!("couldn't read layout!"); }
}

fn main() {
    // part1();
    part2();
}
