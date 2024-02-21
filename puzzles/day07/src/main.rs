use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> u64 {
    let mut card_hands = input
        .lines()
        .map(|line| line.parse::<CardHand>().unwrap())
        .collect::<Vec<_>>();

    card_hands.sort();
    card_hands
        .iter()
        .enumerate()
        .map(|(index, card_hand)| (index as u64 + 1) * card_hand.value)
        .sum()
}

fn second_part(input: &str) -> u64 {
    let mut card_hands = input.lines().map(|line| parse(line)).collect::<Vec<_>>();

    card_hands.sort();
    card_hands
        .iter()
        .enumerate()
        .map(|(index, card_hand)| (index as u64 + 1) * card_hand.value)
        .sum()
}

#[derive(Debug, Eq)]
struct CardHand {
    name: String,
    card_hand_type: CardHandType,
    value: u64,
}

impl FromStr for CardHand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = s.split_whitespace();
        let name = cards.next().unwrap().to_string();
        let value = cards.next().unwrap().parse::<u64>().unwrap();

        let mut char_map = HashMap::new();
        for c in name.chars() {
            let count = char_map.entry(c).or_insert(0);
            *count += 1;
        }

        let card_hand_type = match char_map.len() {
            1 => CardHandType::FiveOfAKind,
            2 => {
                let mut values = char_map.values();
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                if *first == 4 || *second == 4 {
                    CardHandType::FourOfAKind
                } else {
                    CardHandType::FullHouse
                }
            }
            3 => {
                let mut values = char_map.values();
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                let third = values.next().unwrap();
                if *first == 3 || *second == 3 || *third == 3 {
                    CardHandType::ThreeOfAKind
                } else {
                    CardHandType::TwoPairs
                }
            }
            4 => CardHandType::OnePair,
            5 => CardHandType::HighCard,
            _ => unreachable!(),
        };

        Ok(Self {
            name,
            card_hand_type,
            value,
        })
    }
}

impl PartialEq<Self> for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.card_hand_type == other.card_hand_type {
            for (s, o) in self.name.chars().zip(other.name.chars()) {
                let ord = char_cmp_value(s).cmp(&char_cmp_value(o));
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            Ordering::Equal
        } else {
            self.card_hand_type
                .cmp_value()
                .cmp(&other.card_hand_type.cmp_value())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum CardHandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardHandType {
    fn cmp_value(&self) -> u64 {
        match self {
            CardHandType::HighCard => 1,
            CardHandType::OnePair => 2,
            CardHandType::TwoPairs => 3,
            CardHandType::ThreeOfAKind => 4,
            CardHandType::FullHouse => 5,
            CardHandType::FourOfAKind => 6,
            CardHandType::FiveOfAKind => 7,
        }
    }
}

fn char_cmp_value(c: char) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u64,
    }
}

fn parse(s: &str) -> CardHand {
    let mut cards = s.split_whitespace();
    let name = cards.next().unwrap().to_string();
    let value = cards.next().unwrap().parse::<u64>().unwrap();

    let mut char_map = HashMap::new();
    for c in name.chars() {
        let count = char_map.entry(c).or_insert(0);
        *count += 1;
    }

    let jokers = char_map.get(&'J');

    let card_hand_type = match (char_map.len(), jokers) {
        (1, None) | (1, Some(_)) | (2, Some(_)) => CardHandType::FiveOfAKind,
        (2, None) => {
            let mut values = char_map.values();
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            if *first == 4 || *second == 4 {
                CardHandType::FourOfAKind
            } else {
                CardHandType::FullHouse
            }
        }
        (3, None) => {
            let mut values = char_map.values();
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            let third = values.next().unwrap();
            if *first == 3 || *second == 3 || *third == 3 {
                CardHandType::ThreeOfAKind
            } else {
                CardHandType::TwoPairs
            }
        }
        (3, Some(&j)) => {
            let mut values = char_map.values();
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            let third = values.next().unwrap();

            if j == 1 && *first != 3 && *second != 3 && *third != 3 {
                CardHandType::FullHouse
            } else {
                CardHandType::FourOfAKind
            }
        }
        (4, None) | (5, Some(_)) => CardHandType::OnePair,
        (4, Some(_)) => CardHandType::ThreeOfAKind,
        (5, None) => CardHandType::HighCard,
        _ => unreachable!(),
    };

    CardHand {
        name,
        card_hand_type,
        value,
    }
}

// 243276310 too high
// 243305080 too high
// 243101568
