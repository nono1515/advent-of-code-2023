use std::collections::HashMap;
use std::time::Instant;

fn count_stones_after_steps(
    stone: usize,
    n_steps: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&count) = cache.get(&(stone, n_steps)) {
        return count;
    }

    if n_steps == 0 {
        return 1;
    }

    let count = match stone {
        0 => count_stones_after_steps(1, n_steps - 1, cache),
        _ => {
            let n_digits = (stone as f32).log10().floor() as usize + 1;
            if n_digits % 2 == 0 {
                let divisor = 10_usize.pow((n_digits / 2) as u32);
                count_stones_after_steps(stone / divisor, n_steps - 1, cache)
                    + count_stones_after_steps(stone % divisor, n_steps - 1, cache)
            } else {
                count_stones_after_steps(stone * 2024, n_steps - 1, cache)
            }
        }
    };

    cache.insert((stone, n_steps), count);
    count
}

fn solve(input: &str, steps: usize) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| count_stones_after_steps(stone, steps, &mut cache))
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    let part1 = solve(input, 25);
    println!("Part 1: {} in {:?}", part1, now.elapsed());

    let now = Instant::now();
    let part2 = solve(input, 75);
    println!("Part 2: {} in {:?}", part2, now.elapsed());
}

#[test]
fn test_example() {
    let input = "125 17";
    assert_eq!(solve(input, 25), 55312);
}
