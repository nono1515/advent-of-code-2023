fn part1(input: &str) -> u64 {
    let corners = parse_input(&input);
    (0..corners.len())
        .flat_map(|i| {
            let (x1, y1) = corners[i];
            ((i + 1)..corners.len())
                .map(|j| {
                    let (x2, y2) = corners[j];
                    (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1)
                })
                .max()
        })
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq)]
enum WallDirection {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Wall {
    /// ┌
    UpperLeft,
    /// ┐
    UpperRight,
    /// └
    LowerLeft,
    /// ┘
    LowerRight,
    /// |
    Vertical,
    /// -
    Horizontal,
    /// .
    Empty,
}

fn part2(input: &str) -> u64 {
    let corners = parse_input(input);
    let xsize = corners.iter().map(|(x, _)| *x).max().unwrap() as usize;
    let ysize = corners.iter().map(|(_, y)| *y).max().unwrap() as usize;

    let mut borders = vec![vec![Wall::Empty; xsize + 1]; ysize + 1];
    for ((c1, c2), c3) in corners
        .iter()
        .zip(corners.iter().cycle().skip(1))
        .zip(corners.iter().cycle().skip(2))
    {
        let (x1, y1) = (c1.0 as i32, c1.1 as i32);
        let (x2, y2) = (c2.0 as i32, c2.1 as i32);
        let (x3, y3) = (c3.0 as i32, c3.1 as i32);
        let from_prev = (y2 - y1, x2 - x1);
        let to_next = (y3 - y2, x3 - x2);
        let corner_type = match (from_prev, to_next) {
            ((0, dx21), (dy32, 0)) => match (dx21, dy32) {
                (..0, ..0) => Wall::LowerLeft,
                (..0, 0..) => Wall::UpperLeft,
                (0.., ..0) => Wall::LowerRight,
                (0.., 0..) => Wall::UpperRight,
            },
            ((dy21, 0), (0, dx32)) => match (dy21, dx32) {
                (..0, ..0) => Wall::UpperRight,
                (..0, 0..) => Wall::UpperLeft,
                (0.., ..0) => Wall::LowerRight,
                (0.., 0..) => Wall::LowerLeft,
            },
            _ => unreachable!(),
        };
        borders[c2.1 as usize][c2.0 as usize] = corner_type;
        match to_next {
            (0, _) => {
                for x in x2.min(x3)..=x2.max(x3) {
                    borders[y2 as usize][x as usize] = Wall::Horizontal;
                }
            }
            (_, 0) => {
                for y in y2.min(y3)..=y2.max(y3) {
                    borders[y as usize][x2 as usize] = Wall::Vertical;
                }
            }
            _ => unreachable!(),
        };
    }

    // for l in &borders {
    //     for e in l {
    //         let tile = match e {
    //             Wall::UpperLeft => '┌',
    //             Wall::UpperRight => '┐',
    //             Wall::LowerLeft => '└',
    //             Wall::LowerRight => '┘',
    //             Wall::Vertical => '|',
    //             Wall::Horizontal => '-',
    //             Wall::Empty => '.',
    //         };
    //         print!("{tile}");
    //     }
    //     println!();
    // }
    // println!();

    let matrix = (0..=ysize)
        .map(|y| {
            (0..=xsize)
                .scan((false, None), |(is_inside, wall_dir), x| {
                    // update_state(is_inside, wall_dir, x, y, &corners);
                    match borders[y][x] {
                        Wall::Vertical => *is_inside = !*is_inside,
                        Wall::LowerLeft => *wall_dir = Some(WallDirection::Up),
                        Wall::UpperLeft => *wall_dir = Some(WallDirection::Down),
                        Wall::LowerRight => {
                            if let Some(dir) = wall_dir {
                                if *dir == WallDirection::Down {
                                    *is_inside = !*is_inside;
                                }
                            }
                            *wall_dir = None;
                        }
                        Wall::UpperRight => {
                            if let Some(dir) = wall_dir {
                                if *dir == WallDirection::Up {
                                    *is_inside = !*is_inside;
                                }
                            }
                            *wall_dir = None;
                        }
                        Wall::Horizontal | Wall::Empty => (),
                    }
                    // println!("{x}, {y}, {is_inside}, {wall_dir:?}");
                    Some(*is_inside || borders[y][x] != Wall::Empty)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Filling done");
    // for l in &matrix {
    //     for e in l {
    //         if *e {
    //             print!("#");
    //         } else {
    //             print! {"."};
    //         }
    //     }
    //     println!();
    // }

    (0..corners.len())
        .flat_map(|i| {
            let (x1, y1) = corners[i];
            ((i + 1)..corners.len())
                .filter_map(|j| {
                    let (x2, y2) = corners[j];
                    if (x1.min(x2)..=x1.max(x2))
                        .all(|x| matrix[y1 as usize][x as usize] && matrix[y2 as usize][x as usize])
                        && (y1.min(y2)..=y1.max(y2)).all(|y| {
                            matrix[y as usize][x1 as usize] && matrix[y as usize][x2 as usize]
                        })
                    {
                        Some((x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y1.min(y2) + 1))
                    } else {
                        None
                    }
                })
                .max()
        })
        .max()
        .unwrap()
}

fn update_state(
    is_inside: &mut bool,
    wall_dir: &mut Option<WallDirection>,
    x: usize,
    y: usize,
    corners: &Vec<(u64, u64)>,
) {
    let (x, y) = (x as u64, y as u64);
    for (c1, c2) in corners.iter().zip(corners.iter().cycle().skip(1)) {
        if let Some(dir) = wall_dir {
            if *c1 == (x, y) && c2.1 != y {
                if (*dir == WallDirection::Up && c2.1 > y)  // the previous wall is upward
                    || (*dir == WallDirection::Down && c2.1 < y)
                {
                    *is_inside = !*is_inside;
                }
                *wall_dir = None;
                break;
            }
            if *c2 == (x, y) && c1.1 != y {
                if (*dir == WallDirection::Up && c1.1 > y)  // the previous wall is upward
                    || (*dir == WallDirection::Down && c1.1 < y)
                {
                    *is_inside = !*is_inside;
                }
                *wall_dir = None;
                break;
            }
        } else {
            if *c1 == (x, y) {
                if c2.1 < y {
                    *wall_dir = Some(WallDirection::Up);
                } else if c2.1 > y {
                    *wall_dir = Some(WallDirection::Down);
                } else {
                    continue;
                }
                break;
            } else if *c2 == (x, y) {
                if c1.1 < y {
                    *wall_dir = Some(WallDirection::Up);
                } else if c1.1 > y {
                    *wall_dir = Some(WallDirection::Down);
                } else {
                    continue;
                }
                break;
            } else if (c1.0.min(c2.0)..=c1.0.max(c2.0)).contains(&x)
                && (c1.1.min(c2.1)..=c1.1.max(c2.1)).contains(&y)
            {
                *is_inside = !*is_inside;
                break;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_both_parts() {
    let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    assert_eq!(part1(&input), 50);
    assert_eq!(part2(&input), 24);
}
