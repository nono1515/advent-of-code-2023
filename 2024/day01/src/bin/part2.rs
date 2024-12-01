use std::collections::HashMap;

fn main() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
    let input = include_str!("../../input.txt");
    let mut left_n = HashMap::new();
    let mut right_n = HashMap::new();

    input.lines().for_each(|line| {
        let (left, right) = line.split_once("   ").unwrap();

        *left_n
            .entry(u32::from_str_radix(left, 10).unwrap())
            .or_insert(0) += 1;
        *right_n
            .entry(u32::from_str_radix(right, 10).unwrap())
            .or_insert(0) += 1;
    });

    println!(
        "{}",
        left_n
            .iter()
            .map(|(k, v)| k // The number
                * v // Number of times it appears on the left
                * right_n.get(k).unwrap_or(&0)) // Number of times it appears on the right
            .sum::<u32>()
    );
}
