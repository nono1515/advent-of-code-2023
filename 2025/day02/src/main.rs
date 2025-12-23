fn part1(input: &str) -> i32 {
    let ranges = parse_ranges(input);
    ranges
        .iter()
        .map(|(l, h)| {
            let l_len = l.checked_ilog10().unwrap_or(1);
            let h_len = h.checked_ilog10().unwrap_or(0);
            let mut total = 0;
            for n_digit in l_len..=h_len {
                for sep in &factorization(&n_digit) {
                    let nums = (0..)
                }
            }
            total
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    0
}

fn parse_ranges(input: &str) -> Vec<(u32, u32)> {
    input
        .split(',')
        .map(|range_str| range_str.split_once(',').unwrap())
        // .map(|(str1, str2)| (str1.parse::<u32>().unwrap(), str2.parse::<u32>().unwrap()))
        .map(|(str1, str2)| (str1.parse().unwrap(), str2.parse().unwrap()))
        .collect()
}

fn factorization(n: &u32) -> Vec<u32> {
    (2..n.isqrt()).filter(|i| n % i == 0).collect()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_parts() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    assert_eq!(part1(&input), 1227775554);
}
