fn main() {
    let mut input = include_str!("../../input.txt").lines();
    let times = parse_line(input.next().unwrap());
    let distances = parse_line(input.next().unwrap());

    println!("times {:?}", times);
    println!("Distances {:?}", distances);

    let mut product = 1;
    for (t, d) in times.into_iter().zip(distances) {
        let delta = (t.pow(2) - 4 * d) as f64;
        let sol1 = (t as f64 + delta.sqrt()) / 2.;
        let sol2 = (t as f64 - delta.sqrt()) / 2.;
        product *= sol1.ceil() as u64 - sol2.floor() as u64 - 1;
    }

    println!("The product is {}", product);
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|s| {
            if s.chars().all(|c| c.is_ascii_digit()) {
                Some(s.parse().unwrap())
            } else {
                None
            }
        })
        .collect()
}
