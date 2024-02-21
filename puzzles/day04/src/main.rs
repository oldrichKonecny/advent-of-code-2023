use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let matches = compute_card_matches(line);
            if matches == 0 {
                0
            } else {
                2usize.pow(matches - 1)
            }
        })
        .sum()
}

fn compute_card_matches(line: &str) -> u32 {
    let (winnings, actual) = line.split_once(":").unwrap().1.split_once("|").unwrap();
    let winnings = winnings
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let actual = actual
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    actual.iter().filter(|n| winnings.contains(n)).count() as u32
}

fn second_part(input: &str) -> usize {
    let mut cards = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let matches = compute_card_matches(line);
        let current = cards[i];
        for j in i + 1..i + 1 + (matches as usize) {
            if let Some(x) = cards.get_mut(j) {
                *x += current;
            }
        }
    }

    cards.iter().sum()
}
