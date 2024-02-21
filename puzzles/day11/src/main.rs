fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let sky_map = SkyMap::parse(input);
    sky_map.sum_galaxies_distances(|n| n * 2)
}

fn second_part(input: &str) -> usize {
    let sky_map = SkyMap::parse(input);
    sky_map.sum_galaxies_distances(|n| n * 1_000_000)
}

#[derive(Debug)]
struct SkyMap {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl SkyMap {
    fn parse(input: &str) -> Self {
        let sky_map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut galaxies = Vec::new();
        let mut empty_rows = Vec::new();
        for (y, row) in sky_map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '#' {
                    galaxies.push((y, x));
                }
            }
            if row.iter().all(|&c| c == '.') {
                empty_rows.push(y);
            }
        }

        let mut empty_cols = Vec::new();
        for i in 0..sky_map[0].len() {
            if sky_map.iter().all(|row| row[i] == '.') {
                empty_cols.push(i);
            }
        }

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn sum_galaxies_distances(&self, gravity_constants: fn(usize) -> usize) -> usize {
        let mut all_distances = 0;
        for (index, galaxy1) in self.galaxies.iter().enumerate() {
            for galaxy2 in self.galaxies.iter().skip(index) {
                let (distance_y, distance_x) = (
                    (galaxy1.0 as i32 - galaxy2.0 as i32).abs() as usize,
                    (galaxy1.1 as i32 - galaxy2.1 as i32).abs() as usize,
                );
                let empty_y = self
                    .empty_rows
                    .iter()
                    .filter(|&&r| r > galaxy1.0.min(galaxy2.0) && r < galaxy1.0.max(galaxy2.0))
                    .count();
                let empty_x = self
                    .empty_cols
                    .iter()
                    .filter(|&&c| c > galaxy1.1.min(galaxy2.1) && c < galaxy1.1.max(galaxy2.1))
                    .count();
                let distance_y = (distance_y - empty_y) + gravity_constants(empty_y);
                let distance_x = (distance_x - empty_x) + gravity_constants(empty_x);
                all_distances += distance_y + distance_x
            }
        }
        all_distances
    }
}
