use std::collections::HashMap;
use std::time::Instant;

fn update_antinode_next(antinode: &mut Vec<Vec<bool>>, pos1: (usize, usize), pos2: (usize, usize)) {
    let row_idx = (2 * pos1.1).checked_sub(pos2.1);
    let col_idx = (2 * pos1.0).checked_sub(pos2.0);

    if let (Some(row_idx), Some(col_idx)) = (row_idx, col_idx) {
        if let Some(row) = antinode.get_mut(row_idx) {
            if let Some(cell) = row.get_mut(col_idx) {
                *cell = true;
            }
        }
    }
}

fn update_antinode_line(antinode: &mut Vec<Vec<bool>>, pos1: (usize, usize), pos2: (usize, usize)) {
    fn trace_line(antinode: &mut Vec<Vec<bool>>, start: (f32, f32), dx: f32, dy: f32) {
        let mut pos = start;
        while pos.0 >= 0.0
            && pos.0 < antinode[0].len() as f32
            && pos.1 >= 0.0
            && pos.1 < antinode.len() as f32
        {
            if let Some(row) = antinode.get_mut(pos.1 as usize) {
                if let Some(cell) = row.get_mut(pos.0 as usize) {
                    *cell = true;
                }
            }
            pos = (pos.0 + dx, pos.1 + dy);
        }
    }

    let dx = pos2.0 as f32 - pos1.0 as f32;
    let dy = pos2.1 as f32 - pos1.1 as f32;
    let start = (pos1.0 as f32, pos1.1 as f32);

    trace_line(antinode, start, dx, dy);
    trace_line(antinode, start, -dx, -dy);
}

fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut antinode = vec![vec![false; grid[0].len()]; grid.len()];
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c != '.' {
                if let Some(antenna_list) = antennas.get_mut(&c) {
                    for antenna in antenna_list.iter() {
                        update_antinode_next(&mut antinode, *antenna, (x, y));
                        update_antinode_next(&mut antinode, (x, y), *antenna);
                    }
                    antenna_list.push((x, y));
                } else {
                    antennas.insert(c, vec![(x, y)]);
                }
            }
        }
    }

    // for row in &antinode {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|x| if *x { '#' } else { '.' })
    //             .collect::<String>()
    //     );
    // }

    antinode.iter().flatten().filter(|x| **x).count() as u64
}

fn part2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut antinode = vec![vec![false; grid[0].len()]; grid.len()];
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c != '.' {
                if let Some(antenna_list) = antennas.get_mut(&c) {
                    for antenna in antenna_list.iter() {
                        update_antinode_line(&mut antinode, *antenna, (x, y));
                    }
                    antenna_list.push((x, y));
                } else {
                    antennas.insert(c, vec![(x, y)]);
                }
            }
        }
    }

    antinode.iter().flatten().filter(|x| **x).count() as u64
}

fn main() {
    let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(input), now.elapsed());
    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(input), now.elapsed());
}
