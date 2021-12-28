use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Terrain {
    Snow,
    Tree,
}
impl Terrain {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            '.' => Self::Snow,
            _ => panic!("bad terrain character: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Tree => '#',
            Self::Snow => '.',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Slope {
    terrain: Vec<Vec<Terrain>>,
}
impl Slope {
    fn from_str(s: &str) -> Self {
        let mut terrain = Vec::<Vec<Terrain>>::new();

        for line in s.split("\n") {
            terrain.push(line.chars().map(|x| Terrain::from_char(x)).collect());
        }

        Self { terrain }
    }
    fn slope(&self, right: usize, down: usize) -> usize {
        let height = self.terrain.len();
        let mut x = 0usize;
        let mut y = 0usize;
        let mut trees = 0usize;

        loop {
            x += right;
            x %= self.terrain[0].len();
            y += down;

            if y >= height { return trees; }

            if self.terrain[y][x] == Terrain::Tree { trees += 1; }
        }
    }
}

fn read_slope() -> Result<Slope, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }
    }

    if buffer.len() == 0 { Err(()) }
    else { Ok(Slope::from_str(buffer.trim())) }
}

fn part1() {
    if let Ok(slope) = read_slope() {
        println!("{}", slope.slope(3, 1));
    }
    else { panic!("couldn't read slope!"); }
}

fn part2() {
    if let Ok(slope) = read_slope() {
        println!("{}", [
            slope.slope(1,1),
            slope.slope(3,1),
            slope.slope(5,1),
            slope.slope(7,1),
            slope.slope(1,2),
        ].iter().product::<usize>());
    }
    else { panic!("couldn't read slope!"); }
}

fn main() {
    // part1();
    part2();
}
