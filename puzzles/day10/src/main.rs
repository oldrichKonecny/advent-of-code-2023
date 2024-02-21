static SOUTH_CONNECTED_SHAPES: [char; 4] = ['|', '7', 'F', 'S'];
static NORTH_CONNECTED_SHAPES: [char; 4] = ['|', 'L', 'J', 'S'];
static EAST_CONNECTED_SHAPES: [char; 4] = ['-', 'F', 'L', 'S'];
static WEST_CONNECTED_SHAPES: [char; 4] = ['-', '7', 'J', 'S'];

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u32 {
    let mut graph = Graph::parse(input);
    let (y, x) = graph.find_start();
    graph.mark_node((y, x), 0);
    let mut possible_ways = graph.find_and_mark_possible_ways(y, x);
    while !possible_ways.is_empty() {
        let (y, x) = possible_ways.pop().unwrap();
        possible_ways.extend(graph.find_and_mark_possible_ways(y, x));
    }
    graph.print();
    graph.get_max_node()
}

fn second_part(input: &str) -> u32 {
    let mut graph = Graph::parse(input);
    let (y, x) = graph.find_start();
    graph.mark_node((y, x), 0);
    let mut possible_ways = graph.find_and_mark_possible_ways(y, x);
    while !possible_ways.is_empty() {
        let (y, x) = possible_ways.pop().unwrap();
        possible_ways.extend(graph.find_and_mark_possible_ways(y, x));
    }

    graph.number_of_inside_spaces()
}

struct Graph {
    map: Vec<Vec<Option<Node>>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    node_type: char,
    steps_from_start: Option<u32>,
}

