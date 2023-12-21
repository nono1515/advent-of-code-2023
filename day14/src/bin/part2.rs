use std::collections::HashMap;

fn main() {
    let input = include_bytes!("../../input.txt");
    let width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let n_cycles = 1000000000;

    let mut grid = vec![vec![Space::Empty; width - 1]; input.len() / width];
    for (i, &b) in input.iter().enumerate() {
        match b {
            b'\n' | b'.' => (),
            b'#' => grid[i / width][i % width] = Space::StableRock,
            b'O' => grid[i / width][i % width] = Space::MovingRock,
            _ => unreachable!(),
        }
    }

    let mut period = 0;
    let mut offset = 0;
    let mut grids = HashMap::new();
    for i in 1..=n_cycles {
        grid = rotate_grid(grid, Dir::Up);
        grid = rotate_grid(grid, Dir::Left);
        grid = rotate_grid(grid, Dir::Down);
        grid = rotate_grid(grid, Dir::Right);

        if grids.contains_key(&grid) {
            period = i - grids[&grid];
            offset = i;
            break;
        }
        grids.entry(grid.clone()).or_insert(i);
    }

    let rotation_to_go = (n_cycles - offset) % period;

    for _ in 0..rotation_to_go {
        grid = rotate_grid(grid, Dir::Up);
        grid = rotate_grid(grid, Dir::Left);
        grid = rotate_grid(grid, Dir::Down);
        grid = rotate_grid(grid, Dir::Right);
    }

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for &s in row.iter() {
            if s == Space::MovingRock {
                sum += grid.len() - i;
            }
        }
    }

    println!("{}", sum);
}

fn rotate_grid(mut grid: Vec<Vec<Space>>, direction: Dir) -> Vec<Vec<Space>> {
    match direction {
        Dir::Up => (),
        Dir::Down => {
            grid.reverse();
        }
        Dir::Left => grid = transpose(grid),
        Dir::Right => {
            grid = transpose(grid);
            grid.reverse()
        }
    };
    let mut new_grid = grid.clone();
    let mut swap_vals = vec![0; grid[0].len()];
    // let mut col_vals = vec![0; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, &s) in row.iter().enumerate() {
            match s {
                Space::StableRock => swap_vals[j] = i + 1,
                Space::MovingRock if swap_vals[j] != i => {
                    new_grid[i][j] = Space::Empty;
                    new_grid[swap_vals[j]][j] = Space::MovingRock;
                    swap_vals[j] += 1;
                }
                Space::MovingRock => swap_vals[j] += 1,
                Space::Empty => (),
            }
        }
    }

    match direction {
        Dir::Up => (),
        Dir::Down => {
            new_grid.reverse();
        }
        Dir::Left => new_grid = transpose(new_grid),
        Dir::Right => {
            new_grid.reverse();
            new_grid = transpose(new_grid);
        }
    };

    new_grid
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    MovingRock,
    StableRock,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Empty => write!(f, "."),
            Space::MovingRock => write!(f, "O"),
            Space::StableRock => write!(f, "#"),
        }
    }
}

fn print_grid(grid: &[Vec<Space>]) {
    for row in grid {
        for s in row {
            print!("{}", s);
        }
        println!();
    }
    println!();
}

enum Dir {
    Up,
    Left,
    Down,
    Right,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
