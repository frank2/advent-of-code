use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(usize, usize);
impl Coordinate {
    fn neighbors(&self, grid: Coordinate) -> Vec<Coordinate> {
        let (mx, my) = (grid.0 as isize, grid.1 as isize);
        let mut coords = Vec::<Coordinate>::new();

        for ny in -1..=1 {
            for nx in -1..=1 {
                if ny == 0 && nx == 0 { continue; }

                let dx = self.0 as isize + nx;
                let dy = self.1 as isize + ny;

                if dx < 0 || dx >= mx { continue; }
                if dy < 0 || dy >= my { continue; }

                coords.push(Coordinate(dx as usize,dy as usize));
            }
        }

        coords
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FlashState {
    Charging,
    Flashing,
    Flashed,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Octopus {
    energy: u32,
    state: FlashState,
}
impl Octopus {
    fn new(energy: u32) -> Self {
        Self { energy: energy, state: FlashState::Charging }
    }
    
    fn charge(&mut self) -> bool {
        if self.state != FlashState::Charging { return false; }
        
        self.energy += 1;

        if self.energy > 9 {
            self.state = FlashState::Flashing;
            true
        }
        else { false }
    }
    fn flash(&mut self) {
        self.state = FlashState::Flashed;
    }
    fn reset(&mut self) {
        self.energy = 0;
        self.state = FlashState::Charging;
    }
}

fn simulate(pod: &mut Vec<Vec<Octopus>>, steps: usize) -> Option<usize> {
    let mut flash_count = 0usize;
    
    for _ in 0..steps {
        let mut flashing = Vec::<Coordinate>::new();
        let mut flashed = Vec::<Coordinate>::new();
        
        for y in 0..pod.len() {
            for x in 0..pod[0].len() {
                if pod[y][x].charge() {
                    flashing.push(Coordinate(x,y));
                }
            }
        }

        while flashing.len() > 0 {
            let mut new_flashing = Vec::<Coordinate>::new();

            for coord in flashing {
                let (x,y) = (coord.0, coord.1);
                let octopus = &mut pod[y][x];
                octopus.flash();
                flashed.push(coord);

                for neighbor in coord.neighbors(Coordinate(pod[0].len(), pod.len())) {
                    let (nx, ny) = (neighbor.0, neighbor.1);

                    if pod[ny][nx].charge() { new_flashing.push(neighbor); }
                }
            }

            flashing = new_flashing;
        }

        if flashed.len() == pod[0].len() * pod.len() { return None; }

        flash_count += flashed.len();

        for coord in flashed {
            let (x,y) = (coord.0, coord.1);
            pod[y][x].reset();
        }
    }

    Some(flash_count)
}

fn read_pod() -> Result<Vec<Vec<Octopus>>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut octopuses = Vec::<Vec<Octopus>>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        octopuses.push(buffer.trim().as_bytes().iter().map(|&x| Octopus::new((x - '0' as u8) as u32)).collect());
        buffer.clear();
    }

    if octopuses.len() == 0 { Err(()) }
    else { Ok(octopuses) }
}

fn part1() {
    if let Ok(mut pod) = read_pod() {
        println!("{}", simulate(&mut pod, 100).unwrap());
    }
    else { panic!("couldn't read pod!"); }
}

fn part2() {
    if let Ok(mut pod) = read_pod() {
        let mut step = 0usize;
        
        loop {
            if let Some(_) = simulate(&mut pod, 1) {
                step += 1;
            }
            else {
                println!("{}", step+1);
                break;
            }
        }
    }
    else { panic!("couldn't read pod!"); }
}

fn main() {
    // part1(); 
    part2();
}
