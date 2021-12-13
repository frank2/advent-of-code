use std::io;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(usize, usize);
impl Coordinate {
    fn fold_x(&self, bifurcation: usize) -> Self {
        Self(self.0-((self.0 - bifurcation) * 2),self.1)
    }
    fn fold_y(&self, bifurcation: usize) -> Self {
        Self(self.0,self.1-((self.1 - bifurcation) * 2))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Axis {
    X,
    Y,
}
impl Axis {
    fn from_string(s: &String) -> Self {
        match s.as_str() {
            "x" => Self::X,
            "y" => Self::Y,
            _ => panic!("bad axis!"),
        }
    }
}

fn read_origami() -> Result<(HashSet<Coordinate>, Vec<(Axis,usize)>), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut coords = HashSet::<Coordinate>::new();
    let mut instructions = Vec::<(Axis,usize)>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { break; }

        let coord: Vec<usize> = buffer.trim().split(",").map(|x| x.parse().unwrap()).collect();
        coords.insert(Coordinate(coord[0], coord[1]));
            
        buffer.clear();
    }

    if coords.len() == 0 { return Err(()); }

    buffer.clear();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        let instruction: Vec<String> = buffer.trim().replace("fold along ","").split("=").map(|x| x.to_string()).collect();
        instructions.push((Axis::from_string(&instruction[0]),instruction[1].parse().unwrap()));
        
        buffer.clear();
    }

    if instructions.len() == 0 { Err(()) }
    else { Ok((coords, instructions)) }
}

fn fold(coords: &HashSet<Coordinate>, axis: Axis, bifurcation: usize) -> HashSet<Coordinate> {
    let affected_coords: HashSet<Coordinate> = match axis {
        Axis::X => {
            coords.iter().filter(|x| x.0 > bifurcation).cloned().collect()
        },
        Axis::Y => {
            coords.iter().filter(|x| x.1 > bifurcation).cloned().collect()
        },
    };

    let unaffected_coords: HashSet<Coordinate> = coords.difference(&affected_coords).cloned().collect();

    let folded_coords: HashSet<Coordinate> = match axis {
        Axis::X => {
            affected_coords.iter().map(|x| x.fold_x(bifurcation)).collect()
        },
        Axis::Y => {
            affected_coords.iter().map(|x| x.fold_y(bifurcation)).collect()
        }
    };

    folded_coords.union(&unaffected_coords).cloned().collect()
}

fn print_dots(coords: &HashSet<Coordinate>) {
    let (mut mx, mut my) = (0usize, 0usize);

    for coord in coords {
        let (nx,ny) = (coord.0,coord.1);

        if nx+1>mx { mx=nx+1; }
        if ny+1>my { my=ny+1; }
    }

    let mut buffer = vec![vec![' '; mx]; my];

    for coord in coords {
        let (x,y) = (coord.0,coord.1);

        buffer[y][x] = '#';
    }

    let lines: Vec<String> = buffer.iter().map(|x| x.into_iter().collect()).collect();

    for line in lines { println!("{}", line); }
}

fn part1() {
    if let Ok((coords, instructions)) = read_origami() {
        let (axis, bifurcation) = instructions[0];
                
        println!("{}", fold(&coords, axis, bifurcation).len());
    }
    else { panic!("couldn't read origami!"); }
}

fn part2() {
    if let Ok((mut coords, instructions)) = read_origami() {
        let (axis, bifurcation) = instructions[0];

        for (axis, bifurcation) in instructions {
            coords = fold(&coords, axis, bifurcation);
        }

        print_dots(&coords);
    }
    else { panic!("couldn't read origami!"); }
}

fn main() {
    // part1();
    part2();
}
