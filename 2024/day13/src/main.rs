use std::{str::Chars, time::Instant};

fn parse_num(chars: &mut Chars) -> usize {
    chars
        .skip_while(|c| !c.is_numeric())
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part1(grid: &str) -> usize {
    let mut total = 0;

    for ops in grid.split("\n\n") {
        let mut ops_it = ops.chars();
        let x_a = parse_num(&mut ops_it);
        let y_a = parse_num(&mut ops_it);
        let x_b = parse_num(&mut ops_it);
        let y_b = parse_num(&mut ops_it);
        let x = parse_num(&mut ops_it);
        let y = parse_num(&mut ops_it);

        let a = (x * y_b - y * x_b) / (x_a * y_b - y_a * x_b);
        let b = (y_a * x_b - x_a * y_b) / (x_a * y_b - y_a * x_b);
    }

    total
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    0
}

fn main() {
    let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    // let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input), now.elapsed());

    // let now = Instant::now();
    // println!("Part 2: {} in {:?}", part2(&grid), now.elapsed());
}

#[test]
fn test_example() {
    let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    assert_eq!(part1(&input), 1930);

    // assert_eq!(part2(&input), 1206);
}
