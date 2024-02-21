use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let matrix = Matrix::new(input);
    let mut final_sum = 0;
    let mut num_range = None;
    input.lines().enumerate().for_each(|(y, line)| {
        for (x, b) in line.bytes().enumerate() {
            if b.is_ascii_digit() {
                if num_range.is_none() {
                    num_range = Some((x, x));
                } else {
                    num_range = Some((num_range.unwrap().0, x));
                }
            }

            if !b.is_ascii_digit() || x == line.len() - 1 {
                if let Some((x1, x2)) = num_range {
                    if matrix
                        .get_surroundings(x1..=x2, y)
                        .iter()
                        .any(|s| contains_special_char(s))
                    {
                        let slice = line.get(x1..x2 + 1).unwrap();
                        let parsed = slice.parse::<usize>();
                        if parsed.is_err() {
                            println!(
                                "x1: {}, x2: {}, slice: {:?}\nline: {}",
                                x1,
                                x2,
                                line.get(..x2 + 1),
                                line
                            );
                        }
                        final_sum += parsed.unwrap();
                    }
                }
                num_range = None;
            }
        }
    });
    final_sum
}

fn contains_special_char(input: &str) -> bool {
    input.contains(|ch: char| !ch.is_digit(10) && ch != '.')
}

fn second_part(input: &str) -> usize {
    let engine = Engine::parse(input);
    engine.compute_gear_score_sum()
}

struct MatrixRow<'a> {
    row: &'a str,
}

#[derive(Debug)]
struct Matrix<'a> {
    width: usize,
    height: usize,
    data: &'a str,
}

impl<'a> Matrix<'a> {
    fn new(input: &'a str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        Self {
            width,
            height,
            data: input,
        }
    }

    fn get_surroundings(&self, x_range: RangeInclusive<usize>, y: usize) -> Vec<&'a str> {
        fn determine_x_range(
            x_range: &RangeInclusive<usize>,
            width: usize,
        ) -> RangeInclusive<usize> {
            let mut x1 = *x_range.start();
            let mut x2 = *x_range.end();
            if x1 > 0 {
                x1 -= 1;
            }
            if x2 < width - 1 {
                x2 += 1;
            }
            x1..=x2
        }
        let mut surroundings = Vec::new();
        let y_plus = y * (self.width + 1);
        let x_start = *x_range.start();
        let x_end = *x_range.end();
        if x_start > 0 {
            surroundings.push(&self.data[y_plus + (x_start - 1)..y_plus + x_start]);
        }

        if x_end < self.width - 1 {
            surroundings.push(&self.data[y_plus + (x_end + 1)..y_plus + (x_end + 2)]);
        }

        if y > 0 {
            let considered_x_range = determine_x_range(&x_range, self.width);
            let y_plus = (y - 1) * (self.width + 1);
            if let Some(val) = self
                .data
                .get(y_plus + *considered_x_range.start()..y_plus + *considered_x_range.end() + 1)
            {
                surroundings.push(val);
            }
        }

        if y < self.height - 1 {
            let considered_x_range = determine_x_range(&x_range, self.width);
            let y_plus = (y + 1) * (self.width + 1);
            if let Some(val) = self
                .data
                .get(y_plus + *considered_x_range.start()..y_plus + *considered_x_range.end() + 1)
            {
                surroundings.push(val);
            }
        }

        surroundings
    }
}

struct Engine<'a> {
    row_length: usize,
    rows: Vec<&'a str>,
    number_indexes: Vec<Vec<(usize, usize)>>,
    asterisk_indexes: Vec<Vec<usize>>,
}

impl<'a> Engine<'a> {
    fn parse(input: &'a str) -> Self {
        let mut rows = Vec::new();
        let mut number_indexes = Vec::new();
        let mut asterisk_indexes = Vec::new();
        let mut row_length = 0;
        let mut number_start = None;
        input.lines().for_each(|line| {
            row_length = line.len();
            rows.push(line);
            let mut number_indexes_row = Vec::new();
            let mut asterisk_indexes_row = Vec::new();
            line.bytes().enumerate().for_each(|(x, b)| {
                if b == b'*' {
                    asterisk_indexes_row.push(x);
                }
                if b.is_ascii_digit() {
                    if number_start.is_none() {
                        number_start = Some(x);
                    }
                    if x == line.len() - 1 {
                        number_indexes_row.push((number_start.unwrap(), x));
                        number_start = None;
                    }
                } else {
                    if let Some(start) = number_start {
                        number_indexes_row.push((start, x - 1));
                        number_start = None;
                    }
                }
            });
            number_indexes.push(number_indexes_row);
            asterisk_indexes.push(asterisk_indexes_row);
        });

        Self {
            row_length,
            rows,
            number_indexes,
            asterisk_indexes,
        }
    }

    fn compute_gear_score_sum(&self) -> usize {
        self.asterisk_indexes
            .iter()
            .enumerate()
            .map(|(y, asterix_row)| {
                asterix_row
                    .iter()
                    .flat_map(|&x| self.compute_single_gear(x, y))
                    .sum::<usize>()
            })
            .sum()
    }

    fn compute_single_gear(&self, x: usize, y: usize) -> Option<usize> {
        let mut neighbor_numbers = Vec::new();
        if self.number_indexes.get(y - 1).is_some() {
            let numbers =
                self.number_in_indexes((0.max(x - 1), (self.row_length - 1).min(x + 1)), y - 1);
            neighbor_numbers.extend(numbers);
        }
        if self.number_indexes.get(y).is_some() {
            let numbers =
                self.number_in_indexes((0.max(x - 1), (self.row_length - 1).min(x + 1)), y);
            neighbor_numbers.extend(numbers);
        }
        if self.number_indexes.get(y + 1).is_some() {
            let numbers =
                self.number_in_indexes((0.max(x - 1), (self.row_length - 1).min(x + 1)), y + 1);
            neighbor_numbers.extend(numbers);
        }

        if neighbor_numbers.len() == 2 {
            Some(neighbor_numbers.iter().product())
        } else {
            None
        }
    }

    fn number_in_indexes(&self, (x1, x2): (usize, usize), y: usize) -> Vec<usize> {
        let mut number_indexes = Vec::new();
        if let Some(row) = self.number_indexes.get(y) {
            for &(start, end) in row {
                if (x1 >= start && x1 <= end)
                    || (x2 >= start && x2 <= end)
                    || (x1 <= start && x2 >= end)
                {
                    let number = self
                        .rows
                        .get(y)
                        .expect("row not found")
                        .get(start..end + 1)
                        .expect("column not found")
                        .parse::<usize>()
                        .expect("parse failed");
                    number_indexes.push(number);
                }
            }
        }
        number_indexes
    }
}
