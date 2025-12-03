fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (mut d1, mut d2) = (0, 0);
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                let d = c.to_digit(10).unwrap();
                if d > d1 && chars.peek().is_some() {
                    d1 = d;
                    d2 = 0;
                } else {
                    d2 = d2.max(d);
                }
            }
            d1 * 10 + d2
        })
        .sum::<u32>() as i32
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut digits = [None; 12];
            for c in line.chars().rev() {
                let mut d = c.to_digit(10).unwrap() as u64;
                let last_none = digits.iter().rev().position(|c| c.is_none());
                if let Some(i) = last_none {
                    digits[digits.len() - 1 - i] = Some(d);
                } else {
                    for digit in &mut digits {
                        if d > digit.unwrap() {
                            let prev = digit.unwrap();
                            *digit = Some(d);
                            d = prev;
                        } else if d < digit.unwrap() {
                            break;
                        }
                    }
                }
            }
            digits
                .iter()
                .rev()
                .enumerate()
                .map(|(i, d)| 10u64.pow(i as u32) * d.unwrap() as u64)
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_parts() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";

    assert_eq!(part1(&input), 357);
    assert_eq!(part2(&input), 3121910778619);
}
