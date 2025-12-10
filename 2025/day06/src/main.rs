fn part1(nums: &Vec<Vec<u32>>, ops: &Vec<char>) -> u64 {
    let mut total: Vec<u64> = ops
        .iter()
        .map(|c| match c {
            '+' => 0,
            '*' => 1,
            _ => unreachable!(),
        })
        .collect();

    for line in nums {
        for ((n, op), tot) in line.iter().zip(ops).zip(total.iter_mut()) {
            match op {
                '+' => *tot += *n as u64,
                '*' => *tot *= *n as u64,
                _ => unreachable!(),
            }
        }
    }

    total.iter().sum()
}

fn part2(input: &str) -> u64 {
    let mut total = 0;
    let mut curr = Vec::new();
    let mut lines = input.lines().map(|l| l.chars()).collect::<Vec<_>>();
    let mut op = ' ';
    loop {
        let mut num = 0;
        let mut any = false;
        for l in &mut lines {
            if let Some(c) = l.next() {
                any = true;
                match c {
                    '+' | '*' => op = c,
                    ' ' => (),
                    _ => num = 10 * num + c.to_digit(10).unwrap() as u64,
                }
            }
        }
        if num == 0 {
            // println!("{:?}, with op {op}", curr);
            total += curr.iter().fold(if op == '+' { 0 } else { 1 }, |acc, n| {
                if op == '+' { acc + n } else { acc * n }
            });
            curr.clear();
        } else {
            curr.push(num);
        }
        if !any {
            break;
        }
    }
    total
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<char>) {
    let mut lines = input.lines().peekable();
    let mut nums = vec![];
    let mut ops = vec![];
    while let Some(line) = lines.next() {
        if lines.peek().is_some() {
            nums.push(
                line.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            )
        } else {
            ops = line
                .split_whitespace()
                .map(|s| s.parse::<char>().unwrap())
                .collect()
        }
    }
    (nums, ops)
}

fn main() {
    let input = include_str!("../input.txt");
    let (nums, ops) = parse_input(input);
    println!("part1: {}", part1(&nums, &ops));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_both_parts() {
    let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let (nums, ops) = parse_input(input);
    assert_eq!(part1(&nums, &ops), 4277556);
    assert_eq!(part2(&input), 3263827);
}
