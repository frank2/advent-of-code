use std::io;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl Amphipod {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Amber,
            "B" => Self::Bronze,
            "C" => Self::Copper,
            "D" => Self::Desert,
            _ => panic!("bad amphipod string"),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Amber => "A".to_string(),
            Self::Bronze => "B".to_string(),
            Self::Copper => "C".to_string(),
            Self::Desert => "D".to_string(),
        }
    }
    fn as_sideroom(&self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }
    fn as_cost(&self) -> usize {
        10usize.pow(self.as_sideroom() as u32)
    }
}

type RoomId = usize;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Room {
    occupant: Option<Amphipod>,
    rooms: Vec<RoomId>,
}
impl Room {
    fn new(occupant: Option<Amphipod>) -> Self {
        Self { occupant: occupant, rooms: Vec::<RoomId>::new() }
    }
    fn has_room(&self, room: RoomId) -> bool { self.rooms.contains(&room) }
    fn join(&mut self, room: RoomId) { if !self.has_room(room) { self.rooms.push(room); } }
    fn is_occupied(&self) -> bool { self.occupant.is_some() }
    fn set_occupant(&mut self, o: Option<Amphipod>) { self.occupant = o; }
    fn to_string(&self) -> String {
        if self.occupant.is_none() { ".".to_string() }
        else { self.occupant.clone().unwrap().to_string() }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Burrow {
    rooms: Vec<Room>,
    hallway: Vec<RoomId>,
    siderooms: Vec<Vec<RoomId>>,
}
impl Burrow {
    fn new() -> Self {
        Self {
            rooms: Vec::<Room>::new(),
            hallway: Vec::<RoomId>::new(),
            siderooms: Vec::<Vec<RoomId>>::new()
        }
    }
    fn get_room(&self, room_id: RoomId) -> &Room {
        if room_id > self.rooms.len() { panic!("bad room id: {}", room_id); }
        
        &self.rooms[room_id]
    }
    fn get_room_mut(&mut self, room_id: RoomId) -> &mut Room {
        if room_id > self.rooms.len() { panic!("bad room id: {}", room_id); }
        
        &mut self.rooms[room_id]
    }
    fn get_sideroom(&self, amphipod: Amphipod) -> Option<RoomId> {
        let offset = amphipod.as_sideroom();

        for i in (0..self.siderooms.len()).rev() {
            let room = self.get_room(self.siderooms[i][offset]);

            if !room.is_occupied() || room.occupant.clone().unwrap() != amphipod {
                return Some(self.siderooms[i][offset]);
            }
        }

        None
    }

    fn add_room(&mut self, occupant: Option<Amphipod>) -> RoomId {
        self.rooms.push(Room::new(occupant));

        self.rooms.len()-1
    }
    fn join_rooms(&mut self, room_left: RoomId, room_right: RoomId) {
        self.get_room_mut(room_left).join(room_right);
        self.get_room_mut(room_right).join(room_left);
    }

    fn add_hallway(&mut self, room: RoomId) {
        self.hallway.push(room);

        if self.hallway.len() == 1 { return; }

        let this_room = self.hallway.len()-1;
        let prev_room = self.hallway.len()-2;

        self.join_rooms(self.hallway[this_room], self.hallway[prev_room]);
    }
    fn add_sideroom(&mut self, room: RoomId) {
        if self.siderooms.len() == 0 || self.siderooms[self.siderooms.len()-1].len() == 4 {
            self.siderooms.push(Vec::<RoomId>::new());
        }

        let sideroom_id = self.siderooms.len()-1;
        let sideroom_offset = self.siderooms[sideroom_id].len();
        self.siderooms[sideroom_id].push(room);

        let linked_id;

        if sideroom_id == 0 { linked_id = 2+sideroom_offset*2; }
        else { linked_id = self.siderooms[sideroom_id-1][sideroom_offset]; }

        self.join_rooms(room, linked_id);
    }

    fn is_solved(&self) -> bool {
        for row_id in 0..self.siderooms.len() {
            for offset in 0..self.siderooms[row_id].len() {
                let room_id = self.siderooms[row_id][offset];
                let room = self.get_room(room_id);

                if !room.is_occupied() { return false; }

                let occupant = room.occupant.clone().unwrap().as_sideroom();

                if occupant != offset { return false; }
            }
        }

        true
    }
    
    fn move_occupant(&mut self, left: RoomId, right: RoomId) {
        let left_room_ro = self.get_room(left).clone();
        let right_room_ro = self.get_room(right).clone();

        let left_room = self.get_room_mut(left);
        left_room.occupant = right_room_ro.occupant.clone();

        let right_room = self.get_room_mut(right);
        right_room.occupant = left_room_ro.occupant.clone();
    }

    fn find_path(&self, left: RoomId, right: RoomId) -> Option<usize> {
        let left_room = self.get_room(left);
        let right_room = self.get_room(right);

        if !left_room.is_occupied() || right_room.is_occupied() || right_room.rooms.len() >= 3 { return None; }
        let path_cost = left_room.occupant.clone().unwrap().as_cost();
        let mut visited = HashSet::<RoomId>::new();
        let mut stack: Vec<(usize, RoomId)> = left_room.rooms.iter().map(|x| (path_cost, *x)).collect();

        while let Some((cost, room_id)) = stack.pop() {
            if room_id == right { return Some(cost); }
            
            if visited.contains(&room_id) { continue; }
            
            let room = self.get_room(room_id);
            visited.insert(room_id);

            if room.is_occupied() { continue; }

            for next_room in room.rooms.iter().filter(|x| !visited.contains(x)) {
                stack.push((cost+path_cost, *next_room));
            }
        }

        None
    }

    fn valid_moves(&self) -> Vec<(usize, RoomId, RoomId)> {
        let mut moves = Vec::<(usize, RoomId, RoomId)>::new();
        
        // first, get the amphipods in the hallway and calculate the distance to their designated rooms
        for room_id in &self.hallway {
            let room = self.get_room(*room_id);
            if !room.is_occupied() { continue; }
            
            let occupant = room.occupant.clone().unwrap();
            
            if let Some(sideroom) = self.get_sideroom(occupant) {
                if let Some(cost) = self.find_path(*room_id, sideroom) {
                    moves.push((cost, *room_id, sideroom))
                }
            }
        }

        // next, get the amphipods in the siderooms which don't belong and move them into either the hallway or their sideroom
        for row_id in 0..self.siderooms.len() {
            let row = &self.siderooms[row_id];
            
            for amphipod_offset in 0..row.len() {
                let room_id = &row[amphipod_offset];
                let room = self.get_room(*room_id);

                if !room.is_occupied() { continue; }

                let amphipod = room.occupant.clone().unwrap();

                if let Some(sideroom) = self.get_sideroom(amphipod) {
                    // this can move directly into its sideroom
                    if let Some(cost) = self.find_path(*room_id, sideroom) {
                        moves.push((cost, *room_id, sideroom));
                    }
                    // this must be moved into the hallway
                    else {
                        for hallway_id in &self.hallway {
                            if let Some(cost) = self.find_path(*room_id, *hallway_id) {
                                moves.push((cost, *room_id, *hallway_id));
                            }
                        }
                    }
                }
            }
        }

        moves
    }
            
    fn print(&self) {
        let hallway_len = self.hallway.len();

        println!("{}", (0..hallway_len+2).map(|_| "#").collect::<String>());

        print!("#");

        for hallway_id in &self.hallway {
            let room = self.get_room(*hallway_id);
            print!("{}", room.to_string());
        }

        println!("#");

        let padding_count = self.get_room(self.siderooms[0][0]).rooms[0];
        
        for sideroom_row in 0..self.siderooms.len() {
            let padding;
            
            if sideroom_row == 0 {
                padding = (0..padding_count).map(|_| "#").collect::<String>();
            }
            else {
                padding = (0..padding_count).map(|_| " ").collect::<String>();
            }

            print!("{}#", padding);

            for sideroom in &self.siderooms[sideroom_row] {
                let room = self.get_room(*sideroom);
                print!("{}#", room.to_string());
            }

            println!("{}", padding);
        }

        println!("{}#{}",
               (0..padding_count).map(|_| " ").collect::<String>(),
               (0..self.siderooms[0].len()*2).map(|_| "#").collect::<String>()
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct BurrowState {
    burrow: Burrow,
    cost: usize,
    history: Vec<Burrow>,
}
impl BurrowState {
    fn new(burrow: &Burrow, cost: usize, history: &Vec<Burrow>) -> Self {
        Self { burrow: burrow.clone(), cost: cost, history: history.clone() }
    }
    fn perform_move(&self, cost: usize, from: RoomId, to: RoomId) -> Self {
        let mut history = self.history.clone();
        history.push(self.burrow.clone());

        let mut moved = self.burrow.clone();
        moved.move_occupant(from, to);

        Self::new(&moved, self.cost+cost, &history)
    }
}
impl Ord for BurrowState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for BurrowState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(burrow: &Burrow) -> BurrowState {
    let mut heap = BinaryHeap::<BurrowState>::new();
    let mut visited = HashSet::<Burrow>::new();

    heap.push(BurrowState::new(&burrow, 0, &Vec::<Burrow>::new()));

    while let Some(node) = heap.pop() {
        if node.burrow.is_solved() { return node; }
        if visited.contains(&node.burrow) { continue; }

        visited.insert(node.burrow.clone());

        for (cost, from, to) in node.burrow.valid_moves() {
            heap.push(node.perform_move(cost, from, to));
        }
    }

    panic!("couldn't solve burrow!");
}

fn read_burrow() -> Result<Burrow, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut burrow = Burrow::new();
    
    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 { continue; }

        if buffer.trim() == std::iter::repeat("#").take(buffer.len()).collect::<String>() { continue; }
        
        if burrow.hallway.len() == 0 {
            let length = buffer.trim().replace("#","").chars().count();

            for _ in 0..length {
                let room_id = burrow.add_room(None);
                burrow.add_hallway(room_id);
            }
        }
        else {
            for substr in buffer.trim().replace(" ","").split("#") {
                if substr.len() == 0 { continue; }
                    
                let room_id = burrow.add_room(Some(Amphipod::from_str(substr)));
                burrow.add_sideroom(room_id);
            }
        }
        
        buffer.clear();
    }

    if burrow.rooms.len() == 0 || burrow.siderooms.len() == 0 { Err(()) }
    else { Ok(burrow) }
}

fn main() {
    if let Ok(burrow) = read_burrow() {
        let solved = solve(&burrow);
        
        for i in 0..solved.history.len() {
            println!("state {}:", i);
            solved.history[i].print();
            println!("");
        }

        println!("solution:");
        solved.burrow.print();
        println!("");
            
        println!("cost: {}", solved.cost);
    }
    else { panic!("couldn't read burrow!"); }
}
