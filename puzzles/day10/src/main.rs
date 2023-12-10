fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    0
}

fn second_part(input: &str) -> u64 {
    0
}

struct Graph {
    map: Vec<Vec<Option<Node>>>,
}

struct Node {
    node_type: char,
    steps_from_start: Option<u32>,
}

