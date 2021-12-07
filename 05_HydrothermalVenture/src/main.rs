use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(u32, u32);
impl Coordinate {
    fn from_str(s: &str) -> Self {
        let coords: Vec<u32> = s.split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        Self(coords[0], coords[1])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Line(Coordinate, Coordinate);
impl Line {
    pub fn slope(&self) -> Option<f32> {
        let mx = (self.1.0 as f32) - (self.0.0 as f32);

        if mx == 0.0 { return None; }

        let my = (self.1.1 as f32) - (self.0.1 as f32);

        Some(my/mx)
    }
    pub fn traverse(&self) -> Vec<Coordinate> {
        let (mut x1, x2) = (self.0.0 as i32, self.1.0 as i32);
        let (mut y1, y2) = (self.0.1 as i32, self.1.1 as i32);

        let dx = i32::abs(x2-x1);
        let sx = if x1 < x2 { 1i32 } else { -1i32 };

        let dy = -i32::abs(y2-y1);
        let sy = if y1 < y2 { 1i32 } else { -1i32 };

        let mut err = dx+dy;
        let mut coords = Vec::<Coordinate>::new();

        loop {
            coords.push(Coordinate(x1 as u32, y1 as u32));

            if x1 == x2 && y1 == y2 { break; }

            let e2 = 2*err;

            if e2 >= dy { err += dy; x1 += sx; }
            if e2 <= dx { err += dx; y1 += sy; }
        }

        coords
    }
}

fn read_lines() -> Result<Vec<Line>, ()> {
    let mut buffer = String::new();
    let mut lines = Vec::<Line>::new();
    let stdin = io::stdin();
    
    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; } // newline
        
        let coords: Vec<Coordinate> = buffer
            .trim()
            .split(" -> ")
            .map(|x| Coordinate::from_str(x))
            .collect();

        lines.push(Line(coords[0], coords[1]));

        buffer.clear();
    }

    if lines.len() == 0 { return Err(()); }

    Ok(lines)
}

fn traverse_lines(lines: &Vec<Line>) -> u32 {
    let mut visited = HashMap::<Coordinate, u8>::new();
    let mut overlaps = 0u32;

    for line in lines {
        for coordinate in line.traverse() {
            if let Some(&visit) = visited.get(&coordinate) {
                if visit == 2 { continue; }
                
                visited.insert(coordinate, 2);
                overlaps += 1;
            }
            else { visited.insert(coordinate, 1); }
        }
    }

    overlaps
}

fn part1() {
    if let Ok(mut lines) = read_lines() {
        lines = lines.iter()
            .filter(|x| x.slope() == None || x.slope() == Some(0.0))
            .cloned()
            .collect();

        println!("{:?}", traverse_lines(&lines));
    }
    else { panic!("couldn't read line data"); }
}

fn part2() {
    if let Ok(mut lines) = read_lines() {
        println!("{:?}", traverse_lines(&lines));
    }
    else { panic!("couldn't read line data"); }
}

fn main() {
    // part1();
    part2();
}
