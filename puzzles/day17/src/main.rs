use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let input = include_str!("../input_test.txt");

    // println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.shortest_path(0, 0);
    grid.print_lowest_heat_loss();
    let x = grid.get(grid.grid.len() as i32 - 1, grid.grid[0].len() as i32 -1);
    println!("x: {:?}", x);
    0
}

fn second_part(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut path = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !c.is_numeric() {
                path.push((y, x));
            }
        }
    }

    0
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Node>>
}

#[derive(Debug)]
struct Node {
    heat_loss_constant: usize,
    possibilities: [[Option<usize>; 3]; 4],
}

impl Node {
    fn new(heat_loss_constant: usize) -> Self {
        Self {
            heat_loss_constant,
            possibilities: [[None; 3]; 4],
        }
    }

    fn get_min(&self) -> Option<usize> {
        self.possibilities.iter()
            .flat_map(|dir| dir.iter().filter(|x| x.is_some()).min())
            .min()
            .filter(|x| x.is_some())
            .map(|v| *v)
            .unwrap_or(None)
    }
}

#[derive(Debug)]
struct NodeInfo {
    coordinates: (i32, i32),
    distance: usize,
    direction: Option<(usize, Direction)>,
}

impl NodeInfo {
    fn new(coordinates: (i32, i32), distance: usize, direction: Option<(usize, Direction)>) -> Self {
        Self { coordinates, distance, direction }
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input.lines()
            .map(|line| line.bytes()
                .map(|b| Node::new((b - b'0') as usize))
                .collect())
            .collect();
        Self { grid }
    }

    fn print_lowest_heat_loss(&self) {
        for line in &self.grid {
            for node in line {
                print!(" ({},{}) ", node.heat_loss_constant, node.get_min().map(|val| val.to_string()).unwrap_or("x".to_string()));
            }
            println!();
        }
        println!();
    }

    fn get(&self, y: i32, x: i32) -> Option<&Node> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid.get(y as usize).and_then(|line| line.get(x as usize))
    }

    fn get_mut(&mut self, y: i32, x: i32) -> Option<&mut Node> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid.get_mut(y as usize).and_then(|line| line.get_mut(x as usize))
    }

    fn check_and_compute_node_info(&mut self, y: i32, x: i32, direction: Direction, prev_node_info: &NodeInfo) -> Option<NodeInfo> {
        if y < 0 || x < 0 {
            return None;
        }
        let node = self.grid.get_mut(y as usize).and_then(|line| line.get_mut(x as usize));
        if let Some(node) = node {
            if prev_node_info.direction.is_none() || prev_node_info.direction.unwrap().1 != direction || prev_node_info.direction.unwrap().0 < 3 {
                let direction_length = if prev_node_info.direction.is_none() { 1 } else { prev_node_info.direction.filter(|(_, d)| *d == direction).map(|(v, _)| v - 1).unwrap_or_default()};
                if node.possibilities[direction as usize][direction_length].is_none() || node.possibilities[direction as usize][direction_length].unwrap() > prev_node_info.distance + node.heat_loss_constant {
                    node.possibilities[direction as usize][direction_length] = Some(prev_node_info.distance + node.heat_loss_constant);
                    let direction_change = 1 + if prev_node_info.direction.is_none() { 1 } else { prev_node_info.direction.filter(|(_, d)| *d == direction).map(|(v, _)| v).unwrap_or_default()};
                    return Some(NodeInfo::new(
                        (y, x),
                        prev_node_info.distance + node.heat_loss_constant,
                        Some((direction_change, direction))));
                }
            }
        }
        None
    }

    fn shortest_path(&mut self, og_y: i32, og_x: i32) {
        let mut queue = VecDeque::new();
        let mut start = self.get_mut(og_y, og_x).unwrap();
        start.possibilities = [[Some(0); 3]; 4];
        queue.push_back(NodeInfo::new((og_y, og_x), 0, None));

        while let Some(node_info) = queue.pop_front() {
            let (y, x) = node_info.coordinates;

            if let Some(up_node_info) = self.check_and_compute_node_info(y - 1, x, Direction::Up, &node_info) {
                queue.push_back(up_node_info);
            }
            if let Some(down_node_info) = self.check_and_compute_node_info(y + 1, x, Direction::Down, &node_info) {
                queue.push_back(down_node_info);
            }
            if let Some(left_node_info) = self.check_and_compute_node_info(y, x - 1, Direction::Left, &node_info) {
                queue.push_back(left_node_info);
            }
            if let Some(right_node_info) = self.check_and_compute_node_info(y, x + 1, Direction::Right, &node_info) {
                queue.push_back(right_node_info);
            }
        }
    }
}

// 1241 too high