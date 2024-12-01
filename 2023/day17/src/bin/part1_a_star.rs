use std::{fmt, cmp::Reverse};

fn main() {
    let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let input = include_str!("../../input.txt");
    let map: Map<usize> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    // println!("{}", map);

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: map.width - 1,
        y: map.height - 1,
    };

    // let (dist, from) = djikstra(&start, &map);
    // println!("{}", dist);
    // println!("{}", from);

    let cost = a_star(&start, &end, &map);
    println!("{}", cost);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::Up => write!(f, "^"),
            Dir::Down => write!(f, "v"),
            Dir::Left => write!(f, "<"),
            Dir::Right => write!(f, ">"),
            Dir::None => write!(f, " "),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

struct Map<T> {
    map: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    fn empty_like<NewT: Clone>(&self, val: NewT) -> Map<NewT> {
        Map {
            map: vec![vec![val; self.width]; self.height],
            width: self.width,
            height: self.height,
        }
    }

    fn get(&self, p: &Point) -> &T {
        &self.map[p.y][p.x]
    }

    fn set(&mut self, p: &Point, v: T) {
        self.map[p.y][p.x] = v
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

fn a_star(start: &Point, end: &Point, costs: &Map<usize>) -> usize {
    let mut queue = std::collections::BinaryHeap::new();
    let mut visited: Vec<(Point, Dir, Dir, Dir)> = vec![];
    queue.push((
        Reverse(heuristic(start, end)),
        0,
        Point {
            x: start.x,
            y: start.y,
        },
        Dir::None,
        Dir::None,
        Dir::None,
    ));

    loop {
        let (_, c, p, d0, d1, d2) = queue.pop().unwrap();

        if visited.contains(&(p, d0, d1, d2)) {
            continue;
        }
        visited.push((p.clone(), d0, d1, d2));

        // println!("{} {} {} {}", p.x, p.y, c, h);

        if p.x == end.x && p.y == end.y {
            break c;
        }

        // cannot go outside the map
        if p.x < costs.width - 1 && d2 != Dir::Left {
            // Cannot go 3 times in the same direction
            if d0 != Dir::Right || d1 != Dir::Right || d2 != Dir::Right {
                let new_p = Point { x: p.x + 1, y: p.y };
                let new_c = c + costs.get(&new_p);
                queue.push((
                    Reverse(new_c + heuristic(&new_p, end)),
                    new_c,
                    new_p,
                    d1,
                    d2,
                    Dir::Right,
                ))
            }
        }
        if p.x > 0 && d2 != Dir::Right {
            if d0 != Dir::Left || d1 != Dir::Left || d2 != Dir::Left {
                let new_p = Point { x: p.x - 1, y: p.y };
                let new_c = c + costs.get(&new_p);
                queue.push((
                    Reverse(new_c + heuristic(&new_p, end)),
                    new_c,
                    new_p,
                    d1,
                    d2,
                    Dir::Left,
                ))
            }
        }
        if p.y < costs.height - 1 && d2 != Dir::Up {
            if d0 != Dir::Down || d1 != Dir::Down || d2 != Dir::Down {
                let new_p = Point { x: p.x, y: p.y + 1 };
                let new_c = c + costs.get(&new_p);
                queue.push((
                    Reverse(new_c + heuristic(&new_p, end)),
                    new_c,
                    new_p,
                    d1,
                    d2,
                    Dir::Down,
                ))
            }
        }
        if p.y > 0 && d2 != Dir::Down {
            if d0 != Dir::Up || d1 != Dir::Up || d2 != Dir::Up {
                let new_p = Point { x: p.x, y: p.y - 1 };
                let new_c = c + costs.get(&new_p);
                queue.push((
                    Reverse(new_c + heuristic(&new_p, end)),
                    new_c,
                    new_p,
                    d1,
                    d2,
                    Dir::Up,
                ))
            }
        }
    }
}

fn heuristic(p: &Point, end: &Point) -> usize {
    (p.x as i32 - end.x as i32).abs() as usize + (p.y as i32 - end.y as i32).abs() as usize
}
