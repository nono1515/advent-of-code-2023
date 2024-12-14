use std::time::Instant;

fn is_valid(target: u64, nums: &[u64]) -> bool {
    for comb in 0..(1 << nums.len()) {
        let res = nums
            .iter()
            .enumerate()
            .fold(0, |acc, (i, num)| match (comb & (1 << i)) >> i {
                0 => acc + num,
                1 => acc * num,
                _ => unreachable!(),
            });

        if res == target {
            return true;
        }
    }
    false
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (target, nums) = line.split_once(": ").unwrap();
    let target = target.parse::<u64>().unwrap();
    let nums = nums
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (target, nums)
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(parse_line)
        .filter_map(|(target, nums)| {
            if is_valid_reverse(target, nums, false) {
                Some(target)
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(parse_line)
        .filter_map(|(target, nums)| {
            if is_valid_reverse(target, nums, true) {
                Some(target)
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn old_part2(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let (target, nums) = parse_line(line);
        for comb in 0..(1 << nums.len() * 2) {
            let mut res = 0;
            for (i, num) in nums.iter().enumerate() {
                match (comb & (0b11 << (2 * i))) >> (2 * i) {
                    0 => res += num,
                    1 => res *= num,
                    2 => res = (res.to_string() + &num.to_string()).parse().unwrap(),
                    3 => break,
                    _ => unreachable!(),
                }
            }

            if res == target {
                total += target;
                break;
            }
        }
    }

    total
}

fn is_valid_reverse(target: u64, mut nums: Vec<u64>, cat: bool) -> bool {
    if let Some(n) = nums.pop() {
        if nums.is_empty() {
            return n == target;
        } else {
            return {
                if target % n == 0 {
                    is_valid_reverse(target / n, nums.clone(), cat)
                } else {
                    false
                }
            } || (cat && {
                let n_str = n.to_string();
                if target.to_string().ends_with(n_str.as_str()) {
                    is_valid_reverse(target / 10u64.pow(n_str.len() as u32), nums.clone(), cat)
                } else {
                    false
                }
            }) || is_valid_reverse(target - n, nums, cat);
        }
    }
    false
}

fn main() {
    let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(input), now.elapsed());
    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(input), now.elapsed());
}
