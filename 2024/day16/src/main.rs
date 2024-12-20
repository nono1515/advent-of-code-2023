use std::{time::Instant, usize, vec};

#[derive(PartialEq)]
enum Tile {
    Wall,
    Empty {
        up: usize,
        down: usize,
        left: usize,
        right: usize,
    },
}

impl Tile {
    fn new_empty() -> Self {
        Tile::Empty {
            up: usize::MAX,
            down: usize::MAX,
            left: usize::MAX,
            right: usize::MAX,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
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
                        Tile::new_empty()
                    }
                    'E' => {
                        end_pos = Pos { x, y };
                        Tile::new_empty()
                    }
                    _ => Tile::new_empty(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (start_pos, end_pos, map)
}

fn part1(input: &str) -> usize {
    let (s, e, mut map) = parse_input(input);

    let mut queue = vec![(s, 0, Direction::Right)];
    let mut min_cost = usize::MAX;

    while let Some((pos, cost, prev_dir)) = queue.pop() {
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

            let new_cost = {
                if prev_dir == dir {
                    cost + 1
                } else {
                    cost + 1001
                }
            };

            if let Tile::Empty {
                up,
                down,
                left,
                right,
            } = &mut map[new_pos.y][new_pos.x]
            {
                if dir == Direction::Up && *up > new_cost {
                    *up = new_cost;
                    queue.push((new_pos, new_cost, dir));
                } else if dir == Direction::Down && *down > new_cost {
                    *down = new_cost;
                    queue.push((new_pos, new_cost, dir));
                } else if dir == Direction::Left && *left > new_cost {
                    *left = new_cost;
                    queue.push((new_pos, new_cost, dir));
                } else if dir == Direction::Right && *right > new_cost {
                    *right = new_cost;
                    queue.push((new_pos, new_cost, dir));
                }
            }
        }
    }

    min_cost
}

fn part2(input: &str) -> usize {
    let (s, e, mut map) = parse_input(input);

    let mut queue = vec![(s.clone(), 0, Direction::Right)];
    let mut min_cost = usize::MAX;

    while let Some((pos, cost, prev_dir)) = queue.pop() {
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

            let new_cost = {
                if prev_dir == dir {
                    cost + 1
                } else {
                    cost + 1001
                }
            };

            if let Tile::Empty {
                up,
                down,
                left,
                right,
            } = &mut map[new_pos.y][new_pos.x]
            {
                if dir == Direction::Up && *up > new_cost {
                    *up = new_cost;
                    queue.push((new_pos, new_cost, dir));
                } else if dir == Direction::Down && *down > new_cost {
                    *down = new_cost;
                    queue.push((new_pos, new_cost, dir));
                } else if dir == Direction::Left && *left > new_cost {
                    *left = new_cost;
                    queue.push((new_pos, new_cost, dir));
                } else if dir == Direction::Right && *right > new_cost {
                    *right = new_cost;
                    queue.push((new_pos, new_cost, dir));
                }
            }
        }
    }

    let mut queue = vec![(e, None)];
    let mut visited = vec![vec![[false; 4]; map[0].len()]; map.len()];
    visited[s.y][s.x] = [true; 4];

    while let Some((pos, next_dir)) = queue.pop() {
        // visited.push((pos.clone(), next_dir.clone()));
        visited[pos.y][pos.x][next_dir.clone().unwrap_or(Direction::Up) as usize] = true;
        if let Tile::Empty {
            up,
            down,
            left,
            right,
        } = map[pos.y][pos.x]
        {
            [
                (Direction::Up, up),
                (Direction::Down, down),
                (Direction::Left, left),
                (Direction::Right, right),
            ]
            .iter()
            .fold((Vec::new(), 0), |(mut dirs, min_cost), (dir, c)| {
                // println!("{}", c);
                let c = if let Some(next_dir) = &next_dir {
                    if *dir != *next_dir {
                        c.saturating_add(1000)
                    } else {
                        *c
                    }
                } else {
                    *c
                };
                if dirs.is_empty() || c < min_cost {
                    dirs.clear();
                    dirs.push(dir.clone());
                    (dirs, c)
                } else if min_cost == c {
                    dirs.push(dir.clone());
                    (dirs, c)
                } else {
                    (dirs, min_cost)
                }
            })
            .0
            .iter()
            .for_each(|dir| {
                let mut new_pos = pos.clone();
                new_pos.move_back(&dir);
                if !visited[new_pos.y][new_pos.x][dir.clone() as usize] {
                    queue.push((new_pos, Some(dir.clone())));
                }
            });
        }
    }
    // for line in visited.iter() {
    //     for b in line.iter() {
    //         print!("{}", b.iter().any(|b| *b).then(|| "O").unwrap_or("."));
    //     }
    //     println!();
    // }

    visited
        .iter()
        .flatten()
        .filter(|b: &&[bool; 4]| b.iter().any(|b| *b))
        .count()
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

    assert_eq!(part2(&input), 45);
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

    assert_eq!(part2(&input), 64);
}
