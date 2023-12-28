use std::{cmp::Reverse, fmt};

pub fn dijkstra(map: &Map<usize>, dmin: usize, dmax: usize) -> Option<usize> {
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

    loop {
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
                        for d in 1..=dmax {
                            if d > node.x && right == false {
                                break;
                            }
                            if let Some(cost) =
                                map.get(if right { node.x + d } else { node.x - d }, node.y)
                            {
                                cost_sum += cost;
                                if d >= dmin {
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
                        for d in 1..=dmax {
                            if d > node.y && down == false {
                                break;
                            }
                            if let Some(cost) =
                                map.get(node.x, if down { node.y + d } else { node.y - d })
                            {
                                cost_sum += cost;
                                if d >= dmin {
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
    }
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

pub struct Map<T> {
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
