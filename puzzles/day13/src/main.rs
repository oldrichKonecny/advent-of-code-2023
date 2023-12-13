fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    input.split("\n\n")
        .map(ReflectionPattern::parse)
        .filter_map(|pattern| pattern.determine_value(None))
        .sum()
}

fn second_part(input: &str) -> usize {
    input.split("\n\n")
        .map(ReflectionPattern::parse)
        .map(|pattern| pattern.determine_new_value())
        .sum()
}

#[derive(Debug, Clone)]
struct ReflectionPattern {
    pattern: Vec<Vec<u8>>,
}

impl ReflectionPattern {
    fn parse(input: &str) -> Self {
        let pattern = input.lines()
            .map(|line| line.bytes().collect())
            .collect();
        Self {
            pattern,
        }
    }

    fn determine_value(&self, old_value: Option<usize>) -> Option<usize> {
        for i in 1..self.pattern.len() {
           if self.check_horizontal(i) {
               let res = i * 100;
               if old_value.is_none() || old_value.unwrap() != res {
                   return Some(res);
               }
           }
        }
        for i in 1..self.pattern[0].len() {
            if self.check_vertical(i) {
                if old_value.is_none() || old_value.unwrap() != i {
                    return Some(i);
                }
            }
        }
        None
    }

    fn determine_new_value(&self) -> usize {
        let old_value = self.determine_value(None);
        let mut new_pattern = self.clone();
        for i in 0..self.pattern.len() {
            for j in 0..self.pattern[0].len() {
                let byte = new_pattern.pattern[i][j];
                new_pattern.pattern[i][j] = match byte {
                    b'.' => b'#',
                    b'#' => b'.',
                    _ => panic!("Unknown byte: {}", byte),
                };
                if let Some(value) = new_pattern.determine_value(old_value) {
                    return value;
                }
                new_pattern.pattern[i][j] = byte;
            }
        }
        0
    }

    fn check_horizontal(&self, row: usize) -> bool {
        assert_ne!(row, 0);
        let first = &self.pattern[row - 1];
        let sec = &self.pattern[row];
        if first == sec {
            for i in 1..(self.pattern.len() / 2 + 1) {
                if row as i32 - i as i32 - 1 < 0 {
                    return true;
                }
                let first = &self.pattern[row - i - 1];
                if let Some(sec) = self.pattern.get(row + i) {
                    if first != sec {
                        return false;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn check_vertical(&self, col: usize) -> bool {
        assert_ne!(col, 0);
        let first = self.get_column(col-1);
        let sec = self.get_column(col);
        if first == sec {
            for i in 1..(self.pattern[0].len() / 2 + 1) {
                if col as i32 - i as i32 - 1 < 0 {
                    return true;
                }
                let first = self.get_column(col - i - 1);
                if col + i < self.pattern[0].len() {
                    let sec = self.get_column(col + i);
                    if first != sec {
                        return false;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn get_column(&self, col: usize) -> Vec<u8> {
        assert!(col < self.pattern[0].len());
        self.pattern.iter()
            .map(|row| row[col])
            .collect()
    }
}