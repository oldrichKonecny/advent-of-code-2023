use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    let mut parts = input.split("\n\n");
    let seeds = parts
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let convert_table = parts.map(ConvertMap::parse).collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for convert_map in convert_table.iter() {
                value = convert_map.convert(value);
            }
            value
        })
        .min()
        .unwrap()
}

fn second_part(input: &str) -> u64 {
    let mut parts = input.split("\n\n");
    let seed_ranges = parts
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk[0]..=chunk[0] + chunk[1] - 1)
        .collect::<Vec<_>>();

    let convert_table = parts.map(ConvertMap::parse).collect::<Vec<_>>();

    seed_ranges
        .iter()
        .map(|seed_range| {
            let mut min = None;
            for seed in *seed_range.start()..=*seed_range.end() {
                let mut value = seed;
                for convert_map in convert_table.iter() {
                    value = convert_map.convert(value);
                }
                if min.is_none() {
                    min = Some(value);
                } else if value < min.unwrap() {
                    min = Some(value);
                }
            }
            min.unwrap()
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct ConvertMap {
    ranges: Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>,
}

impl ConvertMap {
    fn parse(input: &str) -> Self {
        let mut ranges = Vec::new();
        for line in input.lines().skip(1) {
            let mut split = line.split_whitespace();
            let destination_start = split.next().unwrap().parse::<u64>().unwrap();
            let source_start = split.next().unwrap().parse::<u64>().unwrap();
            let range_length = split.next().unwrap().parse::<u64>().unwrap();

            assert!(split.next().is_none());

            ranges.push((
                source_start..=source_start + range_length - 1,
                destination_start..=destination_start + range_length - 1,
            ));
        }
        Self { ranges }
    }

    fn convert(&self, input: u64) -> u64 {
        for (source, destination) in &self.ranges {
            if source.contains(&input) {
                return input - source.start() + destination.start();
            }
        }
        input
    }
}
