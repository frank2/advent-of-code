use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, PartialEq, Debug)]
struct Image {
    pixels: Vec<Vec<char>>,
}
impl Image {
    fn new() -> Self {
        Self { pixels: Vec::<Vec<char>>::new() }
    }
    fn size(&self) -> (usize, usize) {
        if self.pixels.len() == 0 { (0,0) }
        else { (self.pixels[0].len(),self.pixels.len()) }
    }
    fn add_row(&mut self, row: &Vec<char>) {
        self.pixels.push(row.clone());
    }
    fn append_column(&mut self, image: &Image) {
        if image.pixels.len() != self.pixels.len() {
            panic!("column length does not match, can't append column");
        }
        
        for i in 0..self.pixels.len() {
            self.pixels[i].append(&mut image.pixels[i].clone());
        }
    }
    fn append_row(&mut self, image: &Image) {
        if image.pixels[0].len() != self.pixels[0].len() {
            panic!("row length does not match, can't append row");
        }

        for i in 0..image.pixels.len() {
            self.add_row(&image.pixels[i]);
        }
    }
    fn flip_x(&mut self) {
        self.pixels = self.pixels.iter().map(|x| x.iter().rev().copied().collect::<Vec<char>>()).collect();
    }
    fn flip_y(&mut self) {
        self.pixels = self.pixels.iter().rev().cloned().collect();
    }
    fn rotate_right(&mut self, rotations: usize) {
        for _ in 0..rotations {
            let old_image = self.pixels.clone();
            let size = self.size();

            for y in 0..size.1 {
                for x in 0..size.0 {
                    self.pixels[x][size.1-1-y] = old_image[y][x];
                }
            }
        }
    }
    fn rotate_left(&mut self, rotations: usize) {
        for _ in 0..rotations {
            let old_image = self.pixels.clone();
            let size = self.size();

            for y in 0..size.1 {
                for x in 0..size.0 {
                    self.pixels[size.0-1-x][y] = old_image[y][x];
                }
            }
        }
    }
    fn find_sea_monsters(&self) -> usize {
        //                   # 
        // #    ##    ##    ###
        //  #  #  #  #  #  #
        let monster_coords: Vec<(usize,usize)> = [
            (0,1),
            (1,2),
            (4,2),
            (5,1),
            (6,1),
            (7,2),
            (10,2),
            (11,1),
            (12,1),
            (13,2),
            (16,2),
            (17,1),
            (18,1),
            (18,0),
            (19,1),
        ].iter().copied().collect();
        let mut search_image = self.clone();
        let mut max_monsters = 0usize;

        for _rotation in 0..4 {
            search_image.rotate_right(1);

            for state in 0..4 {
                let mut image_state = search_image.clone();

                match state {
                    1 => image_state.flip_x(),
                    2 => image_state.flip_y(),
                    3 => { image_state.flip_x(); image_state.flip_y(); },
                    _ => (),
                }

                let size = image_state.size();
                let mut monsters = 0usize;

                for y in 0..size.1 {
                    for x in 0..size.0 {
                        let mut found = true;
                        
                        for (dx,dy) in &monster_coords {
                            let nx = x+dx;
                            let ny = y+dy;

                            if nx >= size.0 { found = false; break; }
                            if ny >= size.1 { found = false; break; }
                            if image_state.pixels[ny][nx] != '#' { found = false; break; }
                        }

                        if found { monsters += 1; }
                    }
                }

                if monsters > max_monsters { max_monsters = monsters; }
            }
        }

        max_monsters
    }
        
