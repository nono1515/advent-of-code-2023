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

    // let input = include_str!("../../input.txt");

    let mut map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '.' | '>' | 'v' | '^' | '<' => TileType::Path(None),
                    '#' => TileType::Wall,
                    c => panic!("Unknown char: {}", c),
                })
                .collect()
        })
        .collect();

    let starting_pos = (
        0,
        map[0]
            .iter()
            .position(|t| *t == TileType::Path(None))
            .unwrap(),
    );
    let ending_pos = (
        map.len() - 1,
        map[map.len() - 1]
            .iter()
            .position(|t| *t == TileType::Path(None))
            .unwrap(),
    );

    let mut stack: Vec<((usize, usize), usize, Vec<((usize, usize), usize)>)> =
        vec![(starting_pos, 0, vec![])];
    let mut max_len = 0;

    while let Some((pos, steps, prev_pos_and_steps)) = stack.pop() {
        if pos == ending_pos {
            for &(prev_pos, prev_steps) in &prev_pos_and_steps {
                map[prev_pos.0][prev_pos.1] = TileType::Path(Some(prev_steps));
            }
            max_len = max_len.max(steps);
            continue;
        }
        for (dy, dx) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let new_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
            if prev_pos_and_steps
                .iter()
                .any(|(prev_pos, _)| *prev_pos == new_pos)
            {
                continue;
            }
            match map.get(new_pos.0).and_then(|row| row.get(new_pos.1)) {
                Some(TileType::Path(None)) => {
                    let mut next_prev_pos_and_steps = prev_pos_and_steps.clone();
                    next_prev_pos_and_steps.push((pos, steps));
                    stack.push((new_pos, steps + 1, next_prev_pos_and_steps));
                }
                Some(TileType::Path(Some(other_steps))) => {
                    // if steps + 1 > *other_steps {
                    let mut next_prev_pos_and_steps = prev_pos_and_steps.clone();
                    next_prev_pos_and_steps.push((pos, steps));
                    stack.push((new_pos, steps + 1, next_prev_pos_and_steps));
                    // }
                }
                Some(TileType::Wall) => {}
                None => {}
            }
        }
    }

    println!("{}", max_len);

    print_map(&map);
}

fn print_map(map: &Vec<Vec<TileType>>) {
    for row in map {
        for tile in row {
            print!(
                "{}\t",
                match tile {
                    TileType::Path(Some(steps)) => steps.to_string(),
                    TileType::Path(None) => ".".to_string(),
                    TileType::Wall => "#".to_string(),
                }
            );
        }
        println!();
    }
}

#[derive(PartialEq, Debug)]
enum TileType {
    Path(Option<usize>),
    Wall,
}
