use std::fmt::format;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    input.lines()
        .map(Springs::parse)
        .map(|spring| spring.determine_number_of_combinations())
        .sum()
}

fn second_part(input: &str) -> usize {
    0
}

#[derive(Debug)]
struct Springs {
    row: Vec<Group>,
    damaged_groups: Vec<usize>,
}

impl Springs {
    fn parse(line: &str) -> Self {
        let (row, damaged_groups) = line.split_once(" ").expect(&format!("Cannot split line on space: {}", line));
        let row = row.split(".")
            .filter(|split| !split.is_empty())
            .map(|split| Group::new(split.chars().collect()))
            .collect();
        let damaged_groups = damaged_groups.split(",")
            .map(|group| group.parse::<usize>().expect(&format!("Cannot parse group: {}", group)))
            .collect();
        Self {
            row,
            damaged_groups,
        }
    }

    fn determine_number_of_combinations(&self) -> usize {
        println!("Spring: {:?}", self);
        0

    }

    fn remove_predetermined(&mut self) {
        let mut new_row = Vec::new();
        for damaged in self.damaged_groups.iter() {
            if !self.damaged_groups.contains(&group.group.len()) {
                new_row.push(group);
            }
        }
        self.row = new_row;
    }
}

#[derive(Debug)]
struct Group {
    group: Vec<char>,
}

impl Group {
    fn new(group: Vec<char>) -> Self {
        Self {
            group,
        }
    }

    fn fix_exactly(&self, number_of_damaged: usize) -> bool {
        self.group.len() == number_of_damaged
    }
}

fn rec_combinations(row: &[char], damaged_groups: &[usize], current_combinations: usize) -> usize {
    if row.len() == 0 {
        return 0;
    }

    match row[0] {
        '.' => rec_combinations(&row[1..], damaged_groups, current_combinations),
        '#' => {
            0
        },
        '?'=> {
            0
        },
        _ => panic!("Unknown first char: {} on row {:?}", row[0], row),
    }
}