use day17::{dijkstra, Map};

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

    const DMIN: usize = 1;
    const DMAX: usize = 3;

    let cost = dijkstra(&map, DMIN, DMAX);

    println!("{:?}", cost);
}
