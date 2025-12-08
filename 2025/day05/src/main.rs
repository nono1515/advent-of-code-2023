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

