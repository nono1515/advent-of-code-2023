use std::ops::RangeInclusive;

fn part1(ranges: &Vec<RangeInclusive<u64>>, ids: &Vec<u64>) -> u64 {
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count() as u64
}

fn part2(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut ranges = ranges.clone();
    ranges.sort_by_key(|r| *r.start());
    let mut nonoverlapping_ranges = vec![ranges[0].clone()];
    for r in ranges.into_iter().skip(1) {
        let last_end = *nonoverlapping_ranges.last().unwrap().end();
        if *r.start() > last_end + 1 {
            nonoverlapping_ranges.push(r);
        } else {
            let last_r = nonoverlapping_ranges.last_mut().unwrap();
            *last_r = *last_r.start()..=(*r.end()).max(last_end);
        }
    }
    nonoverlapping_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            l.parse::<u64>().unwrap()..=r.parse::<u64>().unwrap()
        })
        .collect();
    (
        ranges,
        ids_str.lines().map(|c| c.parse().unwrap()).collect(),
    )
}

fn main() {
    let input = include_str!("../input.txt");
    let (ranges, ids) = parse_input(input);
    println!("part1: {}", part1(&ranges, &ids));
    println!("part2: {}", part2(&ranges));
}

#[test]
fn test_both_parts() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let (ranges, ids) = parse_input(input);
    assert_eq!(part1(&ranges, &ids), 3);
    assert_eq!(part2(&ranges), 14);
}

#[test]
fn test_part2_edge_cases() {
    // Test 1: Ranges that share a boundary (5 appears in both)
    let ranges = vec![3..=5, 5..=7];
    // Should merge to 3-7: 5 IDs total
    assert_eq!(part2(&ranges), 5);

    // Test 2: Adjacent ranges (no overlap, no gap)
    let ranges = vec![3..=5, 6..=8];
    // Should merge to 3-8: 6 IDs total
    assert_eq!(part2(&ranges), 6);

    // Test 3: Ranges with a gap
    let ranges = vec![3..=5, 7..=9];
    // Should NOT merge: 3+3 = 6 IDs total
    assert_eq!(part2(&ranges), 6);

    // Test 4: Multiple overlapping ranges
    let ranges = vec![1..=5, 3..=7, 6..=10];
    // Should merge to 1-10: 10 IDs total
    assert_eq!(part2(&ranges), 10);

    // Test 5: Completely contained range
    let ranges = vec![1..=10, 3..=5];
    // Should merge to 1-10: 10 IDs total
    assert_eq!(part2(&ranges), 10);
}
