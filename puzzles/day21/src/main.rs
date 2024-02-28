const EMPTY: u8 = b'.';

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut garden_map = GardenMap::parse(input);
    garden_map.make_steps_and_count(64)
}

fn second_part(input: &str) -> usize {
    let mut garden_map = GardenMap::parse(input);
    let full_odd = garden_map.make_steps_and_count(131);

    let mut garden_map = GardenMap::parse(input);
    let full_even = garden_map.make_steps_and_count(132);

    let mut garden_map = GardenMap::parse(input);
    let middle_odd = garden_map.make_steps_and_count(65);
    let corners_odd = full_odd - middle_odd;

    let mut garden_map = GardenMap::parse(input);
    let middle_even = garden_map.make_steps_and_count(64);
    let corners_even = full_even - middle_even;

    let n = 202_300usize;

    (n + 1).pow(2) * full_odd + n.pow(2) * full_even - (n + 1) * corners_odd + n * corners_even
}

#[derive(Debug)]
struct GardenMap {
    map: Vec<Vec<u8>>,
}

impl GardenMap {
    fn parse(input: &str) -> Self {
        let map = input.lines().map(|line| line.bytes().collect()).collect();
        Self { map }
    }

    fn make_steps_and_count(&mut self, steps: usize) -> usize {
        let start = self.find_start();
        let mut queue = vec![start];
        for step in 1..=steps {
            let mut new_queue = Vec::new();
            for current in queue {
                let to_queue = self.do_single_step_return_marked(current, to_byte(step % 2));
                new_queue.extend(to_queue);
            }
            queue = new_queue;
        }

        self.count_marked(to_byte(steps % 2))
    }

    fn do_single_step_return_marked(
        &mut self,
        current: (usize, usize),
        step: u8,
    ) -> Vec<(usize, usize)> {
        let target = self.map[current.0][current.1];
        if target != b'S' && target != b'0' && target != b'1' {
            return Vec::new();
        }
        let mut marked = Vec::new();
        if current.0 > 0 && self.map[current.0 - 1][current.1] == EMPTY {
            self.map[current.0 - 1][current.1] = step;
            marked.push((current.0 - 1, current.1));
        }
        if current.0 < self.map.len() - 1 && self.map[current.0 + 1][current.1] == EMPTY {
            self.map[current.0 + 1][current.1] = step;
            marked.push((current.0 + 1, current.1));
        }
        if current.1 > 0 && self.map[current.0][current.1 - 1] == EMPTY {
            self.map[current.0][current.1 - 1] = step;
            marked.push((current.0, current.1 - 1));
        }
        if current.1 < self.map[0].len() - 1 && self.map[current.0][current.1 + 1] == EMPTY {
            self.map[current.0][current.1 + 1] = step;
            marked.push((current.0, current.1 + 1));
        }

        marked
    }

    fn find_start(&self) -> (usize, usize) {
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == b'S' {
                    return (i, j);
                }
            }
        }
        panic!("No start found");
    }

    fn count_marked(&self, step: u8) -> usize {
        self.map.iter().flatten().filter(|&&x| x == step).count() + if step == b'0' { 1 } else { 0 }
    }

    fn print(&self) {
        for line in &self.map {
            for byte in line {
                let byte = match *byte {
                    b'0' => 'x',
                    b'1' => '.',
                    _ => *byte as char,
                };
                print!("{}", byte);
            }
            println!();
        }
    }
}

fn to_byte(val: usize) -> u8 {
    match val {
        0 => b'0',
        1 => b'1',
        _ => panic!("Unknown value: {}", val),
    }
}

// 618629741610649 - too low
// 621289922886149 - correct
