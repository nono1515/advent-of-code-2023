fn main() {
    let input = include_str!("../../input.txt");
    let mut total = 0;

    for (i, line) in input.lines().enumerate() {
        let mut parts = line.split_whitespace();
        let springs: Vec<_> = parts
            .next()
            .unwrap()
            .split('.')
            .filter(|s| !s.is_empty())
            .collect();
        let arr: Vec<u32> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let n = compute_arrangements(springs.clone(), arr.clone());
        // println!("{:?}, {:?}", springs, arr);
        // println!("{i}: {n}");

        total += n;
    }

    println!("{}", total);
}

fn compute_arrangements(patterns: Vec<&str>, arrangements: Vec<u32>) -> u32 {
    if arrangements.is_empty() {
        patterns.iter().all(|p| !p.contains('#')) as u32
    } else if patterns.is_empty() {
        0
    } else if (patterns[0].len() as u32) < arrangements[0] {
        if patterns[0].contains('#') {
            0
        } else {
            compute_arrangements(patterns[1..].to_vec(), arrangements[..].to_vec())
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
                new_patterns.push(&patterns[0][i + 1..]);
            }
            new_patterns.extend(&patterns[1..]);
            total += compute_arrangements(new_patterns, arrangements[1..].to_vec());
        }
        if !patterns[0].contains('#') {
            total += compute_arrangements(patterns[1..].to_vec(), arrangements[..].to_vec());
        }
        total
    }
}
