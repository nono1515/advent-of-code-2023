use std::time::Instant;

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn n_paths(grid: &Vec<Vec<u8>>, i: usize, j: usize, distinct: bool) -> usize {
    let mut queue = vec![(i, j)];
    let mut visited_nine = vec![];

    while let Some((y, x)) = queue.pop() {
        let n = grid[y as usize][x as usize];
        for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let row_idx = {
                match dy {
                    -1 => y.checked_sub(1),
                    0 => Some(y),
                    1 => y.checked_add(1),
                    _ => unreachable!(),
                }
            };
            let col_idx = {
                match dx {
                    -1 => x.checked_sub(1),
                    0 => Some(x),
                    1 => x.checked_add(1),
                    _ => unreachable!(),
                }
            };

            if let (Some(row_idx), Some(col_idx)) = (row_idx, col_idx) {
                if let Some(row) = grid.get(row_idx) {
                    if let Some(cell) = row.get(col_idx) {
                        if *cell == n + 1 {
                            match cell {
                                9 => {
                                    if distinct || !visited_nine.contains(&(row_idx, col_idx)) {
                                        visited_nine.push((row_idx, col_idx))
                                    }
                                }
                                _ => queue.push((row_idx, col_idx)),
                            }
                        }
                    }
                }
            }
        }
    }

    visited_nine.len()
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                total += n_paths(&grid, i, j, false);
            }
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                total += n_paths(&grid, i, j, true);
            }
        }
    }

    total
}

fn main() {
    let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(input), now.elapsed());
    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(input), now.elapsed());
}
