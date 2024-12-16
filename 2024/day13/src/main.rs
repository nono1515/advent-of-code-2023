use std::{str::Chars, time::Instant};

fn parse_num(chars: &mut Chars) -> i32 {
    chars
        .skip_while(|c| !c.is_numeric())
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i32>()
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

        let cross_prod = x_a * y_b - y_a * x_b;
        if cross_prod == 0 {
            panic!("The two commands are parallel, we need to handle this case if it happens");
        } else {
            let a = (x * y_b - y * x_b) / cross_prod;
            let b = (y * x_a - x * y_a) / cross_prod;

            if a * x_a + b * x_b == x && a * y_a + b * y_b == y {
                if a <= 100 && b <= 100 {
                    total += (a * 3 + b) as usize;
                    // println!("({}, {}) -> ({}, {}) with cost {}", a, b, x, y, a * 3 + b);
                }
            }
        }
    }

    total
}

fn part2(grid: &str) -> usize {
    let mut total = 0;

    for ops in grid.split("\n\n") {
        let mut ops_it = ops.chars();
        let x_a = parse_num(&mut ops_it) as i64;
        let y_a = parse_num(&mut ops_it) as i64;
        let x_b = parse_num(&mut ops_it) as i64;
        let y_b = parse_num(&mut ops_it) as i64;
        let x = parse_num(&mut ops_it) as i64 + 10000000000000;
        let y = parse_num(&mut ops_it) as i64 + 10000000000000;

        let cross_prod = x_a * y_b - y_a * x_b;
        if cross_prod == 0 {
            panic!("The two commands are parallel, we need to handle this case if it happens");
        } else {
            let a = (x * y_b - y * x_b) / cross_prod;
            let b = (y * x_a - x * y_a) / cross_prod;

            if a * x_a + b * x_b == x && a * y_a + b * y_b == y {
                total += (a * 3 + b) as usize;
                // println!("({}, {}) -> ({}, {}) with cost {}", a, b, x, y, a * 3 + b);
            }
        }
    }

    total
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input), now.elapsed());

    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(&input), now.elapsed());
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

    assert_eq!(part1(&input), 480);

    // the next one was not given by the creator so I completed is myself after completing part 2
    assert_eq!(part2(&input), 875318608908);
}
