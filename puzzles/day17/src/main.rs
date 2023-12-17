use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input_test.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.print_path();
    let mut v = grid.get_mut(2, 2).unwrap();
    v.1 = Some(4);
    grid.print_path();
    0
}

fn second_part(input: &str) -> usize {
    0
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<(u8, Option<usize>)>>
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input.lines()
            .map(|line| line.bytes()
                .map(|b| (b - b'0', None))
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
            for (_, path) in line {
                print!("{}", path.map(|v| v.to_string()).unwrap_or(".".to_string()));
            }
            println!();
        }
        println!();
    }

    fn get(&self, y: i32, x: i32) -> Option<&(u8, Option<usize>)> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid.get(y as usize).and_then(|line| line.get(x as usize))
    }

    fn get_mut(&mut self, y: i32, x: i32) -> Option<&mut (u8, Option<usize>)> {
        if y < 0 || x < 0 {
            return None;
        }
        self.grid.get_mut(y as usize).and_then(|line| line.get_mut(x as usize))
    }

    fn shortest_path(&mut self, y: i32, x: i32) {
        let mut queue = VecDeque::new();
        queue.push_back(((y, x, 0, )));
        while let Some(((y, x), steps)) = queue.pop_front() {
            if let Some((_, Some(v))) = self.get_mut(y as i32, x as i32) {
                if *v > steps {
                    *v = steps;
                } else {
                    continue;
                }
            }
            if let Some((b, _)) = self.get(y as i32, x as i32) {
                if *b == 0 {
                    continue;
                }
            }
            queue.push_back(((y + 1, x), steps + 1));
            queue.push_back(((y - 1, x), steps + 1));
            queue.push_back(((y, x + 1), steps + 1));
            queue.push_back(((y, x - 1), steps + 1));
        }
    }
}

struct NodeInfo {

}
