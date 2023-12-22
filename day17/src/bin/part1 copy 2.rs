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
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: map[0].len() as u32 - 1,
        y: map.len() as u32 - 1,
    };

    let mut paths = vec![Path {
        cost: 0,
        heuristic: heuristic(&(start.x, start.y), &(end.x, end.y)),
        points: vec![start],
    }];

    let cost = loop {
        paths.sort_by(|a, b| a.heuristic.cmp(&b.heuristic).reverse());
        let path = paths.pop();
        let p = path


        break 0 ;
    };
}

struct Point<T> {
    x: T,
    y: T,
}

struct Path<T> {
    cost: u32,
    heuristic: u32,
    points: Vec<Point<T>>,
}

fn heuristic((x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> u32 {
    (*x1 as i32 - *x2 as i32).abs() as u32 + (*y1 as i32 - *y2 as i32).abs() as u32
}
