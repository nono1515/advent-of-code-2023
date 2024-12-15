use std::time::Instant;

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let mut q = vec![(i, j)];
                let mut area = 0;
                let mut perimeter = 0;
                while let Some((y, x)) = q.pop() {
                    if visited[y][x] {
                        continue;
                    }
                    visited[y][x] = true;
                    let mut neighbors = vec![];

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
                                    if *cell == grid[i][j] {
                                        neighbors.push((row_idx, col_idx));
                                        q.push((row_idx, col_idx));
                                    }
                                }
                            }
                        }
                    }

                    area += 1;
                    perimeter += 4 - neighbors.len();
                }
                // println!(
                //     "area: {}, perimeter: {}, character: {}",
                //     area, perimeter, grid[i][j]
                // );

                total += area * perimeter;
            }
        }
    }
    total
}

fn check_outer_corner(neighbors: &Vec<Option<(usize, usize)>>, start: usize) -> bool {
    let corner = neighbors
        .iter()
        .cycle()
        .skip(start)
        .take(3)
        .map(|n| n.is_none())
        .collect::<Vec<bool>>();
    corner[0] && corner[2]
}

fn check_inner_corner(neighbors: &Vec<Option<(usize, usize)>>, start: usize) -> bool {
    neighbors
        .iter()
        .cycle()
        .skip(start)
        .take(3)
        .map(|n| n.is_none())
        .collect::<Vec<bool>>()
        == vec![false, true, false]
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let mut q = vec![(i, j)];
                let mut current = vec![vec![false; grid[0].len()]; grid.len()];
                while let Some((y, x)) = q.pop() {
                    current[y][x] = true;
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
                                    if *cell == grid[i][j] {
                                        q.push((row_idx, col_idx));
                                    }
                                }
                            }
                        }
                    }
                }

                let mut area = 0;
                let mut corners = 0;

                for y in 0..current.len() {
                    for x in 0..current[0].len() {
                        if !current[y][x] {
                            continue;
                        }
                        let neighbors = [
                            (-1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                            (1, 0),
                            (1, -1),
                            (0, -1),
                            (-1, -1),
                        ]
                        .iter()
                        .map(|(dy, dx)| {
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
                            if current.get(row_idx?).and_then(|row| row.get(col_idx?))
                                == Some(&true)
                            {
                                Some((row_idx?, col_idx?))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                        // ...   ##.   .##   ...
                        // ##.   ##.   .##   .##
                        // ##.   ...   ...   .##
                        corners += (0..4)
                            .filter(|i| check_inner_corner(&neighbors, 2 * i))
                            .count();
                        // ##.   ###   ###   .##
                        // ###   ###   ###   ###
                        // ###   ##.   .##   ###
                        corners += (0..4)
                            .filter(|i| check_outer_corner(&neighbors, 2 * i))
                            .count();

                        area += 1;
                    }
                }

                // println!(
                //     "area: {}, edges: {}, character: {}",
                //     area, corners, grid[i][j]
                // );

                total += area * corners;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&grid), now.elapsed());

    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(&grid), now.elapsed());
}

#[test]
fn test_example() {
    let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(part1(&grid), 1930);

    assert_eq!(part2(&grid), 1206);
}

#[test]
fn test_example_part2() {
    let input = "\
AAAA
BBCD
BBCC
EEEC";
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(part2(&grid), 80);

    let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(part2(&grid), 436);

    let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(part2(&grid), 236);
}

#[test]
fn test_example_part2_2() {
    let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(part2(&grid), 368);
}
