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

fn part2(input: &str) -> u64 {
    let corners = parse_input(input);
    let xsize = corners.iter().map(|(x, _)| *x).max().unwrap() as usize;
    let ysize = corners.iter().map(|(_, y)| *y).max().unwrap() as usize;
    let matrix = vec![vec![false; xsize]; ysize];
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
