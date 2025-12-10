fn part1(input: &str) -> u64 {
    let mut total = 0;
    let mut lines = input.lines();
    let mut beam = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'S' => true,
            _ => false,
        })
        .collect::<Vec<_>>();
    while let Some(line) = lines.next() {
        let mut next_beam = vec![false; beam.len()];
        for (i, (c, b)) in line.chars().zip(beam).enumerate() {
            if c == '^' && b {
                if let Some(prev) = next_beam.get_mut(i - 1) {
                    *prev = true;
                }
                if let Some(next) = next_beam.get_mut(i + 1) {
                    *next = true;
                }
                total += 1;
            } else if b {
                next_beam[i] = true;
            }
        }
        beam = next_beam;
    }
    total
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut beam = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'S' => 1,
            _ => 0,
        })
        .collect::<Vec<_>>();

    while let Some(line) = lines.next() {
        let mut next_beam = vec![0; beam.len()];
        for (i, (c, b)) in line.chars().zip(beam).enumerate() {
            if c == '^' && b > 0 {
                if let Some(prev) = next_beam.get_mut(i - 1) {
                    *prev = *prev + b;
                }
                if let Some(next) = next_beam.get_mut(i + 1) {
                    *next = *next + b;
                }
            } else {
                next_beam[i] += b;
            }
        }
        beam = next_beam;
        // println!("{:?}", beam);
    }
    beam.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_both_parts() {
    let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 40);
}
