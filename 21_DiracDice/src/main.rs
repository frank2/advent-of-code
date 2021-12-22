use std::io;
use std::collections::VecDeque;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct DeterministicDice {
    value: usize,
}
impl DeterministicDice {
    fn new() -> Self {
        Self { value: 0 }
    }
    fn roll(&mut self) -> usize {
        let mut values = Vec::<usize>::new();

        for _ in 0..3 {
            let value = self.value;
            self.value += 1;
            values.push(value%100+1)
        }
        
        values.iter().sum()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PlayerState {
    pos: usize,
    score: usize,
}
impl PlayerState {
    fn new(pos: usize) -> Self {
        Self { pos: pos-1, score: 0 }
    }
    fn play(&mut self, roll: usize) -> bool {
        self.pos += roll;
        self.pos %= 10;
        self.score += self.pos+1;

        self.score >= 21
    }
}
        
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct DiracState {
    p1: PlayerState,
    p2: PlayerState,
    turn: bool,
    roll: usize,
    count: usize,
}
impl DiracState {
    fn new(p1: usize, p2: usize) -> Self {
        Self {
            p1: PlayerState::new(p1),
            p2: PlayerState::new(p2),
            turn: true,
            roll: 0,
            count: 1,
        }
    }
    fn roll(&self) -> Vec<DiracState> {
        let turn = !self.turn;
        let roll_multipliers = [1,3,6,7,6,3,1];
        let mut result = Vec::<DiracState>::new();

        for roll in 3..=9 {
            result.push(Self {
                p1: self.p1,
                p2: self.p2,
                turn: turn,
                roll: roll,
                count: self.count*roll_multipliers[roll-3]
            });
        }

        result
    }
    fn play(&mut self) -> Option<usize> {
        let player;

        if self.turn { player = &mut self.p2; }
        else { player = &mut self.p1; }

        if player.play(self.roll) { Some(self.count) }
        else { None }
    }
}

fn play_deterministic(p1: usize, p2: usize, die: &mut DeterministicDice) -> usize {
    let mut p1_score = 0usize;
    let mut p2_score = 0usize;

    let mut p1_position = p1-1;
    let mut p2_position = p2-1;

    loop {
        p1_position += die.roll();
        p1_position %= 10;
        p1_score += p1_position+1;

        if p1_score >= 1000 { return p2_score; }

        p2_position += die.roll();
        p2_position %= 10;
        p2_score += p2_position+1;

        if p2_score >= 1000 { return p1_score; }
    }
}

fn play_dirac(p1: usize, p2: usize) -> usize {
    let init_state = DiracState::new(p1,p2);
    let mut queue = VecDeque::<DiracState>::new();
    let mut p1_wins = 0usize;
    let mut p2_wins = 0usize;

    init_state.roll().iter().copied().for_each(|x| queue.push_back(x));

    while let Some(mut state) = queue.pop_front() {
        let result = state.play();

        if result.is_none() {
            state.roll().iter().copied().for_each(|x| queue.push_back(x));
            continue;
        }
        
        let wins = result.unwrap();
        let player = state.turn;
        let score;

        if player { score = &mut p2_wins; }
        else { score = &mut p1_wins; }

        *score += wins;
    }

    if p1_wins > p2_wins { p1_wins }
    else { p2_wins }
}

fn read_positions() -> Result<(usize, usize), ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut p1_position = 0usize;
    let mut p2_position = 0usize;

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { return Err(()); }
        p1_position = buffer.trim().replace("Player 1 starting position: ", "").parse().unwrap();
        buffer.clear();
    }

    if p1_position <= 0usize || p1_position > 10 { return Err(()); }

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { return Err(()); }
        p2_position = buffer.trim().replace("Player 2 starting position: ", "").parse().unwrap();
        buffer.clear();
    }

    if p2_position <= 0usize || p2_position > 10 { return Err(()); }

    Ok((p1_position, p2_position))
}

fn part1() {
    if let Ok((p1,p2)) = read_positions() {
        let mut dice = DeterministicDice::new();
        let score = play_deterministic(p1,p2,&mut dice);
        
        println!("{}", score*dice.value);
    }
    else { panic!("couldn't read positions!"); }
}

fn part2() {
    if let Ok((p1,p2)) = read_positions() {
        println!("{}", play_dirac(p1,p2));
    }
    else { panic!("couldn't read positions!"); }
}

fn main() {
    // part1();
    part2();
}
