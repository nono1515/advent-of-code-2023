pub fn line_to_nums(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .filter(|s| s.trim().chars().all(|c| c.is_ascii_digit()))
        .map(|s| s.parse().unwrap())
        .collect()
}
