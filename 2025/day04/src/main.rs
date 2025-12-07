fn part1(grid: &mut Vec<Vec<bool>>) -> i32 {
    let mut total = 0;
    let mut to_be_removed = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !grid[i][j] {
                continue;
            }
            let neighbour = [-1, 0, 1]
                .map(|di| {
                    [-1, 0, 1]
                        .map(|dj| {
                            if di == 0 && dj == 0 {
                                false
                            } else {
                                *grid
                                    .get((i as i32 + di) as usize)
                                    .unwrap_or(&vec![false])
                                    .get((j as i32 + dj) as usize)
                                    .unwrap_or(&false)
                            }
                        })
                        .iter()
                        .filter(|&c| *c)
                        .count()
                })
                .iter()
                .sum::<usize>();
            if neighbour < 4 {
                to_be_removed.push((i, j));
                total += 1;
            }
        }
    }
    for (i, j) in &to_be_removed {
        grid[*i][*j] = false;
    }
    total
}

fn part2(grid: &mut Vec<Vec<bool>>) -> i32 {
    let mut total = 0;
    let mut new = part1(grid);
    while new > 0 {
        total += new;
        new = part1(grid);
    }
    total
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '@' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = parse_input(input);
    println!("part1: {}", part1(&mut grid));
    // whoupsi I need to reset the grid in between runs now that I delete some rolls of paper
    // this is not clean at all but it does the job
    let mut grid = parse_input(input);
    println!("part2: {}", part2(&mut grid));
}

#[test]
fn test_both_parts() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let mut grid = parse_input(input);
    assert_eq!(part1(&mut grid), 13);
    let mut grid = parse_input(input);
    assert_eq!(part2(&mut grid), 43);
}
