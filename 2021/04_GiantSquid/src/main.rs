use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Coordinate(usize, usize);

#[derive(Clone, Eq, PartialEq, Debug)]
struct Board {
    board: [[u8; 5]; 5],
    coords: HashMap<u8, Coordinate>,
    winners: HashSet<Coordinate>,
}
impl Board {
    pub fn read_board() -> Result<Self, ()> {
        let mut buffer = String::new();
        let mut board = [[0u8; 5]; 5];
        let mut coords = HashMap::<u8, Coordinate>::new();
        let winners = HashSet::<Coordinate>::new();
        let stdin = io::stdin();

        for y in 0..5 {
            if let Ok(size) = stdin.read_line(&mut buffer) {
                if size == 0 { return Err(()); }
                
                let row: Vec<u8> = buffer.trim()
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse().unwrap())
                    .collect();

                for x in 0..row.len() {
                    board[y][x] = row[x];
                    coords.insert(row[x], Coordinate(x, y));
                }

                buffer.clear();
            }
            else { return Err(()); }
        }

        Ok(Self { board, coords, winners })
    }

    pub fn call(&mut self, ball: u8) -> Option<u32> {
        if let Some(found) = self.coords.get(&ball) {
            self.winners.insert(*found);

            let x = found.0;
            let y = found.1;
            let mut winner = true;

            for sx in 0..5 {
                if x == sx { continue; }
                if !self.winners.contains(&Coordinate(sx, y)) { winner = false; break; }
            }

            if winner { return Some(self.sum_losers() * (ball as u32)); }

            winner = true;

            for sy in 0..5 {
                if y == sy { continue; }
                if !self.winners.contains(&Coordinate(x, sy)) { winner = false; break; }
            }

            if winner { return Some(self.sum_losers() * (ball as u32)); }
        }

        None
    }

    fn sum_losers(&self) -> u32 {
        let mut sum = 0u32;
        
        for y in 0..5 {
            for x in 0..5 {
                let c = Coordinate(x, y);

                if self.winners.contains(&c) { continue; }

                sum += self.board[y][x] as u32;
            }
        }

        sum
    }
}

fn read_bingo() -> Result<(Vec<u8>, Vec<Board>), ()> {
    let mut buffer = String::new();
    let mut boards = Vec::<Board>::new();
    let stdin = io::stdin();

    let result = stdin.read_line(&mut buffer);

    if result.is_err() { return Err(()); }

    let calls: Vec<u8> = buffer.trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        
        if let Ok(board) = Board::read_board() { boards.push(board); }
        else { return Err(()); }
    }

    Ok((calls, boards))
}
    
fn part1() {
    if let Ok((calls, mut boards)) = read_bingo() {
        for call in calls {
            for board in &mut boards {
                if let Some(bingo) = board.call(call) {
                    println!("{}", bingo);
                    return;
                }
            }
        }
    }
    else { panic!("couldn't read bingo data"); }
}

fn part2() {
    if let Ok((calls, mut boards)) = read_bingo() {
        let mut winner = 0u32;
        let mut winners = HashSet::<usize>::new();
    
        for call in calls {
            for i in 0..boards.len() {
                if winners.contains(&i) { continue; }
                
                let board = &mut boards[i];
                
                if let Some(bingo) = board.call(call) {
                    winner = bingo;
                    winners.insert(i);
                }
            }
        }

        println!("{}", winner);
    }
    else { panic!("couldn't read bingo data"); }
}

fn main() {
    // part1();
    part2();
}
