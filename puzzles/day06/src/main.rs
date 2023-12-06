fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    let race = Race::parse(input);

    race.iter()
        .map(|(time, distance)| compute_number_of_winnings(time, distance))
        .product()
}

fn compute_number_of_winnings(time: u64, distance: u64) -> u64 {
    let mut res = 0;
    for t in 1..=time / 2 {
        if t * (time - t) > distance {
            res += 1;
        }
    }
    res * 2 - if time & 1 == 0 { 1 } else { 0 }
}

fn second_part(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines.next().unwrap().
        split_once(":").unwrap().1.trim()
        .split_whitespace()
        .fold(String::new(),|acc, n| acc + n)
        .parse::<u64>().unwrap();
    let distance = lines.next().unwrap().
        split_once(":").unwrap().1.trim()
        .split_whitespace()
        .fold(String::new(),|acc, n| acc + n)
        .parse::<u64>().unwrap();
    compute_number_of_winnings(time, distance)
}

struct Race {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl Race {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let times = lines.next().unwrap().
            split_once(":").unwrap().1.trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let distances = lines.next().unwrap().
            split_once(":").unwrap().1.trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        Self { times, distances }
    }

    fn iter(&self) -> RaceIterator {
        RaceIterator { race: self, index: 0}
    }
}

struct RaceIterator<'a> {
    race: &'a Race,
    index: usize,
}

impl<'a> Iterator for RaceIterator<'a> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.race.times.len() {
            let res = Some((self.race.times[self.index], self.race.distances[self.index]));
            self.index += 1;
            res
        } else {
            None
        }
    }
}

