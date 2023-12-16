fn main() {
    let input = include_str!("../input.txt");

    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn first_part(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    beam_path(&mut grid, (0, 0), Direction::East);
    grid.get_marked_tiles()
}

fn second_part(input: &str) -> usize {
    let grid = Grid::parse(input);
    let mut max = 0;
    for x in 0..grid.grid[0].len() {
        let mut try_grid = grid.clone();
        beam_path(&mut try_grid, (0, x), Direction::South);
        let marked = try_grid.get_marked_tiles();
        if marked > max {
            max = marked;
        }
        let mut try_grid = grid.clone();
        beam_path(&mut try_grid, (grid.grid.len() - 1, x), Direction::North);
        let marked = try_grid.get_marked_tiles();
        if marked > max {
            max = marked;
        }
    }
    for y in 0..grid.grid.len() {
        let mut try_grid = grid.clone();
        beam_path(&mut try_grid, (y, 0), Direction::East);
        let marked = try_grid.get_marked_tiles();
        if marked > max {
            max = marked;
        }
        let mut try_grid = grid.clone();
        beam_path(&mut try_grid, (y, grid.grid[0].len() - 1), Direction::West);
        let marked = try_grid.get_marked_tiles();
        if marked > max {
            max = marked;
        }
    }
    max
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<(char, [Option<Direction>; 4])>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|ch| (ch, [None; 4])).collect())
            .collect();
        Grid { grid }
    }

    fn get_marked_tiles(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter()
                .filter(|(_, marked)| marked.iter().any(|dir| dir.is_some()))
                .count())
            .sum()
    }

    fn print_marked_tiles(&self) {
        for row in &self.grid {
            for (_, marked) in row {
                print!("{}", marked.iter().filter(|dir| dir.is_some()).count());
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn beam_path(grid: &mut Grid, (mut y, mut x): (usize, usize), direction: Direction) {
    match direction {
        Direction::North => {
            while y >= 0 {
                if grid.grid[y][x].1[0].is_some() {
                    return;
                }
                grid.grid[y][x].1[0] = Some(Direction::North);
                match grid.grid[y][x].0 {
                    '.' => {
                        if y == 0 {
                            break;
                        }
                        y -= 1;
                    },
                    '|' => {
                        if y == 0 {
                            break;
                        }
                        y -= 1;
                    },
                    '-' => {
                        if x > 0 {
                            beam_path(grid, (y, x - 1), Direction::West);
                        }
                        if x < grid.grid[y].len() - 1 {
                            return beam_path(grid, (y, x + 1), Direction::East);
                        }
                        break;
                    },
                    '\\' => {
                        if x > 0 {
                            return beam_path(grid, (y, x - 1), Direction::West);
                        }
                        break;
                    },
                    '/' => {
                        if x < grid.grid[y].len() - 1 {
                            return beam_path(grid, (y, x + 1), Direction::East);
                        }
                        break;
                    },
                    _ => panic!("Invalid input")
                }

            }
        }
        Direction::South => {
            while y < grid.grid.len() {
                if grid.grid[y][x].1[1].is_some() {
                    return;
                }
                grid.grid[y][x].1[1] = Some(Direction::South);
                match grid.grid[y][x].0 {
                    '.' => {
                        if y == grid.grid.len() - 1 {
                            break;
                        }
                        y += 1;
                    },
                    '|' => {
                        if y == grid.grid.len() - 1 {
                            break;
                        }
                        y += 1;
                    },
                    '-' => {
                        if x > 0 {
                            beam_path(grid, (y, x - 1), Direction::West);
                        }
                        if x < grid.grid[y].len() - 1 {
                            return beam_path(grid, (y, x + 1), Direction::East);
                        }
                        break;
                    },
                    '\\' => {
                        if x < grid.grid[y].len() - 1 {
                            return beam_path(grid, (y, x + 1), Direction::East);
                        }
                        break;
                    },
                    '/' => {
                        if x > 0 {
                            return beam_path(grid, (y, x - 1), Direction::West);
                        }
                        break;
                    },
                    _ => panic!("Invalid input")
                }
            }
        }
        Direction::West => {
            while x >= 0 {
                if grid.grid[y][x].1[2].is_some() {
                    return;
                }
                grid.grid[y][x].1[2] = Some(Direction::West);
                match grid.grid[y][x].0 {
                    '.' => {
                        if x == 0 {
                            break;
                        }
                        x -= 1;
                    },
                    '|' => {
                        if y > 0 {
                            beam_path(grid, (y - 1, x), Direction::North);
                        }
                        if y < grid.grid.len() - 1 {
                            return beam_path(grid, (y + 1, x), Direction::South);
                        }
                        break;
                    },
                    '-' => {
                        if x == 0 {
                            break;
                        }
                        x -= 1;
                    },
                    '\\' => {
                        if y > 0 {
                            return beam_path(grid, (y - 1, x), Direction::North);
                        }
                        break;
                    },
                    '/' => {
                        if y < grid.grid.len() - 1 {
                            return beam_path(grid, (y + 1, x), Direction::South);
                        }
                        break;
                    },
                    _ => panic!("Invalid input")
                }
            }
        }
        Direction::East => {
            while x < grid.grid[y].len() {
                if grid.grid[y][x].1[3].is_some() {
                    return;
                }
                grid.grid[y][x].1[3] = Some(Direction::East);
                match grid.grid[y][x].0 {
                    '.' => {
                        if x == grid.grid[y].len() - 1 {
                            break;
                        }
                        x += 1;
                    },
                    '|' => {
                        if y > 0 {
                            beam_path(grid, (y - 1, x), Direction::North);
                        }
                        if y < grid.grid.len() - 1 {
                            return beam_path(grid, (y + 1, x), Direction::South);
                        }
                        break;
                    },
                    '-' => {
                        if x == grid.grid[y].len() - 1 {
                            break;
                        }
                        x += 1;
                    },
                    '\\' => {
                        if y < grid.grid.len() - 1 {
                            return beam_path(grid, (y + 1, x), Direction::South);
                        }
                        break;
                    },
                    '/' => {
                        if y > 0 {
                            return beam_path(grid, (y - 1, x), Direction::North);
                        }
                        break;
                    },
                    _ => panic!("Invalid input")
                }
            }
        }
    }
}

