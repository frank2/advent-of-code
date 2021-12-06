use std::io;

struct Command {
    command: String,
    distance: u32,
}
impl Command {   
    pub fn read() -> Result<Self, ()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let result = stdin.read_line(&mut buffer);

        if result.is_err() { return Err(()); }

        let split: Vec<&str> = buffer.split(" ").collect();

        if split.len() != 2 { return Err(()); }

        let (command, distance) = (split[0].to_string(), split[1].trim().parse::<u32>());

        if distance.is_err() { return Err(()); }

        Ok(Self {
            command: command,
            distance: distance.unwrap()
        })
    }
}

fn part1() {
    let mut position = 0u32;
    let mut depth = 0i32;
    
    while let Ok(command) = Command::read() {
        match command.command.as_str() {
            "forward" => position += command.distance,
            "down" => depth += command.distance as i32,
            "up" => depth -= command.distance as i32,
            _ => (),
        }
    }

    println!("{}", (position as i32) * depth);
}

fn part2() {
    let mut position = 0u32;
    let mut depth = 0i32;
    let mut aim = 0i32;
    
    while let Ok(command) = Command::read() {
        match command.command.as_str() {
            "down" => aim += command.distance as i32,
            "up" => aim -= command.distance as i32,
            "forward" => {
                position += command.distance;
                depth += aim * (command.distance as i32);
            },
            _ => (),
        }
    }

    println!("{}", (position as i32) * depth);
}

fn main() {
    // part1();
    part2();
}
