#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Player {
    pos: Pos,
    dir: Direction,
}

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,
    obstacles: Vec<Pos>,
}

fn update_visited(mut visited: Vec<Pos>, player: &Player, closest_obstacle: &Pos) -> Vec<Pos> {
    match player.dir {
        Direction::Up => {
            (closest_obstacle.y + 1..=player.pos.y).for_each(|y| {
                if !visited.contains(&Pos { x: player.pos.x, y }) {
                    visited.push(Pos { x: player.pos.x, y });
                }
            });
        }
        Direction::Down => {
            (player.pos.y..closest_obstacle.y).for_each(|y| {
                if !visited.contains(&Pos { x: player.pos.x, y }) {
                    visited.push(Pos { x: player.pos.x, y });
                }
            });
        }
        Direction::Right => {
            (player.pos.x..closest_obstacle.x).for_each(|x| {
                if !visited.contains(&Pos { x, y: player.pos.y }) {
                    visited.push(Pos { x, y: player.pos.y });
                }
            });
        }
        Direction::Left => {
            (closest_obstacle.x + 1..=player.pos.x).for_each(|x| {
                if !visited.contains(&Pos { x, y: player.pos.y }) {
                    visited.push(Pos { x, y: player.pos.y });
                }
            });
        }
    }

    visited
}

fn update_visited_with_dir(
    mut visited: Vec<(Pos, Direction)>,
    player: &Player,
    closest_obstacle: &Pos,
) -> Vec<(Pos, Direction)> {
    match player.dir {
        Direction::Up => {
            (closest_obstacle.y + 1..=player.pos.y).for_each(|y| {
                if !visited.contains(&(Pos { x: player.pos.x, y }, player.dir)) {
                    visited.push((Pos { x: player.pos.x, y }, player.dir));
                }
            });
        }
        Direction::Down => {
            (player.pos.y..closest_obstacle.y).for_each(|y| {
                if !visited.contains(&(Pos { x: player.pos.x, y }, player.dir)) {
                    visited.push((Pos { x: player.pos.x, y }, player.dir));
                }
            });
        }
        Direction::Right => {
            (player.pos.x..closest_obstacle.x).for_each(|x| {
                if !visited.contains(&(Pos { x, y: player.pos.y }, player.dir)) {
                    visited.push((Pos { x, y: player.pos.y }, player.dir));
                }
            });
        }
        Direction::Left => {
            (closest_obstacle.x + 1..=player.pos.x).for_each(|x| {
                if !visited.contains(&(Pos { x, y: player.pos.y }, player.dir)) {
                    visited.push((Pos { x, y: player.pos.y }, player.dir));
                }
            });
        }
    }

    visited
}

fn part1(player: &Player, map: &Map) -> usize {
    let mut visited = vec![];
    let mut player = player.clone();

    loop {
        // print the map for debug purposes
        // println!();
        // println!("{}", visited.len());
        // for y in 0..map.height {
        //     for x in 0..map.width {
        //         if visited.contains(&Pos { x, y }) {
        //             print!("X");
        //         } else if map.obstacles.contains(&Pos { x, y }) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        let closest_obstacle = map
            .obstacles
            .iter()
            .filter(|obstacle| match player.dir {
                Direction::Up => obstacle.x == player.pos.x && obstacle.y < player.pos.y,
                Direction::Down => obstacle.x == player.pos.x && obstacle.y > player.pos.y,
                Direction::Right => obstacle.y == player.pos.y && obstacle.x > player.pos.x,
                Direction::Left => obstacle.y == player.pos.y && obstacle.x < player.pos.x,
            })
            .min_by_key(|obstacle| {
                (obstacle.x - player.pos.x).abs() + (obstacle.y - player.pos.y).abs()
            });
        if let Some(closest_obstacle) = closest_obstacle {
            match player.dir {
                Direction::Up => {
                    visited = update_visited(visited, &player, closest_obstacle);
                    player.pos.y = closest_obstacle.y + 1;
                    player.dir = Direction::Right;
                }
                Direction::Down => {
                    visited = update_visited(visited, &player, closest_obstacle);
                    player.pos.y = closest_obstacle.y - 1;
                    player.dir = Direction::Left;
                }
                Direction::Right => {
                    visited = update_visited(visited, &player, closest_obstacle);
                    player.pos.x = closest_obstacle.x - 1;
                    player.dir = Direction::Down;
                }
                Direction::Left => {
                    visited = update_visited(visited, &player, closest_obstacle);
                    player.pos.x = closest_obstacle.x + 1;
                    player.dir = Direction::Up;
                }
            }
        } else {
            let map_lim = match player.dir {
                Direction::Up => Pos {
                    x: player.pos.x,
                    y: -1,
                },
                Direction::Down => Pos {
                    x: player.pos.x,
                    y: map.height,
                },
                Direction::Right => Pos {
                    x: map.width,
                    y: player.pos.y,
                },
                Direction::Left => Pos {
                    x: -1,
                    y: player.pos.y,
                },
            };
            visited = update_visited(visited, &player, &map_lim);
            break;
        }
    }

    visited.len()
}

