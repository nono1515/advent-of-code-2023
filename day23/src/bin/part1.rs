fn main() {
    let input: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    let input = include_str!("../../input.txt");

    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '.' => TileType::Path,
                    '>' => TileType::SlopeRight,
                    'v' => TileType::SlopeDown,
                    '^' => TileType::SlopeUp,
                    '<' => TileType::SlopeLeft,
                    '#' => TileType::Wall,
                    c => panic!("Unknown char: {}", c),
                })
                .collect()
        })
        .collect();

    let starting_pos = (0, map[0].iter().position(|t| *t == TileType::Path).unwrap());
    let ending_pos = (
        map.len() - 1,
        map[map.len() - 1]
            .iter()
            .position(|t| *t == TileType::Path)
            .unwrap(),
    );

    let mut stack = vec![(starting_pos, 0, vec![])];
    let mut max_len = 0;

    while let Some((pos, steps, prev_pos)) = stack.pop() {
        if pos == ending_pos {
            max_len = max_len.max(steps);
            continue;
        }
        let d: Vec<(i32, i32)> = match map[pos.0][pos.1] {
            TileType::SlopeUp => vec![(0, -1)],
            TileType::SlopeDown => vec![(0, 1)],
            TileType::SlopeRight => vec![(1, 0)],
            TileType::SlopeLeft => vec![(-1, 0)],
            TileType::Path => vec![(0, -1), (0, 1), (1, 0), (-1, 0)],
            TileType::Wall => panic!("Cannot move into wall but current on one"),
        };
        for (dy, dx) in &d {
            let new_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
            if prev_pos.contains(&new_pos) {
                continue;
            }
            match map.get(new_pos.0).and_then(|row| row.get(new_pos.1)) {
                Some(TileType::Wall) => {}
                None => {}
                Some(_) => {
                    let mut next_prev_pos = prev_pos.clone();
                    next_prev_pos.push(pos);
                    stack.push((new_pos, steps + 1, next_prev_pos))
                }
            }
        }
    }

    println!("{}", max_len);
}

#[derive(PartialEq, Debug)]
enum TileType {
    Path,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
    Wall,
}
