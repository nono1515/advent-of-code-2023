use std::{cmp::Reverse, fmt};

fn main() {
    let input = include_str!("../../input.txt");
    let map: Map<usize> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    const DMIN: usize = 4;
    const DMAX: usize = 10;

    let mut visited = vec![vec![vec![false; 4]; map.width]; map.height];
    let mut queue = std::collections::BinaryHeap::new();
    queue.push((
        Reverse(0),
        Node {
            x: 0,
            y: 0,
            dir: Dir::Down,
        },
    ));
    queue.push((
        Reverse(0),
        Node {
            x: 0,
            y: 0,
            dir: Dir::Right,
        },
    ));

    let mut c = 0;

    let cost = loop {
        if let Some((Reverse(cost), node)) = queue.pop() {
            if node.x == map.width - 1 && node.y == map.height - 1 {
                break Some(cost);
            }

            if visited[node.y][node.x][node.dir as usize] {
                continue;
            }
            visited[node.y][node.x][node.dir as usize] = true;

            match node.dir {
                Dir::Down | Dir::Up => {
                    for right in [true, false] {
                        let mut cost_sum = cost;
                        for d in 1..=DMAX {
                            if d > node.x && right == false {
                                break;
                            }
                            if let Some(cost) =
                                map.get(if right { node.x + d } else { node.x - d }, node.y)
                            {
                                cost_sum += cost;
                                if d >= DMIN {
                                    queue.push((
                                        Reverse(cost_sum),
                                        Node {
                                            x: if right { node.x + d } else { node.x - d },
                                            y: node.y,
                                            dir: if right { Dir::Right } else { Dir::Left },
                                        },
                                    ))
                                }
                            }
                        }
                    }
                }
                Dir::Right | Dir::Left => {
                    for down in [true, false] {
                        let mut cost_sum = cost;
                        for d in 1..=DMAX {
                            if d > node.y && down == false {
                                break;
                            }
                            if let Some(cost) =
                                map.get(node.x, if down { node.y + d } else { node.y - d })
                            {
                                cost_sum += cost;
                                if d >= DMIN {
                                    queue.push((
                                        Reverse(cost_sum),
                                        Node {
                                            x: node.x,
                                            y: if down { node.y + d } else { node.y - d },
                                            dir: if down { Dir::Down } else { Dir::Up },
                                        },
                                    ))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            break None;
        }
    };

    println!("{:?}", cost);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Dir {
    Up = 0,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    x: usize,
    y: usize,
    dir: Dir,
}

struct Map<T> {
    map: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if y >= self.height {
            None
        } else {
            self.map.get(y).unwrap().get(x)
        }
    }
}

impl<T: fmt::Display> fmt::Display for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.map {
            for e in row {
                write!(f, "{:>2} ", e)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T> FromIterator<Vec<T>> for Map<T> {
    fn from_iter<TIter: IntoIterator<Item = Vec<T>>>(iter: TIter) -> Self {
        let map: Vec<Vec<T>> = iter.into_iter().collect();
        let width = map[0].len();
        let height = map.len();
        Map { map, width, height }
    }
}
