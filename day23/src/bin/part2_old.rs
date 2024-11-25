use std::{collections::VecDeque, sync::atomic::AtomicUsize};

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

    map[starting_pos.0][starting_pos.1] = TileType::Path(Some(0));

    let mut stack = VecDeque::new();
    stack.push_back(starting_pos);

    while let Some(pos) = stack.pop_back() {
        if let TileType::Path(Some(steps)) = map[pos.0][pos.1] {
            if pos == ending_pos {
                continue;
            }
            for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
                match map.get(new_pos.0).and_then(|row| row.get(new_pos.1)) {
                    Some(TileType::Path(None)) => {
                        map[new_pos.0][new_pos.1] = TileType::Path(Some(steps + 1));
                        stack.push_back(new_pos);
                    }
                    Some(TileType::Path(Some(other_steps))) => {
                        if steps + 1 > *other_steps {
                            map[new_pos.0][new_pos.1] = TileType::Path(Some(steps + 1));
                            stack.push_back(new_pos);
                        }
                    }
                    Some(TileType::Wall) => {}
                    None => {}
                }
            }
        }
    }

    println!("{:?}", map[ending_pos.0][ending_pos.1]);
}

#[derive(PartialEq, Debug)]
enum TileType {
    Path(Option<usize>),
    Wall,
}