    fn print(&self) {
        for row in &self.pixels {
            println!("{}", row.iter().collect::<String>());
        }
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Border {
    Top,
    Right,
    Bottom,
    Left,
}
impl Border {
    fn from_int(i: usize) -> Self {
        match i {
            0 => Self::Top,
            1 => Self::Right,
            2 => Self::Bottom,
            3 => Self::Left,
            _ => panic!("bad border: {}", i),
        }
    }
    fn to_int(&self) -> usize {
        match self {
            Self::Top => 0,
            Self::Right => 1,
            Self::Bottom => 2,
            Self::Left => 3,
        }
    }
}

type TileID = usize;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Tile {
    image: Image,
    neighbors: HashMap<Border, TileID>,
}
impl Tile {
    fn new() -> Self {
        Self { image: Image::new(), neighbors: HashMap::<Border, TileID>::new() }
    }
    fn size(&self) -> (usize, usize) {
        self.image.size()
    }
    fn add_row(&mut self, row: &Vec<char>) {
        self.image.add_row(row);
    }
    fn add_neighbor(&mut self, border: Border, id: TileID) {
        self.neighbors.insert(border, id);
    }
    fn has_neighbor(&self, border: Border) -> bool {
        self.neighbors.get(&border).is_some()
    }
    fn get_neighbor(&self, border: Border) -> Option<&TileID> {
        self.neighbors.get(&border)
    }
    fn get_border(&self, border: Border) -> Vec<char> {
        let size = self.size();
        
        match border {
            Border::Top => self.image.pixels[0].clone(),
            Border::Right => (0..size.1).map(|y| self.image.pixels[y][size.0-1]).collect(),
            Border::Bottom => self.image.pixels[size.1-1].clone(),
            Border::Left => (0..size.1).map(|y| self.image.pixels[y][0]).collect(),
        }
    }
    fn borders(&self) -> Vec<Vec<char>> {
        [
            self.get_border(Border::Top),
            self.get_border(Border::Right),
            self.get_border(Border::Bottom),
            self.get_border(Border::Left),
        ].iter().cloned().collect()
    }
    fn find_border(&self, border: &Vec<char>) -> Option<Border> {
        let borders = self.borders();
        let mut index = borders.iter().position(|x| x == border);

        if index.is_none() {
            index = borders.iter().position(|x| *x == border.iter().rev().copied().collect::<Vec<char>>());
        }

        if index.is_none() { None }
        else { Some(Border::from_int(index.unwrap())) }
    }
    fn find_neighbor(&self, id: TileID) -> Option<Border> {
        self.neighbors.iter().filter(|&(_,x)| *x == id).map(|(x,_)| *x).next()
    }
    fn flip_x(&mut self) {
        let old_neighbors = self.neighbors.clone();

        self.image.flip_x();
        
        self.neighbors.remove(&Border::Left);
        self.neighbors.remove(&Border::Right);

        if let Some(left_neighbor) = old_neighbors.get(&Border::Left) {
            self.neighbors.insert(Border::Right, *left_neighbor);
        }

        if let Some(right_neighbor) = old_neighbors.get(&Border::Right) {
            self.neighbors.insert(Border::Left, *right_neighbor);
        }
    }
    fn flip_y(&mut self) {
        let old_neighbors = self.neighbors.clone();

        self.image.flip_y();
        
        self.neighbors.remove(&Border::Top);
        self.neighbors.remove(&Border::Bottom);

        if let Some(top_neighbor) = old_neighbors.get(&Border::Top) {
            self.neighbors.insert(Border::Bottom, *top_neighbor);
        }

        if let Some(bottom_neighbor) = old_neighbors.get(&Border::Bottom) {
            self.neighbors.insert(Border::Top, *bottom_neighbor);
        }
    }
    fn rotate_right(&mut self, rotations: usize) {
        self.image.rotate_right(rotations);
        
        for _ in 0..rotations {
            let old_neighbors = self.neighbors.clone();
            self.neighbors = HashMap::<Border, TileID>::new();

            for (border, neighbor) in old_neighbors {
                let border_rotated = (border.to_int()+1) % 4;
                self.neighbors.insert(Border::from_int(border_rotated), neighbor);
            }
        }
    }
    fn rotate_left(&mut self, rotations: usize) {
        self.image.rotate_left(rotations);

        for _ in 0..rotations {
            let old_neighbors = self.neighbors.clone();
            self.neighbors = HashMap::<Border, TileID>::new();

            for (border, neighbor) in old_neighbors {
                let border_rotated = if border.to_int() == 0 { 3 } else { border.to_int()-1 };
                self.neighbors.insert(Border::from_int(border_rotated), neighbor);
            }
        }
    }
    fn strip_border(&self) -> Image {
        let size = self.size();
        let mut result = Image::new();

        for y in 1..size.1-1 {
            let mut row = Vec::<char>::new();
            
            for x in 1..size.0-1 {
                row.push(self.image.pixels[y][x]);
            }

            result.add_row(&row);
        }

        result
    }
    fn print(&self) {
        self.image.print();
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Puzzle {
    tiles: HashMap<TileID, Tile>
}
impl Puzzle {
    fn new() -> Self {
        Self { tiles: HashMap::<TileID, Tile>::new() }
    }
    fn add_tile(&mut self, id: TileID, tile: &Tile) {
        self.tiles.insert(id, tile.clone());
    }
    fn get_tile(&self, id: TileID) -> &Tile {
        self.tiles.get(&id).unwrap()
    }
    fn get_tile_mut(&mut self, id: TileID) -> &mut Tile {
        self.tiles.get_mut(&id).unwrap()
    }
    fn assemble(&mut self) {
        let mut border_map = HashMap::<Vec<char>, HashSet<TileID>>::new();

        for (id, tile) in &self.tiles {
            for border in tile.borders() {
                let mut border_entry = border_map.get_mut(&border);

                if border_entry.is_none() {
                    border_entry = border_map.get_mut(&border.iter().rev().copied().collect::<Vec<char>>());
                }

                if border_entry.is_none() {
                    border_map.insert(border.clone(), HashSet::<TileID>::new());
                    border_entry = border_map.get_mut(&border);
                }

                border_entry.unwrap().insert(*id);
            }
        }

        for (border, matches) in &border_map {
            if matches.len() < 2 { continue; }

            let match_vec: Vec<TileID> = matches.iter().copied().collect();
            let (left, right) = (match_vec[0],match_vec[1]);

            let left_tile = self.get_tile_mut(left);
            let left_border = left_tile.find_border(border).unwrap();
            left_tile.add_neighbor(left_border, right);

            let right_tile = self.get_tile_mut(right);
            let right_border = right_tile.find_border(border).unwrap();
            right_tile.add_neighbor(right_border, left);
        }
    }
    fn solve(&mut self) {
        // first, start with a corner piece-- any corner piece is fine
        
        let (corner_id, corner) = self.tiles.iter_mut().filter(|(_,x)| x.neighbors.len() == 2).next().unwrap();
        let mut borders: Vec<usize> = corner.neighbors.keys().map(|x| x.to_int()).collect();
        
        borders.sort();

        if borders[0] == 0 && borders[1] == 3 {
            borders.reverse();
        }

        // if this isn't a right/bottom piece (upper-left corner), rotate it until it is.
        
        let rotations = Border::Right.to_int() as isize - borders[0] as isize;

        if rotations < 0 { corner.rotate_left(isize::abs(rotations) as usize); }
        else { corner.rotate_right(rotations as usize); }

        // complete the row until the end is reached, repeat until no more rows can be produced

        let mut fixed_image = vec![vec![*corner_id]];
        let mut prev_id = *corner_id;
        let mut next = *corner.get_neighbor(Border::Right).unwrap();

        loop {
            let prev_tile = self.get_tile(prev_id);

            // if the previous tile had a right neighbor, then we still have a row to complete.
            // otherwise, prepare the next row.
            let left_neighbor;

            if prev_tile.get_neighbor(Border::Right).is_some() {
                left_neighbor = Some(prev_id);
            }
            else {
                left_neighbor = None;
                fixed_image.push(Vec::<TileID>::new());
            }

            let current_row = fixed_image.len()-1;

            // if current_row is 0, we're in the top row and don't have a top neighbor.
            // otherwise, get the top neighbor.
            let top_neighbor;
            
            if current_row == 0 { top_neighbor = None; }
            else { top_neighbor = Some(fixed_image[current_row-1][fixed_image[current_row].len()]); }
            
            let tile_id = next;
            let tile = self.get_tile(tile_id);

            let left_match;
            let top_match;

            // find the neighbor that matches either the upper neighbor or the left neighbor

            if left_neighbor.is_some() { left_match = tile.find_neighbor(left_neighbor.unwrap()); }
            else { left_match = None; }

            if top_neighbor.is_some() { top_match = tile.find_neighbor(top_neighbor.unwrap()); }
            else { top_match = None; }

            let target_border;
            let rotations;
            
            if left_match.is_none() {
                target_border = Border::Top;
                rotations = Border::Top.to_int() as isize - top_match.unwrap().to_int() as isize;
            }
            else {
                target_border = Border::Left;
                rotations = Border::Left.to_int() as isize - left_match.unwrap().to_int() as isize;
            }

            // rotate the tile

            let tile_mut = self.get_tile_mut(tile_id);

            if rotations < 0 { tile_mut.rotate_left(isize::abs(rotations) as usize); }
            else { tile_mut.rotate_right(rotations as usize); }

            // if this is the first entry to the row, the leftmost border neighbor should be none. otherwise, flip it.
            // otherwise, check the top neighbor. it should be none if it's the first row, or the top neighbor otherwise.

            if target_border == Border::Top {
                if tile_mut.has_neighbor(Border::Left) { tile_mut.flip_x(); }
            }
            else if target_border == Border::Left {
                if tile_mut.has_neighbor(Border::Top) &&
                    (top_match.is_none() || *tile_mut.get_neighbor(Border::Top).unwrap() != top_neighbor.unwrap()) || 
                    !tile_mut.has_neighbor(Border::Top) && top_match.is_some() {
                        tile_mut.flip_y();
                }
            }

            prev_id = tile_id;
            fixed_image[current_row].push(tile_id);
            
            let tile = self.get_tile(tile_id);

            // if the current tile has a right neighbor, iterate to that
            // otherwise, check the leftmost member of the row.
            // if it has a bottom neighbor, iterate to that. if not, we've solved the puzzle.

            if tile.has_neighbor(Border::Right) {
                next = *tile.get_neighbor(Border::Right).unwrap();
            }
            else {
                let leftmost = fixed_image[current_row][0];
                let leftmost_tile = self.get_tile(leftmost);

                if leftmost_tile.has_neighbor(Border::Bottom) {
                    next = *leftmost_tile.get_neighbor(Border::Bottom).unwrap();
                }
                else {
                    break;
                }
            }
        }
    }
    fn render(&self) -> Image {
        let orientation: HashSet<Border> = [Border::Right, Border::Bottom].iter().copied().collect();
        let (corner_id, corner) = self.tiles.iter()
            .filter(|(_,x)| x.neighbors.keys().copied().collect::<HashSet<Border>>() == orientation)
            .next()
            .unwrap();

        let mut rendered_tiles = vec![vec![(*corner_id, corner.strip_border())]];
        let mut next_id = *corner.get_neighbor(Border::Right).unwrap();

        loop {
            let current_row = rendered_tiles.len()-1;
            let next_tile = self.get_tile(next_id);

            rendered_tiles[current_row].push((next_id, next_tile.strip_border()));

            if next_tile.has_neighbor(Border::Right) {
                next_id = *next_tile.get_neighbor(Border::Right).unwrap();
            }
            else {
                let first_id = rendered_tiles[current_row][0].0;
                let first_tile = self.get_tile(first_id);

                if !first_tile.has_neighbor(Border::Bottom) { break; }
                else {
                    rendered_tiles.push(Vec::<(TileID, Image)>::new());
                    next_id = *first_tile.get_neighbor(Border::Bottom).unwrap();
                }
            }
        }

        let mut rendered_rows = Vec::<Image>::new();

        for y in 0..rendered_tiles.len() {
            let row = &rendered_tiles[y];
            let mut row_image = row[0].1.clone();

            for x in 1..row.len() {
                row_image.append_column(&row[x].1);
            }

            rendered_rows.push(row_image);
        }

        let mut rendered_image = rendered_rows[0].clone();

        for i in 1..rendered_rows.len() {
            rendered_image.append_row(&rendered_rows[i]);
        }

        rendered_image
    }
}

fn read_tiles() -> Result<Puzzle, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut result = Puzzle::new();
    let mut current_id = 0usize;
    let mut current_tile = Tile::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 || size == 1 {
            if current_tile.size().0 > 0 && current_tile.size().1 > 0 {
                result.add_tile(current_id, &current_tile);
                current_tile = Tile::new();
            }
            
            buffer.clear();

            if size == 0 { break; }
            else { continue; }
        }

        if buffer.contains("Tile") {
            current_id = buffer.trim().replace("Tile ","").replace(":","").parse().unwrap();
        }
        else {
            current_tile.add_row(&buffer.trim().chars().collect());
        }

        buffer.clear();
    }

    if result.tiles.len() == 0 { Err(()) }
    else { Ok(result) }
}

fn part1() {
    if let Ok(mut puzzle) = read_tiles() {
        puzzle.assemble();
        
        println!("{}", puzzle.tiles
                 .iter()
                 .filter(|(_,tile)| tile.neighbors.len() == 2)
                 .map(|(id,_)| *id)
                 .product::<usize>());
    }
    else { panic!("couldn't read tiles!"); }
}

fn part2() {
    if let Ok(mut puzzle) = read_tiles() {
        puzzle.assemble();
        puzzle.solve();
        
        let image = puzzle.render();
        let monsters = image.find_sea_monsters();
        let roughness = image.pixels.iter().map(|x| x.iter().filter(|&y| *y=='#').count()).sum::<usize>() - (monsters*15);

        println!("{}", roughness);
    }
    else { panic!("couldn't read tiles!"); }
}

fn main() {
    // part1();
    part2();
}
