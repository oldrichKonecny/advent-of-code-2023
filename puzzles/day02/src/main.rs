fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn second_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split(";")
                .map(Game::parse)
                .reduce(|a, b| {
                    Game::new(a.red.max(b.red), a.green.max(b.green), a.blue.max(b.blue))
                })
                .map(|game| game.power())
                .unwrap_or_default()
        })
        .sum()
}

fn first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game_number, results) = line.split_once(":").unwrap();
            let game_number = game_number
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            if results
                .split(";")
                .map(Game::parse)
                .all(|game| is_game_possible(&game))
            {
                game_number
            } else {
                0
            }
        })
        .sum()
}

fn is_game_possible(game: &Game) -> bool {
    game.red <= 12 && game.green <= 13 && game.blue <= 14
}

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn parse(game: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        game.split(",").for_each(|part| {
            let (value, color) = part.trim().split_once(" ").unwrap();
            let value = value.trim().parse::<u32>().unwrap();
            match color {
                "red" => red = value,
                "green" => green = value,
                "blue" => blue = value,
                _ => panic!("Unknown color: {}", color),
            }
        });

        Self { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
