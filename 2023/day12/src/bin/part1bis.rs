use std::collections::VecDeque;

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
        let arr: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let n = compute_arrangements(&springs, &arr);
        println!("{:?}, {:?}", springs, arr);
        println!("{i}: {n}");

        total += n;
    }

    println!("{}", total);
}

fn compute_arrangements(patterns: &Vec<&str>, alignements: &Vec<usize>) -> u32 {
    let mut total = 0;

    let mut arrs = VecDeque::new();
    arrs.push_back(vec![alignements.len()]);

    while !arrs.iter().all(|tup| tup.len() == patterns.len()) {
        let e = arrs.pop_front().unwrap();
        if let Some((last, elems)) = e.split_last() {
            for i in 0..=alignements.len() {
                if *last as i32 - i as i32 >= 0 {
                    let mut new = elems.to_vec();
                    new.push(last - i);
                    new.push(i);
                    arrs.push_back(new);
                }
            }
        }
    }

    for arr in arrs {
        total += count_sol(&patterns, &alignements, arr);
    }

    total
}

fn count_sol(patterns: &Vec<&str>, alignements: &Vec<usize>, arrangement: Vec<usize>) -> u32 {
    // Check if there is an unffiled pattern that contains a # to faster computation
    if patterns
        .iter()
        .zip(arrangement.clone())
        .any(|(p, arr)| p.contains('#') && arr == 0)
    {
        return 0;
    }

    let mut count = 1;

    // println!("{:?}, {:?}, {:?}", patterns, alignements, arrangement);
    // patterns example: ["?", "???##??#????"]
    // alignements example: [1, 1, 6, 1]
    // arrangement example: [1, 3]

    let mut prev_i = 0;
    for (p, arr) in patterns.iter().zip(arrangement) {
        if arr == 0 {
            continue;
        }
        let current_alignement = &alignements[prev_i..prev_i + arr].to_vec();
        count *= arrangement_per_pattern(p, current_alignement.to_owned());
        prev_i += arr;
    }
    count
}

fn arrangement_per_pattern(pattern: &str, alignement: Vec<usize>) -> u32 {
    let mut total = 0;
    // fast check to escape early
    if pattern.len() < alignement.iter().sum() {
        return 0;
    }
    for i in 0..=(pattern.len() - alignement[0]) {
        // We must not skip any '#'

        if i > 0 && pattern.chars().nth(i - 1) == Some('#') {
            continue;
        }
        if pattern[..i].contains('#') {
            continue;
        }
        if pattern.chars().nth(i + alignement[0]) == Some('#') {
            continue;
        }

        // if the current alignement does not fit the patern
        if alignement.len() == 1 {
            if pattern
                .chars()
                .enumerate()
                .all(|(c_i, c)| c != '#' || (i <= c_i && c_i < i + alignement[0]))
            {
                total += 1;
            }
        } else if i + alignement[0] + 1 > pattern.len() {
            continue;
        } else {
            total += arrangement_per_pattern(
                &pattern[i + alignement[0] + 1..],
                alignement[1..].to_vec(),
            );
        }
    }
    total
}