impl Graph {
    fn parse(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().map(|c| Node::parse(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { map }
    }

    fn find_and_mark_possible_ways(&mut self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let current_node = self
            .map
            .get(y)
            .expect("Cannot get y")
            .get(x)
            .expect("Cannot get x")
            .as_ref()
            .expect("Cannot get node")
            .clone();
        let current_steps = current_node.steps_from_start.expect(&format!(
            "Cannot get steps from start for node {:?}",
            current_node
        ));

        let mut possible_ways = Vec::new();
        if y > 0 {
            if let Some(row) = self.map.get_mut(y - 1) {
                if let Some(Some(node)) = row.get_mut(x) {
                    if SOUTH_CONNECTED_SHAPES.contains(&node.node_type)
                        && NORTH_CONNECTED_SHAPES.contains(&current_node.node_type)
                        && (node.steps_from_start.is_none()
                            || node.steps_from_start.unwrap() > current_steps + 1)
                    {
                        possible_ways.push((y - 1, x));
                        node.steps_from_start = Some(current_node.steps_from_start.unwrap() + 1);
                    }
                }
            }
        }
        if x > 0 {
            if let Some(Some(node)) = self.map[y].get_mut(x - 1) {
                if EAST_CONNECTED_SHAPES.contains(&node.node_type)
                    && WEST_CONNECTED_SHAPES.contains(&current_node.node_type)
                    && (node.steps_from_start.is_none()
                        || node.steps_from_start.unwrap() > current_steps + 1)
                {
                    possible_ways.push((y, x - 1));
                    node.steps_from_start = Some(current_node.steps_from_start.unwrap() + 1);
                }
            }
        }
        if let Some(row) = self.map.get_mut(y + 1) {
            if let Some(Some(node)) = row.get_mut(x) {
                if NORTH_CONNECTED_SHAPES.contains(&node.node_type)
                    && SOUTH_CONNECTED_SHAPES.contains(&current_node.node_type)
                    && (node.steps_from_start.is_none()
                        || node.steps_from_start.unwrap() > current_steps + 1)
                {
                    possible_ways.push((y + 1, x));
                    node.steps_from_start = Some(current_node.steps_from_start.unwrap() + 1);
                }
            }
        }
        if let Some(Some(node)) = self.map[y].get_mut(x + 1) {
            if WEST_CONNECTED_SHAPES.contains(&node.node_type)
                && EAST_CONNECTED_SHAPES.contains(&current_node.node_type)
                && (node.steps_from_start.is_none()
                    || node.steps_from_start.unwrap() > current_steps + 1)
            {
                possible_ways.push((y, x + 1));
                node.steps_from_start = Some(current_node.steps_from_start.unwrap() + 1);
            }
        }

        possible_ways
    }

    fn find_start(&self) -> (usize, usize) {
        for (y, line) in self.map.iter().enumerate() {
            for (x, node) in line.iter().enumerate() {
                if let Some(node) = node {
                    if node.node_type == 'S' {
                        return (y, x);
                    }
                }
            }
        }
        panic!("No start found");
    }

    fn mark_node(&mut self, (y, x): (usize, usize), steps_from_start: u32) {
        if let Some(node) = self
            .map
            .get_mut(y)
            .expect("Cannot get y")
            .get_mut(x)
            .expect("Cannot get x")
        {
            if node.steps_from_start.is_none() || node.steps_from_start.unwrap() > steps_from_start
            {
                node.steps_from_start = Some(steps_from_start);
            }
        }
    }

    fn get_max_node(&self) -> u32 {
        self.map
            .iter()
            .filter_map(|line| {
                line.iter()
                    .filter_map(|node| node.as_ref())
                    .filter_map(|node| node.steps_from_start)
                    .max()
            })
            .max()
            .unwrap_or_default()
    }

    fn number_of_inside_spaces(&self) -> u32 {
        let mut spaces = 0;
        for (y, line) in self.map.iter().enumerate() {
            for (x, node) in line.iter().map(|n| n.as_ref()).enumerate() {
                if (node.is_none() || node.unwrap().steps_from_start.is_none())
                    && self.is_node_inside((y, x))
                {
                    spaces += 1;
                }
            }
        }
        spaces
    }

    fn is_node_inside(&self, (y, x): (usize, usize)) -> bool {
        let row = self.map.get(y).expect("Cannot get y");
        let mut num_of_crosses_left = 0;
        let mut is_line_down = false;
        let mut is_line_up = false;
        for i in 0..=x {
            let node = row.get(i).expect("Cannot get x");
            match node {
                None => {
                    is_line_up = false;
                    is_line_down = false;
                }
                Some(node) => match (node.steps_from_start.is_some(), node.node_type) {
                    (true, '|') => {
                        num_of_crosses_left += 1;
                    }
                    (true, 'F') => {
                        is_line_up = true;
                        num_of_crosses_left += 1;
                    }
                    (true, 'J') => {
                        if is_line_up {
                            is_line_up = false;
                        } else {
                            num_of_crosses_left += 1;
                            is_line_down = false;
                        }
                    }
                    (true, 'L') => {
                        is_line_down = true;
                        num_of_crosses_left += 1;
                    }
                    (true, '7') => {
                        if is_line_down {
                            is_line_down = false;
                        } else {
                            num_of_crosses_left += 1;
                            is_line_up = false;
                        }
                    }
                    (false, _) => {
                        is_line_up = false;
                        is_line_down = false;
                    }
                    _ => {}
                },
            }
        }

        num_of_crosses_left & 1 == 1
    }

    fn print(&self) {
        for line in &self.map {
            for node in line {
                if let Some(node) = node {
                    print!("{}", node.node_type);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        for line in &self.map {
            for node in line {
                if let Some(node) = node {
                    print!(
                        "{} ",
                        node.steps_from_start
                            .map(|s| s.to_string())
                            .unwrap_or(".".to_string())
                    );
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        for (y, line) in self.map.iter().enumerate() {
            for (x, node) in line.iter().enumerate() {
                if let Some(node) = node {
                    if node.steps_from_start.is_some() {
                        if node.node_type == 'S' {
                            print!("S");
                        } else {
                            print!("H");
                        }
                    } else {
                        if self.is_node_inside((y, x)) {
                            print!("X");
                        } else {
                            print!(".");
                        }
                    }
                } else if self.is_node_inside((y, x)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Node {
    fn parse(node_type: char) -> Option<Self> {
        if node_type == '.' {
            return None;
        }
        Some(Self {
            node_type,
            steps_from_start: None,
        })
    }
}