fn is_loop(player: &Player, map: &Map, visited: &Vec<(Pos, Direction)>) -> bool {
    let mut visited = visited.clone();
    let mut player = player.clone();

    loop {
        let closest_obstacle = map
            .obstacles
            .iter()
            .filter(|obstacle| match player.dir {
                Direction::Up => obstacle.x == player.pos.x && obstacle.y < player.pos.y,
                Direction::Down => obstacle.x == player.pos.x && obstacle.y > player.pos.y,
                Direction::Right => obstacle.y == player.pos.y && obstacle.x > player.pos.x,
                Direction::Left => obstacle.y == player.pos.y && obstacle.x < player.pos.x,
            })
            .min_by_key(|obstacle| {
                (obstacle.x - player.pos.x).abs() + (obstacle.y - player.pos.y).abs()
            });
        if let Some(closest_obstacle) = closest_obstacle {
            match player.dir {
                Direction::Up => {
                    visited = update_visited_with_dir(visited, &player, closest_obstacle);
                    player.pos.y = closest_obstacle.y + 1;
                    player.dir = Direction::Right;
                }
                Direction::Down => {
                    visited = update_visited_with_dir(visited, &player, closest_obstacle);
                    player.pos.y = closest_obstacle.y - 1;
                    player.dir = Direction::Left;
                }
                Direction::Right => {
                    visited = update_visited_with_dir(visited, &player, closest_obstacle);
                    player.pos.x = closest_obstacle.x - 1;
                    player.dir = Direction::Down;
                }
                Direction::Left => {
                    visited = update_visited_with_dir(visited, &player, closest_obstacle);
                    player.pos.x = closest_obstacle.x + 1;
                    player.dir = Direction::Up;
                }
            }
            if visited.contains(&(player.pos, player.dir)) {
                return true;
            }
        } else {
            return false;
        }
    }
}

fn part2(player: &Player, map: &Map) -> u32 {
    let mut total = 0;

    let mut visited = vec![];
    let mut player = player.clone();

    loop {
        visited.push((player.pos, player.dir));
        let next_pos = match player.dir {
            Direction::Up => Pos {
                x: player.pos.x,
                y: player.pos.y - 1,
            },
            Direction::Down => Pos {
                x: player.pos.x,
                y: player.pos.y + 1,
            },
            Direction::Right => Pos {
                x: player.pos.x + 1,
                y: player.pos.y,
            },
            Direction::Left => Pos {
                x: player.pos.x - 1,
                y: player.pos.y,
            },
        };
        if next_pos.x < 0 || next_pos.x >= map.width || next_pos.y < 0 || next_pos.y >= map.height {
            break;
        }

        if map.obstacles.contains(&next_pos) {
            player.dir = match player.dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            if !visited.iter().any(|(pos, _)| *pos == next_pos) {
                let mut new_map = map.clone();
                new_map.obstacles.push(next_pos);
                if is_loop(&player, &new_map, &visited) {
                    total += 1;
                }
            }
            player.pos = next_pos;
        }
    }

    total
}

fn main() {
    let input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let input = include_str!("../input.txt");

    let mut obstacles = vec![];
    let mut player = Player {
        pos: Pos { x: 0, y: 0 },
        dir: Direction::Up,
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.push(Pos {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c == '^' {
                player.pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

    let map = Map {
        width: input.lines().next().unwrap().len() as i32,
        height: input.lines().count() as i32,
        obstacles,
    };

    println!("Part 1: {}", part1(&player, &map));
    println!("Part 2: {}", part2(&player, &map));
}
