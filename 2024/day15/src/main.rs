use std::{fmt::Debug, time::Instant};

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Wall,
    Box,
    DoubleBox(u16),
    Empty,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Box => write!(f, "O"),
            Tile::DoubleBox(id) => write!(f, "{}", id % 10),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn move_back(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }
}

fn print_map(pos: &Pos, map: &Vec<Vec<Tile>>) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if x == pos.x && y == pos.y {
                print!("@");
            } else {
                print!("{:?}", tile);
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> (Pos, Vec<Vec<Tile>>, Vec<Direction>) {
    let (map_str, commands_str) = input.split_once("\n\n").unwrap();
    let mut pos = Pos::new(0, 0);
    let map = map_str
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => {
                        pos = Pos { x, y };
                        Tile::Empty
                    }
                    _ => Tile::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let commands = commands_str
        .chars()
        .filter(|c| ['^', 'v', '<', '>'].contains(c))
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();

    (pos, map, commands)
}

fn move_robot(pos: &mut Pos, map: &mut Vec<Vec<Tile>>, dir: Direction) {
    pos.move_dir(&dir);
    match map[pos.y][pos.x] {
        Tile::Wall => pos.move_back(&dir),
        Tile::Box => {
            let mut final_pos = pos.clone();
            while map[final_pos.y][final_pos.x] == Tile::Box {
                final_pos.move_dir(&dir);
            }

            if map[final_pos.y][final_pos.x] == Tile::Empty {
                map[final_pos.y][final_pos.x] = Tile::Box;
                map[pos.y][pos.x] = Tile::Empty;
            } else if map[final_pos.y][final_pos.x] == Tile::Wall {
                pos.move_back(&dir);
            } else {
                panic!("Invalid state");
            }
        }
        Tile::DoubleBox(id) => {
            let mut final_pos = pos.clone();
            while let Tile::DoubleBox(_) = map[final_pos.y][final_pos.x] {
                final_pos.move_dir(&dir);
            }

            if map[final_pos.y][final_pos.x] == Tile::Empty {
                while &final_pos != pos {
                    let mut next_post = final_pos.clone();
                    next_post.move_back(&dir);
                    map[final_pos.y][final_pos.x] = map[next_post.y][next_post.x];
                    final_pos = next_post;
                }
                map[pos.y][pos.x] = Tile::Empty;
            } else if map[final_pos.y][final_pos.x] == Tile::Wall {
                pos.move_back(&dir);
            } else {
                panic!("Invalid state");
            }
        }
        Tile::Empty => (),
    }
}

fn find_pair(pos: &Pos, id: u16, map: &Vec<Vec<Tile>>) -> (Pos, u16) {
    let mut out_pos = pos.clone();
    if map[pos.y][pos.x + 1] == Tile::DoubleBox(id) {
        out_pos.move_dir(&Direction::Right);
        (out_pos, id)
    } else if map[pos.y][pos.x - 1] == Tile::DoubleBox(id) {
        out_pos.move_dir(&Direction::Left);
        (out_pos, id)
    } else {
        panic!("No pair found")
    }
}

fn move_robot_double_lr(pos: &mut Pos, map: &mut Vec<Vec<Tile>>, dir: Direction) {
    if dir == Direction::Left || dir == Direction::Right {
        move_robot(pos, map, dir);
    } else {
        pos.move_dir(&dir);

        match map[pos.y][pos.x] {
            Tile::Wall => pos.move_back(&dir),
            Tile::DoubleBox(id) => {
                let pair = find_pair(pos, id, map);
                let mut just_added = vec![(pos.clone(), id), pair];
                let mut to_move = vec![];

                let ok_to_move = loop {
                    just_added.iter_mut().for_each(|(pos, _id)| {
                        pos.move_dir(&dir);
                    });

                    if just_added
                        .iter()
                        .all(|(pos, _)| map[pos.y][pos.x] == Tile::Empty)
                    {
                        to_move.append(&mut just_added);
                        break true;
                    }
                    if just_added
                        .iter()
                        .any(|(pos, _)| map[pos.y][pos.x] == Tile::Wall)
                    {
                        break false;
                    }

                    let mut next_to_add = vec![];
                    just_added.iter().for_each(|(pos, _id)| {
                        if let Tile::DoubleBox(next_id) = map[pos.y][pos.x] {
                            next_to_add.push((pos.clone(), next_id));
                            next_to_add.push(find_pair(&pos, next_id, &map));
                        }
                    });
                    to_move.append(&mut just_added);
                    just_added = next_to_add;
                };

                if ok_to_move {
                    while let Some((mut pos, id)) = to_move.pop() {
                        map[pos.y][pos.x] = Tile::DoubleBox(id);
                        pos.move_back(&dir);
                        map[pos.y][pos.x] = Tile::Empty;
                    }
                } else {
                    pos.move_back(&dir);
                }
            }
            Tile::Empty => (),
            _ => panic!("There shouldnt be any single box"),
        }
    }
}

fn part1(input: &str) -> usize {
    let (mut pos, mut map, commands) = parse_input(input);

    for c in commands {
        move_robot(&mut pos, &mut map, c);
    }

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| **t == Tile::Box)
                .map(|(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let (mut pos, map, commands) = parse_input(input);
    pos.x = 2 * pos.x;
    let mut id = 0;
    let mut map = map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|&tile| match tile {
                    Tile::Box => {
                        id += 1;
                        std::iter::repeat(Tile::DoubleBox(id)).take(2)
                    }
                    _ => std::iter::repeat(tile).take(2),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for c in commands {
        move_robot_double_lr(&mut pos, &mut map, c);
        // println!("{:?}", c);
        // print_map(&pos, &map);
        // println!("");
    }

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| matches!(**t, Tile::DoubleBox(_)))
                .step_by(2)
                .map(|(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input), now.elapsed());

    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(&input), now.elapsed());
}

#[test]
fn test_large() {
    let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    assert_eq!(part1(&input), 10092);

    assert_eq!(part2(&input), 9021);
}

#[test]
fn test_small() {
    let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    assert_eq!(part1(&input), 2028);

    // assert_eq!(part2(&input, 11, 7), 0);
}
