use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    let (instructions, graph) = input.split_once("\n\n").unwrap();
    let mut instruction_iterator = instructions.parse::<InstructionIterator>().unwrap();

    let graph = graph.lines()
        .map(|line| {
            let (node, edges) = line.split_once(" = ").unwrap();
            let (left, right) = edges[1..edges.len()-1].split_once(", ").unwrap();
            (node, (left, right))
        })
       .collect::<HashMap<_, _>>();

    let mut node = graph.get("AAA").unwrap();
    let mut current_node_name = "AAA";
    loop {
        if current_node_name == "ZZZ" {
            break instruction_iterator.number_of_steps;
        }
        match  instruction_iterator.next().unwrap() {
            Instruction::Left => {
                current_node_name = node.0;
                node = graph.get(node.0).unwrap();
            }
            Instruction::Right => {
                current_node_name = node.1;
                node = graph.get(node.1).unwrap();
            }
        }
    }
}

fn second_part(input: &str) -> u64 {
    let (instructions, graph) = input.split_once("\n\n").unwrap();
    let mut instruction_iterator = instructions.parse::<InstructionIterator>().unwrap();

    let graph = graph.lines()
        .map(|line| {
            let (node, edges) = line.split_once(" = ").unwrap();
            let (left, right) = edges[1..edges.len()-1].split_once(", ").unwrap();
            (node, (left, right))
        })
        .collect::<HashMap<_, _>>();
    let mut all_nodes = graph.keys()
        .filter(|node| node.ends_with("A"))
        .collect::<Vec<_>>();

    loop {
        if all_nodes.iter().all(|node| node.ends_with("Z")) {
            break instruction_iterator.number_of_steps;
        }
        let inst = instruction_iterator.next().unwrap();
        match  inst {
            Instruction::Left => {
                for node in all_nodes.iter_mut() {
                    let (left, _) = graph.get(*node).unwrap();
                    *node = left;
                }
            }
            Instruction::Right => {
                for node in all_nodes.iter_mut() {
                    let (_, right) = graph.get(*node).unwrap();
                    *node = right;
                }
            }
        }
    }
}


#[derive(Debug)]
struct InstructionIterator {
    instructions: Vec<Instruction>,
    index: usize,
    number_of_steps: u64,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Iterator for InstructionIterator {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions[self.index];
        if self.index == self.instructions.len() -1 {
            self.index = 0
        } else {
            self.index += 1;
        }
        self.number_of_steps += 1;
        Some(instruction)
    }
}

impl FromStr for InstructionIterator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.chars()
            .map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction"),
            })
            .collect::<Vec<_>>();

        Ok(InstructionIterator {
            instructions,
            index: 0,
            number_of_steps: 0,
        })
    }
}