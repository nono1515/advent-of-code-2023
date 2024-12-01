use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let mut total = 0;

    for (i, line) in input.lines().enumerate() {
        let mut parts = line.split_whitespace();
        let springs = [parts.next().unwrap()].repeat(5).join("?");
        let aligns = [parts.next().unwrap()].repeat(5).join(",");
        let line_part2 = format!("{} {}", springs, aligns);
        let mut parts_part2 = line_part2.split_whitespace();
        let springs: Vec<_> = parts_part2
            .next()
            .unwrap()
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect();
        let arr: Vec<u32> = parts_part2
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        // println!("{:?}, {:?}", springs, arr);
        let mut memo: HashMap<(Vec<String>, Vec<u32>), u64> = HashMap::new();
        let n = compute_arrangements(springs, arr, &mut memo);
        // println!("{i}: {n}");

        total += n;
    }

    println!("{}", total);
}

fn compute_arrangements(
    patterns: Vec<String>,
    arrangements: Vec<u32>,
    memo: &mut HashMap<(Vec<String>, Vec<u32>), u64>,
) -> u64 {
    if let Some(&result) = memo.get(&(patterns.clone(), arrangements.clone())) {
        return result;
    }

    let result = if arrangements.is_empty() {
        patterns.iter().all(|p| !p.contains('#')) as u64
    } else if patterns.is_empty() {
        0
    } else if (patterns[0].len() as u32) < arrangements[0] {
        if patterns[0].contains('#') {
            0
        } else {
            compute_arrangements(patterns[1..].to_vec(), arrangements[..].to_vec(), memo)
        }
    } else {
        let mut total = 0;
        for i in (arrangements[0] as usize)..=(patterns[0].len()) {
            // No # are allowed before the split
            if patterns[0][..i - arrangements[0] as usize].contains('#') {
                continue;
            }
            // Neither right after the split, as it would be be skipped
            if patterns[0].chars().nth(i).unwrap_or('.') == '#' {
                continue;
            }
            let mut new_patterns = vec![];
            if patterns[0].len() > i + 1 && patterns[0].chars().nth(i).unwrap() != '#' {
                new_patterns.push(patterns[0][i + 1..].to_owned()); // Convert to owned strings
            }
            new_patterns.extend_from_slice(&patterns[1..]);
            total += compute_arrangements(new_patterns, arrangements[1..].to_vec(), memo);
        }
        if !patterns[0].contains('#') {
            total += compute_arrangements(patterns[1..].to_vec(), arrangements[..].to_vec(), memo);
        }
        total
    };

    memo.insert((patterns.clone(), arrangements.clone()), result);
    result
}
