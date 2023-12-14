fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut boulder_map = BoulderMap::parse(input);
    boulder_map.tilt_north();
    boulder_map.compute_rock_weights()
}

fn second_part(input: &str) -> usize {
    let mut boulder_map = BoulderMap::parse(input);
    for _ in 0..1000 {
        boulder_map.tilt_north();
        boulder_map.tilt_west();
        boulder_map.tilt_south();
        boulder_map.tilt_east();
    }
    boulder_map.compute_rock_weights()
}

struct BoulderMap {
    map: Vec<Vec<char>>,
}

impl BoulderMap {
    fn parse(input: &str) -> Self {
        let map = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        Self {
            map,
        }
    }

    fn print(&self) {
        for line in &self.map {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn compute_rock_weights(&self) -> usize {
        let boulder_length = self.map.len();
        let mut weights = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 'O' {
                    weights += boulder_length - y;
                }
            }
        }
        weights
    }

    fn tilt_north(&mut self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 'O' {
                    let mut new_y = y;
                    while new_y > 0 && self.map[new_y - 1][x] == '.' {
                        new_y -= 1;
                    }
                    if new_y != y {
                        self.map[new_y][x] = 'O';
                        self.map[y][x] = '.';
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.map.len()).rev() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 'O' {
                    let mut new_y = y;
                    while let Some(row) = self.map.get(new_y + 1) {
                        if row[x] == '.' {
                            new_y += 1;
                        } else {
                            break;
                        }
                    }
                    if new_y != y {
                        self.map[new_y][x] = 'O';
                        self.map[y][x] = '.';
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.map.len() {
            for x in (0..self.map[y].len()).rev() {
                if self.map[y][x] == 'O' {
                    let mut new_x = x;
                    while let Some(c) = self.map[y].get(new_x + 1) {
                        if *c == '.' {
                            new_x += 1;
                        } else {
                            break;
                        }
                    }
                    if new_x != x {
                        self.map[y][new_x] = 'O';
                        self.map[y][x] = '.';
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 'O' {
                    let mut new_x = x;
                    while new_x > 0 && self.map[y][new_x - 1] == '.' {
                        new_x -= 1;
                    }
                    if new_x != x {
                        self.map[y][new_x] = 'O';
                        self.map[y][x] = '.';
                    }
                }
            }
        }
    }
}


