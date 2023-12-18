use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let input = include_str!("../input_test.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.shortest_path(0, 0);
    grid.print_path();
    grid.get(grid.grid.len() as i32 - 1, grid.grid[0].len() as i32 -1).unwrap().1.unwrap()
}

fn second_part(input: &str) -> usize {
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
    grid: Vec<Vec<(usize, Option<usize>)>>
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input.lines()
            .map(|line| line.bytes()
                .map(|b| ((b - b'0') as usize, None))
                .collect())
            .collect();
        Self { grid }
    }

    fn print(&self) {
        for line in &self.grid {
            for (b, _) in line {
                print!("{}", b);
            }
            println!();
        }
        println!();
    }

    fn print_path(&self) {
        for line in &self.grid {
            for (x, path) in line {
                print!(" {} ", path.map(|v| format!("({},{:3})", *x, v)).unwrap_or(".".to_string()));
            }
            println!();
        }
        println!();
    }

    fn get(&self, y: i32, x: i32) -> Option<&(usize, Option<usize>)> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid.get(y as usize).and_then(|line| line.get(x as usize))
    }

    fn get_mut(&mut self, y: i32, x: i32) -> Option<&mut (usize, Option<usize>)> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid.get_mut(y as usize).and_then(|line| line.get_mut(x as usize))
    }

    fn shortest_path(&mut self, og_y: i32, og_x: i32) {
        let mut queue = BinaryHeap::new();
        let mut start = self.get_mut(og_y, og_x).unwrap();
        start.1 = Some(0);
        queue.push(NodeInfo::new((og_y, og_x), 0, None));
        while let Some(node_info) = queue.pop() {
            println!("Node info: {:?}", node_info);
            let (y, x) = node_info.coordinates;
            if let Some(up_node) = self.get_mut(y - 1, x) {
                if (node_info.direction.is_none() || node_info.direction.unwrap().1 != Direction::Up || node_info.direction.unwrap().0 < 3)
                    && (up_node.1.is_none() || up_node.1.unwrap() >= node_info.distance + up_node.0) {
                    println!("    Looking at up node:({}, {}) {:?} --> {}", y - 1, x, up_node, node_info.distance + up_node.0);
                    up_node.1 = Some(node_info.distance + up_node.0);
                    let direction_change = 1 + if node_info.direction.is_none() { 1 } else { node_info.direction.filter(|(_, d)| *d == Direction::Up).map(|(v, _)| v).unwrap_or_default()};
                    queue.push(NodeInfo::new(
                            (y - 1, x),
                            node_info.distance + up_node.0,
                            Some((direction_change, Direction::Up))));
                }
            }
            if let Some(down_node) = self.get_mut(y + 1, x) {
                if (node_info.direction.is_none() || node_info.direction.unwrap().1 != Direction::Down || node_info.direction.unwrap().0 < 3)
                    && (down_node.1.is_none() || down_node.1.unwrap() >= node_info.distance + down_node.0) {
                    println!("    Looking at down node:({}, {}) {:?} --> {}", y + 1, x, down_node, node_info.distance + down_node.0);
                    down_node.1 = Some(node_info.distance + down_node.0);
                    let direction_change = 1 + if node_info.direction.is_none() { 1 } else { node_info.direction.filter(|(_, d)| *d == Direction::Down).map(|(v, _)| v).unwrap_or_default()};
                    queue.push(NodeInfo::new(
                        (y + 1, x),
                        node_info.distance + down_node.0,
                        Some((direction_change, Direction::Down))));
                }
            }
            if let Some(left_node) = self.get_mut(y, x - 1) {
                if (node_info.direction.is_none() || node_info.direction.unwrap().1 != Direction::Left || node_info.direction.unwrap().0 < 3)
                    && (left_node.1.is_none() || left_node.1.unwrap() >= node_info.distance + left_node.0) {
                    println!("    Looking at left node:({}, {}) {:?} --> {}", y, x - 1, left_node, node_info.distance + left_node.0);
                    left_node.1 = Some(node_info.distance + left_node.0);
                    let direction_change = 1 + if node_info.direction.is_none() { 1 } else { node_info.direction.filter(|(_, d)| *d == Direction::Left).map(|(v, _)| v).unwrap_or_default()};
                    queue.push(NodeInfo::new(
                        (y, x - 1),
                        node_info.distance + left_node.0,
                        Some((direction_change, Direction::Left))));
                }
            }
            if let Some(right_node) = self.get_mut(y, x + 1) {
                if (node_info.direction.is_none() || node_info.direction.unwrap().1 != Direction::Right || node_info.direction.unwrap().0 < 3)
                    && (right_node.1.is_none() || right_node.1.unwrap() >= node_info.distance + right_node.0) {
                    println!("    Looking at right node:({}, {}) {:?} --> {}", y, x+1, right_node, node_info.distance + right_node.0);
                    right_node.1 = Some(node_info.distance + right_node.0);
                    let direction_change = 1 + if node_info.direction.is_none() { 1 } else { node_info.direction.filter(|(_, d)| *d == Direction::Right).map(|(v, _)| v).unwrap_or_default()};
                    queue.push(NodeInfo::new(
                        (y, x + 1),
                        node_info.distance + right_node.0,
                        Some((direction_change, Direction::Right))));
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}