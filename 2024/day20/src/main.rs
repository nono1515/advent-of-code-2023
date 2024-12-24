use std::{collections::HashMap, time::Instant};

fn build_maze(input: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = (x, y)
                        }
                        if c == 'E' {
                            end = (x, y)
                        }
                        c != '#'
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    )
}

fn print_maze(maze: &Vec<Vec<bool>>) {
    for row in maze {
        println!(
            "{:?}",
            row.iter()
                .map(|c| if *c { '.' } else { '#' })
                .collect::<String>()
        );
    }
}

fn iter_ortho(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = pos;
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(move |(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
}

fn bfs(
    maze: &Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(i32, i32), usize> {
    let mut path = HashMap::new();

    path.insert((start.0 as i32, start.1 as i32), 0);
    let mut queue = vec![(start, 0)];
    while let Some((pos, dist)) = queue.pop() {
        if pos == end {
            continue;
        }
        for (x, y) in iter_ortho(pos) {
            if maze[y][x] && !path.contains_key(&(x as i32, y as i32)) {
                path.insert((x as i32, y as i32), dist + 1);
                queue.push(((x, y), dist + 1));
            }
        }
    }

    path
}

fn part1(input: &str, min_gain: usize) -> usize {
    let (maze, start, end) = build_maze(input);
    // print_maze(&maze);
    let path = bfs(&maze, start, end);

    let mut total = 0;

    for ((x, y), cost) in &path {
        total += [
            (-2, 0),
            (-1, 1),
            (0, 2),
            (1, 1),
            (2, 0),
            (1, -1),
            (0, -2),
            (-1, -1),
        ]
        .iter()
        .map(|(dx, dy)| (*x as i32 + dx, *y as i32 + dy))
        .filter(|pos| {
            path.get(pos)
                .unwrap_or(&usize::MAX)
                .saturating_add(min_gain + 2)
                <= *cost
        })
        .count();
    }

    total
}

fn part2(input: &str, min_gain: usize, max_cheat: usize) -> usize {
    let (maze, start, end) = build_maze(input);
    let path = bfs(&maze, start, end);

    // Sort points by cost to enable early breaking
    let mut sorted_points: Vec<_> = path.iter().collect();
    sorted_points.sort_by_key(|(_, &cost)| cost);

    // Pre-calculate y-strips for each x coordinate
    let mut strips: HashMap<i32, Vec<((i32, i32), usize)>> = HashMap::new();
    for (&(x, y), &cost) in &path {
        strips.entry(x).or_default().push(((x, y), cost));
    }

    // Sort strips by y coordinate
    for strip in strips.values_mut() {
        strip.sort_by_key(|((_, y), _)| *y);
    }

    let mut total = 0;
    for &(&(x, y), cost) in &sorted_points {
        for x_offset in -(max_cheat as i32)..=max_cheat as i32 {
            let x2 = x + x_offset;
            if let Some(strip) = strips.get(&x2) {
                let y_range = max_cheat as i32 - x_offset.abs();

                // Binary search for y range bounds
                let start_idx = strip.partition_point(|((_, y2), _)| *y2 < y - y_range);
                let end_idx = strip.partition_point(|((_, y2), _)| *y2 <= y + y_range);

                for &((_, y2), c2) in &strip[start_idx..end_idx] {
                    if c2 + min_gain + ((x - x2).abs() + (y - y2).abs()) as usize <= *cost {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input, 100), now.elapsed());

    let now = Instant::now();
    println!(
        "Part 2: {:?} in {:?}",
        part2(&input, 100, 20),
        now.elapsed()
    );
}

#[test]
fn test_large() {
    let input = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    assert_eq!(part1(&input, 10), 10);

    // assert_eq!(part2(&input, 50, 20), 285);
    let mut prev = 0;
    for lim in (50..=76).rev().step_by(2) {
        let n = part2(input, lim, 20);
        println!(
            "There are {} cheats that save {} picoseconds.",
            n - prev,
            lim
        );
        prev = n;
    }
    assert_eq!(part2(&input, 50, 20), 285);
}
