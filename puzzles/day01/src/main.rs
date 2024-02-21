fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first = None;
            let mut second = None;
            for c in line.chars() {
                if c.is_digit(10) {
                    let val = c.to_digit(10).unwrap();
                    first.get_or_insert(val);
                    second = Some(val);
                }
            }
            first
                .map(|f| f * 10)
                .map(|f| f + second.unwrap())
                .unwrap_or_default()
        })
        .sum()
}

fn second_part(input: &str) -> u32 {
    let regex_first =
        regex::Regex::new(r"^.*?([1-9]|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();
    let regex_last =
        regex::Regex::new(r"^.*([1-9]|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();
    input
        .lines()
        .map(|line| {
            let first = regex_first
                .captures(line)
                .map(|c| c.get(1).unwrap())
                .map(|g| g.as_str())
                .map(parse_number)
                .expect(&format!("Invalid line first: {}", line));

            let last = regex_last
                .captures(line)
                .map(|c| c.get(1).unwrap())
                .map(|g| g.as_str())
                .map(parse_number)
                .expect(&format!("Invalid line last: {}", line));

            first * 10 + last
        })
        .sum()
}

fn parse_number(number: &str) -> u32 {
    match number {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => number
            .parse()
            .expect(&format!("Invalid number: {}", number)),
    }
}
