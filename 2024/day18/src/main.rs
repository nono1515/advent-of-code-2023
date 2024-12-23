use std::{collections::VecDeque, time::Instant};

#[derive(Debug)]
struct NoPathError;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| l.split(','))
        .map(|mut l| {
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>()
}

fn make_grid(pos: &Vec<(usize, usize)>, grid_size: usize, steps: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![true; grid_size]; grid_size];
    for s in 0..steps {
        grid[pos[s].0][pos[s].1] = false;
    }
    grid
}

fn bfs(grid: &Vec<Vec<bool>>) -> Result<usize, NoPathError> {
    let grid_size = grid.len();

    let mut visited = vec![vec![false; grid_size]; grid_size];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    while let Some((y, x, c)) = queue.pop_front() {
        if y == grid_size - 1 && x == grid_size - 1 {
            return Ok(c);
        }
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
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
                        if *cell {
                            queue.push_back((row_idx, col_idx, c + 1));
                        }
                    }
                }
            }
        }
    }

    Err(NoPathError {})
}

fn part1(input: &str, grid_size: usize, steps: usize) -> usize {
    let pos = parse_input(input);
    let grid = make_grid(&pos, grid_size, steps);

    if let Ok(c) = bfs(&grid) {
        c
    } else {
        panic!("no path found")
    }
}

fn part2(input: &str, grid_size: usize, steps: usize) -> (usize, usize) {
    let pos = parse_input(input);

    let mut low = steps;
    let mut high = pos.len();

    while high - low > 1 {
        let mid = low + (high - low) / 2;
        // We need to add 1 because the parameters `steps` is not inclusive
        let grid = make_grid(&pos, grid_size, mid + 1);
        match bfs(&grid) {
            Ok(_) => low = mid,
            Err(_) => high = mid,
        }
    }

    pos[high]
}

// This function is too slow too work on the full input although it works on the example
// The idea was to compute every possible path, keep track of them and remove the paths
// that get interupted by a falling byte. In the end, I just had to reuse function from
// part 1.
#[allow(dead_code)]
fn part2_by_path(input: &str, grid_size: usize, steps: usize) -> (usize, usize) {
    let pos = parse_input(input);
    let grid = make_grid(&pos, grid_size, steps);

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, vec![]));

    let mut path_out = vec![];

    while let Some((y, x, mut visited)) = queue.pop_front() {
        if y == grid_size - 1 && x == grid_size - 1 {
            path_out.push(visited);
            continue;
        }
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.push((y, x));
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
                        if *cell {
                            queue.push_back((row_idx, col_idx, visited.clone()));
                        }
                    }
                }
            }
        }
    }

    println!("BFS Done");

    let mut pos_idx = steps;
    while !path_out.is_empty() {
        let p = pos[pos_idx];

        path_out = path_out
            .into_iter()
            .filter(|visited| !visited.contains(&p))
            .collect::<Vec<Vec<(usize, usize)>>>();

        pos_idx += 1;
    }

    *pos.get(pos_idx - 1).unwrap_or(&(0, 0))
}

fn main() {
    // let input = "\
    // Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";

    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input, 71, 1024), now.elapsed());

    let now = Instant::now();
    println!(
        "Part 2: {:?} in {:?}",
        part2(&input, 71, 1024),
        now.elapsed()
    );
}

#[test]
fn test_large() {
    let input = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    assert_eq!(part1(&input, 7, 12), 22);

    assert_eq!(part2(&input, 7, 12), (6, 1));
}
