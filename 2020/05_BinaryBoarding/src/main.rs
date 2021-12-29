use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Seat(usize,usize);
impl Seat {
    fn from_str(s: &str) -> Self {
        let fixed = s.replace("F","0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1");

        let row = &fixed[..7];
        let column = &fixed[7..];

        Self(usize::from_str_radix(row, 2).unwrap(), usize::from_str_radix(column, 2).unwrap())
    }
    fn id(&self) -> usize {
        self.0 * 8 + self.1
    }
}

fn read_seats() -> Result<Vec<Seat>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut seats = Vec::<Seat>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        seats.push(Seat::from_str(buffer.trim()));
        buffer.clear();
    }

    if seats.len() == 0 { Err(()) }
    else { Ok(seats) }
}

fn part1() {
    if let Ok(seats) = read_seats() {
        println!("{}", seats.iter().map(|x| x.id()).reduce(|acc, x| if x > acc { x } else { acc }).unwrap());
    }
    else { panic!("couldn't read seats!"); }
}

fn part2() {
    if let Ok(mut seats) = read_seats() {
        let mut sorted_seats: Vec<usize> = seats.iter().map(|x| x.id()).collect();
        sorted_seats.sort();
        
        let neighbor = sorted_seats.iter()
            .enumerate()
            .map(|(i, x)| (
                *x,
                if i == 0 { None } else { Some(sorted_seats[i-1] as isize - sorted_seats[i] as isize) },
                if i == sorted_seats.len()-1 { None } else { Some(sorted_seats[i+1] as isize - sorted_seats[i] as isize) }
            ))
            .filter(|x| x.2 == Some(2))
            .next()
            .unwrap()
            .0;

        println!("{}", neighbor+1);
    }
    else { panic!("couldn't read seats!"); }
}

fn main() {
    // part1();
    part2();
}
