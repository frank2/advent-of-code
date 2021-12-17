use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Velocity(isize, isize);
impl Velocity {
    fn step(&self, step: isize) -> Velocity {
        let dy = self.1 - step;
        let mut dx = isize::abs(self.0) - step;

        if dx < 0 { dx = 0; }
        if self.0 < 0 { dx = -dx; }

        Velocity(dx, dy)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coordinate(isize, isize);
impl Coordinate {
    fn add_velocity(&self, v: Velocity) -> Coordinate {
        Coordinate(self.0+v.0, self.1+v.1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Rectangle(Coordinate, Coordinate);
impl Rectangle {
    fn contains_x(&self, c: Coordinate) -> bool {
        let x1 = if self.0.0 < self.1.0 { self.0.0 } else { self.1.0 };
        let x2 = if self.0.0 >= self.1.0 { self.0.0 } else { self.1.0 };
        
        c.0 >= x1 && c.0 <= x2
    }
    fn contains_y(&self, c: Coordinate) -> bool {
        let y1 = if self.0.1 < self.1.1 { self.0.1 } else { self.1.1 };
        let y2 = if self.0.1 >= self.1.1 { self.0.1 } else { self.1.1 };
        
        c.1 >= y1 && c.1 <= y2
    }
    fn contains(&self, c: Coordinate) -> bool {
        self.contains_x(c) && self.contains_y(c)
    }
    fn missed(&self, c: Coordinate) -> bool {
        (c.1 < self.0.1 && c.1 < self.1.1) || (c.0 > self.0.0 && c.0 > self.1.0)
    }
    fn lowest_point(&self) -> isize {
        if self.0.1 < self.1.1 { self.0.1 } else { self.1.1 }
    }
    fn furthest_point(&self) -> isize {
        if self.0.0 > self.1.0 { self.0.0 } else { self.1.0 }
    }
}

fn read_area() -> Result<Rectangle, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { return Err(()); }
        
        buffer = buffer.trim().replace("target area: ", "");
        let chunks: Vec<&str> = buffer.split(", ").collect();

        let x_coords: Vec<&str> = chunks[0].split("=").collect();
        let x_chunks: Vec<&str> = x_coords[1].split("..").collect();
        let x1 = x_chunks[0].parse::<isize>().unwrap();
        let x2 = x_chunks[1].parse::<isize>().unwrap();

        let y_coords: Vec<&str> = chunks[1].split("=").collect();
        let y_chunks: Vec<&str> = y_coords[1].split("..").collect();
        let y1 = y_chunks[0].parse::<isize>().unwrap();
        let y2 = y_chunks[1].parse::<isize>().unwrap();

        let c1 = Coordinate(x1,y1);
        let c2 = Coordinate(x2,y2);

        Ok(Rectangle(c1, c2))
    }
    else { Err(()) }
}

fn calculate_high_point(area: Rectangle) -> isize {
    let origin = Coordinate(0,0);
    let vy = isize::abs(area.lowest_point())-1;
    let velocity = Velocity(0,vy);
    let mut stepped_coord = origin.clone();
    let mut prev_y = stepped_coord.1;
    let mut step = 0isize;

    loop {
        stepped_coord = stepped_coord.add_velocity(velocity.step(step));

        if stepped_coord.1 < prev_y { return prev_y; }
        
        prev_y = stepped_coord.1;
        step += 1;
    }
}

fn calculate_all_velocities(area: Rectangle) -> usize {
    let origin = Coordinate(0,0);
    let mut velocities = 0usize;

    // first, find x1
    let mut x1 = 1isize;

    loop {
        let velocity = Velocity(x1, 0);
        let mut coord = origin.clone();
        let mut last_x = coord.0;
        let mut found = false;
        let mut step = 0isize;

        loop {
            coord = coord.add_velocity(velocity.step(step));

            if coord.0 == last_x { break; }
            if area.contains_x(coord) { found = true; continue; }

            last_x = coord.0;
            step += 1;
        }

        if found { break; }

        x1 += 1;
    }

    // now we can get all velocities
    let x2 = area.furthest_point();
    let y1 = area.lowest_point();
    let y2 = isize::abs(y1)-1;

    for y in y1..=y2 {
        for x in x1..=x2 {
            let velocity = Velocity(x,y);
            let mut coord = origin.clone();
            let mut step = 0isize;

            loop {
                coord = coord.add_velocity(velocity.step(step));

                if area.contains(coord) { velocities += 1; break; }
                if area.missed(coord) { break; }

                step += 1;
            }
        }
    }

    velocities
}

fn part1() {
    if let Ok(area) = read_area() {
        println!("{}", calculate_high_point(area));
    }
    else { panic!("couldn't read area!"); }
}

fn part2() {
    if let Ok(area) = read_area() {
        println!("{}", calculate_all_velocities(area));
    }
    else { panic!("couldn't read area!"); }
}

fn main() {
    // part1();
    part2();
}
