use std::io;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(isize,isize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Pixel {
    Light,
    Dark,
}
impl Pixel {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Light,
            '.' => Self::Dark,
            _ => panic!("bad pixel character"),
        }
    }
    fn to_int(&self) -> usize {
        match self {
            Self::Light => 1,
            Self::Dark => 0,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Light => '#',
            Self::Dark => '.',
        }
    }
    fn enhance(&self) -> usize {
        match self {
            Self::Light => 511,
            Self::Dark => 0,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Image {
    pixels: HashMap<Coordinate, Pixel>,
    fill: Pixel,
    min: Coordinate,
    max: Coordinate,
}
impl Image {
    fn new() -> Self {
        let pixels = HashMap::<Coordinate, Pixel>::new();
        let fill = Pixel::Dark;
        let min = Coordinate(0,0);
        let max = min;

        Self { pixels, fill, min, max }
    }
    fn get(&self, c: Coordinate) -> Pixel {
        *self.pixels.get(&c).or(Some(&self.fill)).unwrap()
    }
    fn set(&mut self, c: Coordinate, p: Pixel) {
        self.pixels.insert(c, p);

        if c.0 < self.min.0 {
            self.min.0 = c.0;
        }
        else if c.0+1 > self.max.0 {
            self.max.0 = c.0+1;
        }

        if c.1 < self.min.1 {
            self.min.1 = c.1;
        }
        else if c.1+1 > self.max.1 {
            self.max.1 = c.1+1;
        }
    }
    fn enhance(&self, c: Coordinate, algorithm: &Vec<Pixel>) -> Pixel {
        let mut bits = Vec::<usize>::new();

        for y in (c.1-1)..=(c.1+1) {
            for x in (c.0-1)..=(c.0+1) {
                bits.push(self.get(Coordinate(x,y)).to_int());
            }
        }

        let mut index = 0usize;

        for bit in &bits {
            index <<= 1;
            index |= bit;
        }

        algorithm[index]
    }
    fn light_pixels(&self) -> usize {
        self.pixels.values().filter(|&x| *x == Pixel::Light).count()
    }
    fn print(&self) {
        for y in self.min.1..self.max.1 {
            for x in self.min.0..self.max.0 {
                print!("{}", self.get(Coordinate(x,y)).to_char());
            }

            println!("");
        }
    }
}

fn enhance_image(image: &Image, algorithm: &Vec<Pixel>, steps: usize) -> Image {
    let mut result = image.clone();
    
    for _ in 0..steps {
        let mut new_image = Image::new();
        let start = Coordinate(result.min.0-1,result.min.1-1);
        let end = Coordinate(result.max.0+1,result.max.1+1);

        for y in start.1..end.1 {
            for x in start.0..end.0 {
                let coord = Coordinate(x,y);
                
                new_image.set(coord, result.enhance(coord, &algorithm));
            }
        }

        new_image.fill = algorithm[result.fill.enhance()];

        result = new_image
    }

    result
}

fn read_image() -> Result<(Vec<Pixel>, Image), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut algorithm = Vec::<Pixel>::new();
    let mut image = Image::new();
    let mut y = 0isize;

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 { return Err(()); }
        
        algorithm = buffer.trim().chars().map(|x| Pixel::from_char(x)).collect();
        buffer.clear();
    }

    if algorithm.len() != 512 { return Err(()); }

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size != 1 { return Err(()); }
    }
    
    buffer.clear();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        let pixels: Vec<Pixel> = buffer.trim().chars().map(|x| Pixel::from_char(x)).collect();

        for x in 0..pixels.len() {
            let coord = Coordinate(x as isize, y);
            image.set(coord, pixels[x]);
        }

        y += 1;
        buffer.clear();
    }

    if image.pixels.len() == 0 { Err(()) }
    else { Ok((algorithm, image)) }
}

fn part1() {
    if let Ok((algorithm, mut image)) = read_image() {
        image = enhance_image(&image, &algorithm, 2);
        println!("{}", image.light_pixels());
    }
    else { panic!("couldn't read image!"); }
}

fn part2() {
    if let Ok((algorithm, mut image)) = read_image() {
        image = enhance_image(&image, &algorithm, 50);
        println!("{}", image.light_pixels());
    }
    else { panic!("couldn't read image!"); }
}

fn main() {
    // part1();
    part2();
}
