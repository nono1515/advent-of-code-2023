use std::{time::Instant, usize};

#[derive(PartialEq)]
enum Tile {
    Wall,
    Empty,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
}

fn parse_input(input: &str) -> (Pos, Pos, Vec<Vec<Tile>>) {
    let mut start_pos = Pos::new(0, 0);
    let mut end_pos = Pos::new(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    'S' => {
                        start_pos = Pos { x, y };
                        Tile::Empty
                    }
                    'E' => {
                        end_pos = Pos { x, y };
                        Tile::Empty
                    }
                    _ => Tile::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (start_pos, end_pos, map)
}

fn part1(input: &str) -> usize {
    let (s, e, map) = parse_input(input);

    let mut min_cost = usize::MAX;
    let mut queue = vec![(s, 0, Direction::Right, vec![])];

    while let Some((pos, cost, prev_dir, path)) = queue.pop() {
        if cost > min_cost {
            continue;
        }

        if pos == e {
            min_cost = min_cost.min(cost);
            continue;
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let mut new_pos = pos.clone();
            new_pos.move_dir(&dir);

            if path.contains(&new_pos) {
                continue;
            }

            if map[new_pos.y][new_pos.x] == Tile::Empty {
                let mut new_path = path.clone();
                new_path.push(pos.clone());
                if dir != prev_dir {
                    queue.push((new_pos, cost + 1001, dir, new_path));
                } else {
                    queue.push((new_pos, cost + 1, dir, new_path));
                }
            }
        }
    }

    min_cost
}

fn part2(input: &str) -> usize {
    0
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
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    assert_eq!(part1(&input), 7036);

    // assert_eq!(part2(&input), );
}

#[test]
fn test_small() {
    let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    assert_eq!(part1(&input), 11048);

    // assert_eq!(part2(&input, 11, 7), 0);
}
