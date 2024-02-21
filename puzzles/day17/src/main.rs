use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.shortest_path(0, 0);
    let x = grid.get(grid.grid.len() as i32 - 1, grid.grid[0].len() as i32 - 1);
    x.unwrap().get_min().unwrap()
}

fn second_part(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.shortest_path_ultra(0, 0);
    let x = grid.get(grid.grid.len() as i32 - 1, grid.grid[0].len() as i32 - 1);
    x.unwrap().get_min_ultra().unwrap()
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
    grid: Vec<Vec<Node>>,
}

#[derive(Debug)]
struct Node {
    heat_loss_constant: usize,
    possibilities: [[Option<usize>; 10]; 4],
}

impl Node {
    fn new(heat_loss_constant: usize) -> Self {
        Self {
            heat_loss_constant,
            possibilities: [[None; 10]; 4],
        }
    }

    fn get_min(&self) -> Option<usize> {
        self.possibilities
            .iter()
            .flat_map(|dir| dir.iter().filter(|x| x.is_some()).min())
            .min()
            .filter(|x| x.is_some())
            .map(|v| *v)
            .unwrap_or(None)
    }

    fn get_min_ultra(&self) -> Option<usize> {
        self.possibilities
            .iter()
            .flat_map(|dir| dir.iter().skip(3).filter(|x| x.is_some()).min())
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
    fn new(
        coordinates: (i32, i32),
        distance: usize,
        direction: Option<(usize, Direction)>,
    ) -> Self {
        Self {
            coordinates,
            distance,
            direction,
        }
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(|b| Node::new((b - b'0') as usize))
                    .collect()
            })
            .collect();
        Self { grid }
    }

    fn _print_lowest_heat_loss(&self) {
        for line in &self.grid {
            for node in line {
                print!(
                    " ({},{:3}) ",
                    node.heat_loss_constant,
                    node.get_min()
                        .map(|val| val.to_string())
                        .unwrap_or("x".to_string())
                );
            }
            println!();
        }
        println!();
    }

    fn get(&self, y: i32, x: i32) -> Option<&Node> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid
            .get(y as usize)
            .and_then(|line| line.get(x as usize))
    }

    fn get_mut(&mut self, y: i32, x: i32) -> Option<&mut Node> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid
            .get_mut(y as usize)
            .and_then(|line| line.get_mut(x as usize))
    }

    fn check_and_compute_node_info(
        &mut self,
        y: i32,
        x: i32,
        direction: Direction,
        prev_node_info: &NodeInfo,
    ) -> Option<NodeInfo> {
        if y < 0 || x < 0 {
            return None;
        }
        if let Some((_, d)) = prev_node_info.direction {
            match (direction, d) {
                (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left) => return None,
                _ => {}
            }
        }

        let node = self
            .grid
            .get_mut(y as usize)
            .and_then(|line| line.get_mut(x as usize));
        if let Some(node) = node {
            if prev_node_info.direction.is_none()
                || prev_node_info.direction.unwrap().1 != direction
                || prev_node_info.direction.unwrap().0 < 3
            {
                let direction_length = if prev_node_info.direction.is_none() {
                    1
                } else {
                    prev_node_info
                        .direction
                        .filter(|(_, d)| *d == direction)
                        .map(|(v, _)| v)
                        .unwrap_or_default()
                };
                if node.possibilities[direction as usize][direction_length].is_none()
                    || node.possibilities[direction as usize][direction_length].unwrap()
                        > prev_node_info.distance + node.heat_loss_constant
                {
                    node.possibilities[direction as usize][direction_length] =
                        Some(prev_node_info.distance + node.heat_loss_constant);
                    return Some(NodeInfo::new(
                        (y, x),
                        prev_node_info.distance + node.heat_loss_constant,
                        Some((direction_length + 1, direction)),
                    ));
                }
            }
        }
        None
    }

    fn check_and_compute_node_info_ultra(
        &mut self,
        y: i32,
        x: i32,
        direction: Direction,
        prev_node_info: &NodeInfo,
    ) -> Option<NodeInfo> {
        if y < 0 || x < 0 {
            return None;
        }
        if let Some((_, d)) = prev_node_info.direction {
            match (direction, d) {
                (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left) => return None,
                _ => {}
            }
        }

        let node = self
            .grid
            .get_mut(y as usize)
            .and_then(|line| line.get_mut(x as usize));
        if let Some(node) = node {
            if prev_node_info.direction.is_none()
                || (prev_node_info.direction.unwrap().1 == direction
                    && prev_node_info.direction.unwrap().0 < 10)
                || (prev_node_info.direction.unwrap().1 != direction
                    && prev_node_info.direction.unwrap().0 >= 4
                    && prev_node_info.direction.unwrap().0 <= 10)
            {
                let direction_length = if prev_node_info.direction.is_none() {
                    0
                } else {
                    prev_node_info
                        .direction
                        .filter(|(_, d)| *d == direction)
                        .map(|(v, _)| v)
                        .unwrap_or_default()
                };
                if node.possibilities[direction as usize][direction_length].is_none()
                    || node.possibilities[direction as usize][direction_length].unwrap()
                        > prev_node_info.distance + node.heat_loss_constant
                {
                    // println!("Considering node: ({}, {}) - {} [prev ({}, {}) {:?}]", y, x, node.heat_loss_constant, prev_node_info.coordinates.0, prev_node_info.coordinates.1, prev_node_info.direction);
                    node.possibilities[direction as usize][direction_length] =
                        Some(prev_node_info.distance + node.heat_loss_constant);
                    return Some(NodeInfo::new(
                        (y, x),
                        prev_node_info.distance + node.heat_loss_constant,
                        Some((direction_length + 1, direction)),
                    ));
                }
            }
        }
        None
    }

    fn shortest_path(&mut self, og_y: i32, og_x: i32) {
        let mut queue = VecDeque::new();
        let start = self.get_mut(og_y, og_x).unwrap();
        start.possibilities = [[Some(0); 10]; 4];
        queue.push_back(NodeInfo::new((og_y, og_x), 0, None));

        while let Some(node_info) = queue.pop_front() {
            let (y, x) = node_info.coordinates;

            if let Some(up_node_info) =
                self.check_and_compute_node_info(y - 1, x, Direction::Up, &node_info)
            {
                queue.push_back(up_node_info);
            }
            if let Some(down_node_info) =
                self.check_and_compute_node_info(y + 1, x, Direction::Down, &node_info)
            {
                queue.push_back(down_node_info);
            }
            if let Some(left_node_info) =
                self.check_and_compute_node_info(y, x - 1, Direction::Left, &node_info)
            {
                queue.push_back(left_node_info);
            }
            if let Some(right_node_info) =
                self.check_and_compute_node_info(y, x + 1, Direction::Right, &node_info)
            {
                queue.push_back(right_node_info);
            }
        }
    }

    fn shortest_path_ultra(&mut self, og_y: i32, og_x: i32) {
        let mut queue = VecDeque::new();
        let start = self.get_mut(og_y, og_x).unwrap();
        start.possibilities = [[Some(0); 10]; 4];
        queue.push_back(NodeInfo::new((og_y, og_x), 0, None));

        while let Some(node_info) = queue.pop_front() {
            let (y, x) = node_info.coordinates;

            if let Some(up_node_info) =
                self.check_and_compute_node_info_ultra(y - 1, x, Direction::Up, &node_info)
            {
                queue.push_back(up_node_info);
            }
            if let Some(down_node_info) =
                self.check_and_compute_node_info_ultra(y + 1, x, Direction::Down, &node_info)
            {
                queue.push_back(down_node_info);
            }
            if let Some(left_node_info) =
                self.check_and_compute_node_info_ultra(y, x - 1, Direction::Left, &node_info)
            {
                queue.push_back(left_node_info);
            }
            if let Some(right_node_info) =
                self.check_and_compute_node_info_ultra(y, x + 1, Direction::Right, &node_info)
            {
                queue.push_back(right_node_info);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part_0() {
        let input = include_str!("../input.txt");
        assert_eq!(first_part(input), 1195);
    }

    #[test]
    fn test_second_part_0() {
        let input = include_str!("../input.txt");
        assert_eq!(second_part(input), 1347);
    }

    #[test]
    fn test_second_part_1() {
        let input = "
            111159999999999
            999159999999999
            999159999999999
            999159999999999
            999111111111111";
        assert_eq!(second_part(input), 34);
    }

    #[test]
    fn test_second_part_2() {
        let input = "
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991";
        assert_eq!(second_part(input), 71);
    }

    #[test]
    fn test_second_part_3() {
        let input = "
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533";
        assert_eq!(second_part(input), 94);
    }

    #[test]
    fn test_second_part_4() {
        let input = "
            11111111119999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999919999999999
            99999999911111111111";
        assert_eq!(second_part(input), 29);
    }

    #[test]
    fn test_second_part_5() {
        let input = "
            011111111119999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999919999999999
            999999999911111111111";
        assert_eq!(second_part(input), 30);
    }
}
