fn main() {
    let input = include_str!("../input_test.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let dig_instructions = input.lines()
        .map(Instruction::parse)
        .collect::<Vec<_>>();
    let basin = Basin::new(&dig_instructions);
    basin.print();
    basin.compute_content()
}

fn second_part(input: &str) -> usize {
    0
}

#[derive(Debug)]
struct Basin {
    tiles: Vec<Vec<char>>,
}

impl Basin {
    fn compute_content(&self) -> usize {
        let mut content = 0;
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                let tile = self.tiles[y][x];
                if tile == 'x' || tile == '#' {
                    content += 1;
                }
            }
        }
        content
    }

    fn new(dig_instructions: &[Instruction]) -> Self {
        let mut max_height = 0i32;
        let mut min_height = 0i32;
        let mut max_width = 0i32;
        let mut min_width = 0i32;
        let mut height = 0i32;
        let mut width = 0i32;
        for instruction in dig_instructions {
           match instruction.direction {
               Direction::U => {
                   height -= instruction.distance as i32;
                     if height < min_height {
                          min_height = height;
                     }
               },
               Direction::D => {
                   height += instruction.distance as i32;
                   if height > max_height {
                       max_height = height;
                   }
               },
               Direction::L => {
                   width -= instruction.distance as i32;
                        if width < min_width {
                            min_width = width;
                        }
               },
               Direction::R => {
                   width += instruction.distance as i32;
                     if width > max_width {
                          max_width = width;
                     }
               },
           }
        }

        let vec_width = (max_width - min_width) as usize + 1;
        let vec_height = (max_height - min_height) as usize + 1;
        let mut tiles = vec![vec!['.'; vec_width]; vec_height];
        let mut y = (0 - min_height) as usize;
        let mut x = (0 - min_width) as usize;
        for instruction in dig_instructions {
            match instruction.direction {
                Direction::U => {
                    for _ in 0..instruction.distance {
                        y -= 1;
                        tiles[y][x] = '#';
                    }
                }
                Direction::D => {
                    for _ in 0..instruction.distance {
                        y += 1;
                        tiles[y][x] = '#';
                    }
                }
                Direction::L => {
                    for _ in 0..instruction.distance {
                        x -= 1;
                        tiles[y][x] = '#';
                    }
                }
                Direction::R => {
                    for _ in 0..instruction.distance {
                        x += 1;
                        tiles[y][x] = '#';
                    }
                }
            }
        }

        Self { tiles }
    }

    fn print(&self) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                print!("{}", self.tiles[y][x]);
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
    color: String,
}

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let direction = match split.next().unwrap() {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!("Invalid direction"),
        };
        let distance = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap();
        let color = color[2..color.len() - 1].to_string();
        Self {
            direction,
            distance,
            color,
        }
    }
}
