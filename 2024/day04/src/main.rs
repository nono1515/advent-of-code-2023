const DIRS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

fn match_pattern(grid: &Vec<Vec<char>>, i: usize, j: usize, pattern: &str) -> i32 {
    let mut n_matches = 0;
    for dir in DIRS {
        let mut pos = (i, j);
        let (dy, dx) = match dir {
            "N" => (-1, 0),
            "NE" => (-1, 1),
            "E" => (0, 1),
            "SE" => (1, 1),
            "S" => (1, 0),
            "SW" => (1, -1),
            "W" => (0, -1),
            "NW" => (-1, -1),
            _ => unreachable!(),
        };

        for c in pattern.chars() {
            if grid.get(pos.0).and_then(|row| row.get(pos.1)) == Some(&c) {
                if c == 'S' {
                    n_matches += 1;
                }
                match dy {
                    1 => pos.0 += 1,
                    -1 => {
                        if let Some(new_y) = pos.0.checked_sub(1) {
                            pos.0 = new_y;
                        } else {
                            // Out of grid
                            break;
                        }
                    }
                    0 => (),
                    _ => unreachable!(),
                }
                match dx {
                    1 => pos.1 += 1,
                    -1 => {
                        if let Some(new_x) = pos.1.checked_sub(1) {
                            pos.1 = new_x;
                        } else {
                            // Out of grid
                            break;
                        }
                    }
                    0 => (),
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        }
    }
    n_matches
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let pattern = "XMAS";
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let matches = match_pattern(&grid, i, j, pattern);
            count += matches;
        }
    }
    count
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 'A' {
                let nw_se = match (grid[i - 1][j - 1], grid[i + 1][j + 1]) {
                    ('M', 'S') | ('S', 'M') => true,
                    _ => false,
                };
                let ne_sw = match (grid[i - 1][j + 1], grid[i + 1][j - 1]) {
                    ('M', 'S') | ('S', 'M') => true,
                    _ => false,
                };
                if nw_se && ne_sw {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
