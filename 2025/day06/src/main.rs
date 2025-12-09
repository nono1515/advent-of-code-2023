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

fn part2(nums: &Vec<Vec<u32>>, ops: &Vec<char>) -> u64 {
    0
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<char>) {
    let mut lines = input.lines().peekable();
    let mut nums = vec![];
    let mut ops = vec![];
    while let Some(line) = lines.next() {
        println!("{:?}", line.split(' ').collect::<Vec<_>>());
        if lines.peek().is_some() {
            nums.push(line.split(' ').map(|s| s.parse::<u32>().unwrap()).collect())
        } else {
            ops = line
                .split(' ')
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
    println!("part2: {}", part2(&nums, &ops));
}

#[test]
fn test_both_parts() {
    let input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let (nums, ops) = parse_input(input);
    assert_eq!(part1(&nums, &ops), 4277556);
    // assert_eq!(part2(&nums, &ops), 14);
}
