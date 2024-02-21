fn main() {
    let input = include_str!("../input_test.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    0
}

fn second_part(input: &str) -> u64 {
    0
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


}
