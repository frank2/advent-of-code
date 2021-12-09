use std::io;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(usize, usize);

#[derive(Clone, Eq, PartialEq, Debug)]
struct HeightMap {
    data: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}
impl HeightMap {
    pub fn read() -> Result<Self, ()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut data = Vec::<Vec<u32>>::new();

        while let Ok(size) = stdin.read_line(&mut buffer) {
            if size == 0 { break; }
            if size == 1 { continue; }

            data.push(buffer.trim()
                      .as_bytes()
                      .iter()
                      .map(|&x| (x - '0' as u8) as u32)
                      .collect());

            buffer.clear();
        }
            
        Ok(Self { data: data.clone(), height: data.len(), width: data[0].len() })
    }

    pub fn get_point(&self, coord: Coordinate) -> u32 {
        self.data[coord.1][coord.0]
    }

    pub fn get_deltas(&self, coord: Coordinate) -> Vec<Option<i32>> {
        let mut dl: Option<i32> = None;
        let mut dr: Option<i32> = None;
        let mut du: Option<i32> = None;
        let mut dd: Option<i32> = None;
        let height_value = self.get_point(coord);
        let x = coord.0;
        let y = coord.1;

        if x != 0 { dl = Some(delta(height_value, self.data[y][x-1])); }
        if x+1 != self.width { dr = Some(delta(height_value, self.data[y][x+1])); }
        if y != 0 { du = Some(delta(height_value, self.data[y-1][x])); }
        if y+1 != self.height { dd = Some(delta(height_value, self.data[y+1][x])); }

        [dl, dr, du, dd].iter().cloned().collect()
    }

    pub fn low_points(&self) -> Vec<Coordinate> {
        let mut coords = Vec::<Coordinate>::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinate(x,y);
                
                let valid_depths: Vec<i32> = self.get_deltas(coord).iter()
                    .filter(|&x| x.is_some())
                    .map(|&x| x.unwrap())
                    .collect();

                if valid_depths.iter().filter(|&x| *x < 0).count() == valid_depths.len() {
                    coords.push(coord);
                }
            }
        }

        coords
    }

    pub fn basins(&self) -> Vec<HashSet<Coordinate>> {
        let mut basins = Vec::<HashSet::<Coordinate>>::new();
        let low_points = self.low_points();

        for low_point in low_points {
            let mut basin = HashSet::<Coordinate>::new();

            self.find_basin(low_point, &mut basin);

            basins.push(basin);
        }

        basins
    }

    fn find_basin(&self, coord: Coordinate, basin: &mut HashSet::<Coordinate>) {
        if basin.contains(&coord) { return; }
        
        let height_value = self.get_point(coord);

        if height_value == 9 { return; }

        basin.insert(coord);

        if coord.0 != 0 { self.find_basin(Coordinate(coord.0-1, coord.1), basin); }
        if coord.1 != 0 { self.find_basin(Coordinate(coord.0, coord.1-1), basin); }
        if coord.0+1 != self.width { self.find_basin(Coordinate(coord.0+1, coord.1), basin); }
        if coord.1+1 != self.height { self.find_basin(Coordinate(coord.0, coord.1+1), basin); }
    }
}

fn delta(a: u32, b: u32) -> i32 {
    a as i32 - b as i32
}

fn part1() {
    if let Ok(height_map) = HeightMap::read() {
        println!("{:?}", height_map.low_points()
                 .iter()
                 .map(|&x| height_map.get_point(x)+1)
                 .sum::<u32>());
    }
}

fn part2() {
    if let Ok(height_map) = HeightMap::read() {
        let mut basin_sizes: Vec<usize> = height_map.basins()
            .iter()
            .map(|x| x.len())
            .collect();

        basin_sizes.sort();
        basin_sizes.reverse();
        
        println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]); 
    }
}

fn main() {
    // part1();
    part2();
}
