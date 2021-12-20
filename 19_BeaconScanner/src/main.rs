use std::io;
use std::collections::{HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(isize,isize,isize);
impl Coordinate {
    fn delta(&self, coord: Coordinate) -> Coordinate {
        Self(self.0-coord.0,self.1-coord.1,self.2-coord.2)
    }
    fn invert(&self) -> Coordinate {
        Self(-self.0,-self.1,-self.2)
    }
    fn distance(&self, coord: Coordinate) -> isize {
        isize::abs(self.0-coord.0)+isize::abs(self.1-coord.1)+isize::abs(self.2-coord.2)
    }
    fn rotations(&self) -> Vec<Coordinate> {
        let x = self.0;
        let y = self.1;
        let z = self.2;

        [
            Coordinate(x,y,z),
            Coordinate(x,-y,-z),
            Coordinate(x,z,-y),
            Coordinate(x,-z,y),
         
            Coordinate(y,x,-z),
            Coordinate(y,-x,z),
            Coordinate(y,z,x),
            Coordinate(y,-z,-x),

            Coordinate(z,x,y),
            Coordinate(z,-x,-y),
            Coordinate(z,y,-x),
            Coordinate(z,-y,x),

            Coordinate(-x,y,-z),
            Coordinate(-x,-y,z),
            Coordinate(-x,z,y),
            Coordinate(-x,-z,-y),

            Coordinate(-y,x,z),
            Coordinate(-y,-x,-z),
            Coordinate(-y,z,-x),
            Coordinate(-y,-z,x),

            Coordinate(-z,x,-y),
            Coordinate(-z,-x,y),
            Coordinate(-z,y,x),
            Coordinate(-z,-y,-x)
         ].iter().copied().collect()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Scanner {
    beacons: Vec<Coordinate>,
    location: Option<Coordinate>
}
impl Scanner {
    fn new() -> Self {
        Self { beacons: Vec::<Coordinate>::new(), location: None }
    }
    fn push(&mut self, coord: Coordinate) {
        self.beacons.push(coord);
    }
    fn overlap(&mut self, known: &HashSet<Coordinate>) -> Option<HashSet<Coordinate>> {
        let mut rotations = Vec::<Vec<Coordinate>>::new();

        for coord in &self.beacons {
            rotations.push(coord.rotations());
        }

        for rotation_index in 0..rotations[0].len() {
            for beacon_index in 0..rotations.len() {
                let target_beacon = rotations[beacon_index][rotation_index];

                for known_beacon in known {
                    let known_delta = target_beacon.delta(*known_beacon);
                    let mut beacon_deltas = HashSet::<Coordinate>::new();

                    for delta_beacon_index in 0..rotations.len() {
                        let delta_beacon = rotations[delta_beacon_index][rotation_index].delta(known_delta);
                        beacon_deltas.insert(delta_beacon);
                    }

                    let overlaps: HashSet<Coordinate> = beacon_deltas.intersection(&known).copied().collect();

                    if overlaps.len() >= 12 {
                        println!("overlaps: {:?}", overlaps);
                        self.location = Some(known_delta.invert());
                        println!("location: {:?}", self.location);
                        return Some(beacon_deltas.difference(&known).copied().collect());
                    }
                }
            }
        }

        None
    }
}

fn process_beacons(scanners: &mut Vec<Scanner>) -> HashSet<Coordinate> {
    scanners[0].location = Some(Coordinate(0,0,0));
    let mut known_beacons: HashSet<Coordinate> = scanners[0].beacons.iter().copied().collect();
    let mut located: HashSet<usize> = [0].iter().copied().collect();

    while located.len() < scanners.len() {
        let mut new_beacons = HashSet::<Coordinate>::new();
        
        for i in 0..scanners.len() {
            if located.contains(&i) { continue; }

            let result = scanners[i].overlap(&known_beacons);

            if result.is_none() { continue; }

            new_beacons = new_beacons.union(&result.unwrap()).copied().collect();
            located.insert(i);
        }

        println!("found {} new beacons", new_beacons.len());
        println!("located: {} / {:?}", located.len(), located);
        known_beacons = known_beacons.union(&new_beacons).copied().collect();
    }

    known_beacons
}

fn read_beacons() -> Result<Vec<Scanner>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut scanners = Vec::<Scanner>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        if buffer.contains("scanner") {
            scanners.push(Scanner::new());
            buffer.clear();
            continue;
        }

        let chunks: Vec<isize> = buffer.trim().split(",").map(|x| x.parse::<isize>().unwrap()).collect();
        
        let index = scanners.len()-1;
        scanners[index].push(Coordinate(chunks[0],chunks[1],chunks[2]));
        buffer.clear();
    }

    if scanners.len() == 0 { Err(()) }
    else { Ok(scanners) }
}

fn part1() {
    if let Ok(mut beacons) = read_beacons() {
        println!("{}", process_beacons(&mut beacons).len());
    }
    else { panic!("couldn't read beacons!"); }
}

fn part2() {
    if let Ok(mut beacons) = read_beacons() {
        process_beacons(&mut beacons);

        let mut max_distance = 0isize;
        let mut visited = HashSet::<(usize, usize)>::new();

        for i in 0..beacons.len() {
            for j in 0..beacons.len() {
                if i == j { continue; }

                let index;

                if i < j { index = (i,j); }
                else { index = (j,i); }

                if visited.contains(&index) { continue; }
                visited.insert(index);

                let l1 = beacons[i].location.unwrap();
                let l2 = beacons[j].location.unwrap();
                let distance = l1.distance(l2);

                if distance > max_distance { max_distance = distance; }
            }
        }

        println!("{}", max_distance);
    }
}

fn main() {
    // part1();
    part2();
}
