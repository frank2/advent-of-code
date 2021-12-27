use std::io;
use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate(usize, usize);
impl Coordinate {
    fn neighbors(&self, grid: Coordinate) -> Vec<Coordinate> {
        let (mx,my) = (grid.0 as isize,grid.1 as isize);
        let mut coords = Vec::<Coordinate>::new();

        for y in -1..=1isize {
            for x in -1..=1isize {
                if isize::abs(y) == isize::abs(x) { continue; }

                let nx = self.0 as isize + x;
                let ny = self.1 as isize + y;

                if nx < 0 || ny < 0 { continue; }
                if nx >= mx || ny >= my { continue; }

                coords.push(Coordinate(nx as usize,ny as usize));
            }
        }

        coords
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node(usize);

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashMap<Coordinate, Node>,
    grid: Coordinate,
}
impl Graph {
    fn new(grid: Coordinate) -> Self {
        Self { nodes: HashMap::<Coordinate, Node>::new(), grid: grid }
    }
    fn add_node(&mut self, coord: Coordinate, risk: usize) {
        self.nodes.insert(coord, Node(risk));
    }
    fn read() -> Result<Self, ()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut graph_data = Vec::<Vec<usize>>::new();

        while let Ok(size) = stdin.read_line(&mut buffer) {
            if size == 0 { break; }
            if size == 1 { continue; }
            
            graph_data.push(buffer.trim().chars().map(|x| x as usize - '0' as usize).collect());
            buffer.clear();
        }

        if graph_data.len() == 0 { return Err(()); }
        
        let grid = Coordinate(graph_data[0].len(), graph_data.len());
        let mut graph = Self::new(grid);

        for y in 0..graph_data.len() {
            for x in 0..graph_data[y].len() {
                let risk = graph_data[y][x];
                let coord = Coordinate(x,y);

                graph.add_node(coord, risk);
            }
        }

        Ok(graph)
    }
    fn traverse(&self) -> usize {
        // binary heap relies on the Ord trait, so we have to implement a bunch of boilerplate...
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        struct Visit {
            label: Coordinate,
            risk: usize,
        }
        impl Visit {
            fn new(label: Coordinate, risk: usize) -> Self { Self { label, risk } }
        }
        impl Ord for Visit {
            fn cmp(&self, other: &Self) -> Ordering {
                other.risk.cmp(&self.risk)
                    .then_with(|| self.label.0.cmp(&other.label.0))
                    .then_with(|| self.label.1.cmp(&other.label.1))
            }
        }
        impl PartialOrd for Visit {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let start = Coordinate(0,0);
        let end = Coordinate(self.grid.0-1,self.grid.1-1);

        let mut risks = HashMap::<Coordinate, usize>::new();
        let mut heap = BinaryHeap::<Visit>::new();

        risks.insert(start, 0);
        heap.push(Visit::new(start, 0));

        while let Some(Visit { label, risk }) = heap.pop() {
            if label == end { return risk; }
            
            for neighbor in label.neighbors(self.grid) {
                let neighbor_node = self.nodes.get(&neighbor).unwrap();
                let new_risk = risk + neighbor_node.0;
                let shorter = risks.get(&neighbor).map_or(true, |&current| new_risk < current);

                if !shorter { continue; }

                risks.insert(neighbor, new_risk);
                heap.push(Visit::new(neighbor, new_risk));
            }
        }

        *risks.get(&end).unwrap()
    }
    fn inflate(&self) -> Graph {
        let mut new_graph = self.clone();

        new_graph.grid = Coordinate(self.grid.0*5,self.grid.1*5);

        for oy in 0..self.grid.1 {
            for ox in 0..self.grid.0 {
                let original_coord = Coordinate(ox,oy);
                let original_risk = self.nodes.get(&original_coord).unwrap().0;

                for ny in 0..5usize {
                    for nx in 0..5usize {
                        if nx == 0 && ny == 0 { continue; }
                        
                        let dx = self.grid.0 * nx + original_coord.0;
                        let dy = self.grid.1 * ny + original_coord.1;
                        
                        let new_coord = Coordinate(dx,dy);
                        let new_risk = original_risk + nx + ny;
                        let mut wrapped_risk = new_risk % 10;

                        if new_risk >= 10 { wrapped_risk += new_risk / 10; }

                        new_graph.add_node(new_coord, wrapped_risk);
                    }
                }
            }
        }

        new_graph
    }
}
        
fn part1() {
    if let Ok(graph) = Graph::read() {
        println!("{}", graph.traverse());
    }
    else { panic!("couldn't read graph!"); }
}

fn part2() {
    if let Ok(graph) = Graph::read() {
        let new_graph = graph.inflate();
        println!("{}", new_graph.traverse());
    }
    else { panic!("couldn't read graph!"); }
}

fn main() {
    // part1();
    part2();
}
