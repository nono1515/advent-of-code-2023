use std::fmt;

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

    // let input = include_str!("../../input.txt");
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

    let (dist, from) = djikstra(&start, &map);

    println!("{}", dist);
    println!("{}", from);
    // println!("{}", path.get(&end));
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

fn djikstra(start: &Point, costs: &Map<usize>) -> (Map<usize>, Map<Dir>) {
    let mut dist = costs.empty_like(usize::MAX);
    dist.set(start, 0);
    let mut from = costs.empty_like(Dir::None);

    for y in 0..costs.height {
        for x in 0..costs.width {
            let d = dist.get(&Point { x, y }).clone();
            if x < costs.width - 1 {
                let cost_right = d + costs.get(&Point { x: x + 1, y });
                if cost_right < *dist.get(&Point { x: x + 1, y }) {
                    dist.set(&Point { x: x + 1, y }, cost_right);
                    from.set(&Point { x: x + 1, y }, Dir::Left);
                }
            }
            if x > 0 {
                let cost_left = d + costs.get(&Point { x: x - 1, y });
                if cost_left < *dist.get(&Point { x: x - 1, y }) {
                    dist.set(&Point { x: x - 1, y }, cost_left);
                    from.set(&Point { x: x - 1, y }, Dir::Right);
                }
            }
            if y < costs.height - 1 {
                let cost_down = d + costs.get(&Point { x, y: y + 1 });
                if cost_down < *dist.get(&Point { x, y: y + 1 }) {
                    dist.set(&Point { x, y: y + 1 }, cost_down);
                    from.set(&Point { x, y: y + 1 }, Dir::Up);
                }
            }
            if y > 0 {
                let cost_up = d + costs.get(&Point { x, y: y - 1 });
                if cost_up < *dist.get(&Point { x, y: y - 1 }) {
                    dist.set(&Point { x, y: y - 1 }, cost_up);
                    from.set(&Point { x, y: y - 1 }, Dir::Down);
                }
            }
        }
    }

    (dist, from)
}

fn a_star(start: &Point, end: &Point, costs: &Map<usize>) -> (Map<usize>, Map<Dir>) {
    let mut dist = costs.empty_like(usize::MAX);
    dist.set(start, 0);
    let mut from = costs.empty_like(Dir::None);

    // let mut visited = vec![];
    let mut queue = vec![];
    queue.push((heuristic(&0, start, end), start));

    loop {
        queue.sort();
        queue.reverse();
        let (c, p) = queue.pop().unwrap();
        
    }

    (dist, from)
}

fn heuristic(cost: &usize, p: &Point, end: &Point) -> usize {
    *cost + (p.x as i32 - end.x as i32).abs() as usize + (p.y as i32 - end.y as i32).abs() as usize
}
