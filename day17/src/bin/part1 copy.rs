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

    let end_points = (map[0].len() as u32, map.len() as u32);
    let mut points = vec![(
        0,
        heuristic(&0, &(0, 0), &end_points),
        0,
        0,
        Dir::None,
        Dir::None,
        Dir::None,
    )];
    let mut visited = vec![];

    let cost = loop {
        points.sort_by(|p, q| p.0.cmp(&q.0).reverse());
        // println!("{:?}", points);
        let p = points.pop().unwrap();
        visited.push(p);
        println!("{}, {}", p.0, p.1);
        println!("{}, {}", p.2, p.3);

        if p.3 > 0 && (p.4 != Dir::Up || p.5 != Dir::Up || p.6 != Dir::Up) {
            let new_point = (p.2, p.3 - 1);
            let cost = p.0 + map[new_point.0 as usize][new_point.1 as usize] as u32;
            if new_point == end_points {
                break cost;
            }
            let new_point = (
                cost,
                heuristic(&cost, &new_point, &end_points),
                new_point.0,
                new_point.1,
                p.5,
                p.6,
                Dir::Up,
            );
            if !visited.contains(&new_point) {
                points.push(new_point);
            }
        }
        if p.2 > 0 && (p.4 != Dir::Left || p.5 != Dir::Left || p.6 != Dir::Left) {
            let new_point = (p.2 - 1, p.3);
            let cost = p.0 + map[new_point.0 as usize][new_point.1 as usize] as u32;
            if new_point == end_points {
                break cost;
            }
            let new_point = (
                cost,
                heuristic(&cost, &new_point, &end_points),
                new_point.0,
                new_point.1,
                p.5,
                p.6,
                Dir::Left,
            );
            if !visited.contains(&new_point) {
                points.push(new_point);
            }
        }
        if p.3 < map.len() as u32 - 1 && (p.4 != Dir::Down || p.5 != Dir::Down || p.6 != Dir::Down)
        {
            let new_point = (p.2, p.3 + 1);
            let cost = p.0 + map[new_point.0 as usize][new_point.1 as usize] as u32;
            if new_point == end_points {
                break cost;
            }
            let new_point = (
                cost,
                heuristic(&cost, &new_point, &end_points),
                new_point.0,
                new_point.1,
                p.5,

                p.6,
                Dir::Down,
            );
            if !visited.contains(&new_point) {
                points.push(new_point);
            }
        }
        if p.2 < map[0].len() as u32 - 1 && (p.4 != Dir::Right || p.5 != Dir::Right || p.6 != Dir::Right)
        {
            let new_point = (p.2 + 1, p.3);
            let cost = p.0 + map[new_point.0 as usize][new_point.1 as usize] as u32;
            if new_point == end_points {
                break cost;
            }
            let new_point = (
                cost,
                heuristic(&cost, &new_point, &end_points),
                new_point.0,
                new_point.1,
                p.5,

                p.6,
                Dir::Right,
            );
            if !visited.contains(&new_point) {
                points.push(new_point);
            }
        }
    };

    println!("{}", cost);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn heuristic(cost: &u32, (x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> u32 {
    cost + (*x1 as i32 - *x2 as i32).abs() as u32 + (*y1 as i32 - *y2 as i32).abs() as u32 
}
