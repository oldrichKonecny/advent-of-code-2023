use std::cell::RefCell;
use std::rc::Rc;
use ahash::AHashMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    input.lines()
        .map(Springs::parse)
        .map(|spring| rec_combinations(&spring.row, &spring.damaged_groups))
        .sum()
}

fn second_part(input: &str) -> usize {
    let mut cache = Rc::new(RefCell::new(AHashMap::new()));
    input.lines()
        .map(|line| Springs::parse_part2(line, 5))
        .map(|spring| rec_combinations_with_cache(&spring.row, &spring.damaged_groups, cache.clone()))
        .sum()
}

#[derive(Debug)]
struct Springs {
    row: Vec<char>,
    damaged_groups: Vec<usize>,
}

impl Springs {
    fn parse(line: &str) -> Self {
        let (row, damaged_groups) = line.split_once(" ").expect(&format!("Cannot split line on space: {}", line));
        let row = row.chars().collect();
        let damaged_groups = damaged_groups.split(",")
            .map(|group| group.parse::<usize>().expect(&format!("Cannot parse group: {}", group)))
            .collect();
        Self {
            row,
            damaged_groups,
        }
    }

    fn parse_part2(line: &str, multiple: usize) -> Self {
        let (row, damaged_groups) = line.split_once(" ").expect(&format!("Cannot split line on space: {}", line));
        let row = row.chars().collect::<Vec<_>>();
        let damaged_groups = damaged_groups.split(",")
            .map(|group| group.parse::<usize>().expect(&format!("Cannot parse group: {}", group)))
            .collect::<Vec<_>>();
        let mut full_row = Vec::with_capacity(row.len() * multiple);
        for i in 0..multiple {
            full_row.extend_from_slice(&row);
            if i < multiple - 1 {
                full_row.push('?');
            }
        }
        let damaged_groups = (0..multiple)
            .flat_map(|_| damaged_groups.iter().copied())
            .collect();
        Self {
            row: full_row,
            damaged_groups,
        }
    }
}

fn rec_combinations_with_cache(row: &[char], damaged_groups: &[usize], cache: Rc<RefCell<AHashMap<(Vec<char>, Vec<usize>), usize>>>) -> usize {
    if let Some(res) = cache.borrow().get(&(row.to_vec(), damaged_groups.to_vec())) {
        return *res;
    }

    if damaged_groups.is_empty() {
        return if !row.contains(&'#') { 1 } else { 0 }
    }
    if row.is_empty() {
        return 0;
    }

    let handle_pound = |damaged: usize| {
        if damaged > row.len() {
            return 0;
        }

        for (i, &ch) in row.iter().enumerate() {
            if i < damaged && (ch == '#' || ch == '?') {
                continue;
            } else if i == damaged && (ch == '.' || ch == '?') {
                return rec_combinations_with_cache(&row[i+1..], &damaged_groups[1..], cache.clone())
            } else {
                return 0;
            }
        }

        if damaged_groups.len() == 1 { 1 } else { 0 }
    };

    match (row[0], damaged_groups[0]) {
        ('.', _) => {
            let res = rec_combinations_with_cache(&row[1..], &damaged_groups, cache.clone());
            cache.borrow_mut().insert((row.to_vec(), damaged_groups.to_vec()), res);
            res
        },
        ('#', damaged) => {
            let res = handle_pound(damaged);
            cache.borrow_mut().insert((row.to_vec(), damaged_groups.to_vec()), res);
            res
        },
        ('?', damaged) => {
            let res = rec_combinations_with_cache(&row[1..], &damaged_groups, cache.clone()) + handle_pound(damaged);
            cache.borrow_mut().insert((row.to_vec(), damaged_groups.to_vec()), res);
            res
        },
        _ => panic!("Unknown character: {} and group: {} in row: {}", row[0], damaged_groups[0], row.iter().collect::<String>())
    }
}

fn rec_combinations(row: &[char], damaged_groups: &[usize]) -> usize {
    if damaged_groups.is_empty() {
        return if !row.contains(&'#') { 1 } else { 0 }
    }
    if row.is_empty() {
        return 0;
    }

    let handle_pound = |damaged: usize| {
        if damaged > row.len() {
            return 0;
        }

        for (i, &ch) in row.iter().enumerate() {
            if i < damaged && (ch == '#' || ch == '?') {
                continue;
            } else if i == damaged && (ch == '.' || ch == '?') {
                return rec_combinations(&row[i+1..], &damaged_groups[1..])
            } else {
                return 0;
            }
        }

        if damaged_groups.len() == 1 { 1 } else { 0 }
    };

    match (row[0], damaged_groups[0]) {
        ('.', _) => {
            rec_combinations(&row[1..], &damaged_groups)
        },
        ('#', damaged) => {
            handle_pound(damaged)
        },
        ('?', damaged) => {
            rec_combinations(&row[1..], &damaged_groups) + handle_pound(damaged)
        },
        _ => panic!("Unknown character: {} and group: {} in row: {}", row[0], damaged_groups[0], row.iter().collect::<String>())
    }
}