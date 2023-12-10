fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> i64 {
    input.lines()
        .map(parse_line)
        .map(|numbers|determine_next(&numbers))
        .sum()
}

fn determine_next(numbers: &[i64]) -> i64 {
    let mut number_map = Vec::new();
    let mut temp = numbers.iter().cloned().collect::<Vec<_>>();
    loop {
        temp = temp.windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();
        if !temp.iter().all(|&n| n == 0) {
            number_map.push(temp.clone());
        } else {
            break;
        }
    }

    numbers.last().unwrap() + number_map.iter().rev().skip(1)
        .fold(number_map.last().unwrap().last().unwrap().clone(), |acc, nums| {
            nums.last().unwrap() + acc
        })
}

fn second_part(input: &str) -> i64 {
    input.lines()
        .map(parse_line)
        .map(|numbers| numbers.iter().rev().map(|n| n.to_owned()).collect::<Vec<_>>())
        .map(|rev_numbers|determine_next(&rev_numbers))
        .sum()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}