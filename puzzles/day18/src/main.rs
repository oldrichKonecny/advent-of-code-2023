fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> i64 {
    let dig_instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let basin = Basin::new(&dig_instructions);
    basin.compute_shoelace()
}

fn second_part(input: &str) -> i64 {
    let dig_instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let basin = Basin::new_with_color(&dig_instructions);
    basin.compute_shoelace()
}

#[derive(Debug)]
struct Basin {
    bounds: Vec<(i64, i64)>,
}

impl Basin {
    fn new(dig_instructions: &[Instruction]) -> Self {
        let mut bounds = vec![(0, 0)];
        let mut last_point = (0, 0);

        for instruction in dig_instructions {
            match instruction.direction {
                Direction::U => last_point = (last_point.0, last_point.1 - instruction.distance),
                Direction::D => last_point = (last_point.0, last_point.1 + instruction.distance),
                Direction::L => last_point = (last_point.0 - instruction.distance, last_point.1),
                Direction::R => last_point = (last_point.0 + instruction.distance, last_point.1),
            }
            bounds.push(last_point);
        }
        Self { bounds }
    }

    fn new_with_color(dig_instructions: &[Instruction]) -> Self {
        fn decode_color(color: &str) -> (Direction, i64) {
            let distance = i64::from_str_radix(&color[..color.len() - 1], 16).unwrap();
            let direction = &color[color.len() - 1..];
            let direction = match direction {
                "3" => Direction::U,
                "1" => Direction::D,
                "2" => Direction::L,
                "0" => Direction::R,
                _ => panic!("Invalid direction"),
            };
            (direction, distance)
        }

        let mut bounds = vec![(0, 0)];
        let mut last_point = (0, 0);

        dig_instructions
            .iter()
            .map(|instruction| decode_color(&instruction.color))
            .for_each(|(direction, distance)| {
                match direction {
                    Direction::U => last_point = (last_point.0, last_point.1 - distance),
                    Direction::D => last_point = (last_point.0, last_point.1 + distance),
                    Direction::L => last_point = (last_point.0 - distance, last_point.1),
                    Direction::R => last_point = (last_point.0 + distance, last_point.1),
                }
                bounds.push(last_point);
            });

        Self { bounds }
    }

    fn compute_shoelace(&self) -> i64 {
        let mut shoelace = 0;
        let mut perimeter = 0;
        for i in 0..self.bounds.len() - 1 {
            let a = self.bounds[i];
            let b = self.bounds[i + 1];
            shoelace += a.0 * b.1 - b.0 * a.1;
            perimeter += if a.0 == b.0 {
                (a.1 - b.1).abs()
            } else {
                (a.0 - b.0).abs()
            };
        }
        (shoelace.abs() / 2) + (perimeter / 2) + 1
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
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
