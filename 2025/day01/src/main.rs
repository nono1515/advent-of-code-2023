fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).unwrap(),
                line.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
            )
        })
        .scan(50, |acc, (c, n)| {
            match c {
                'L' => *acc -= n,
                'R' => *acc += n,
                _ => unreachable!(),
            };
            Some(*acc)
        })
        .filter(|n| n % 100 == 0)
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).unwrap(),
                line.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
            )
        })
        .scan(50i32, |acc, (c, n)| {
            let prev_zero = *acc == 0;
            let prev_q = acc.div_euclid(100);
            match c {
                'L' => *acc -= n,
                'R' => *acc += n,
                _ => unreachable!(),
            };
            let new_q = acc.div_euclid(100);
            *acc = acc.rem_euclid(100);
            Some(
                (new_q - prev_q).abs()
                + (*acc == 0 && c == 'L') as i32  // if we arrive at 0 from the right, add one
                - (prev_zero && c == 'L') as i32, // if we start from 0 and go left, we'll change
                                                  // of 100s without crossing 0 so remove one
            )
        })
        .sum::<i32>() as u32
}

fn main() {
    let f = include_str!("../input.txt");
    println!("part 1: {}", part1(f));
    println!("part 2: {}", part2(f));
}

#[test]
fn test_parts() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 6);
}
