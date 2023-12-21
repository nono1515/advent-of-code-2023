fn main() {
    let mut input = include_str!("../../input.txt").lines();
    let t = parse_line(input.next().unwrap());
    let d = parse_line(input.next().unwrap());

    println!("time {}", t);
    println!("Distance {}", d);

    let delta = (t.pow(2) - 4 * d) as f64;
    let sol1 = (t as f64 + delta.sqrt()) / 2.;
    let sol2 = (t as f64 - delta.sqrt()) / 2.;

    println!(
        "There are {} ways",
        sol1.ceil() as u64 - sol2.floor() as u64 - 1
    );
}

fn parse_line(line: &str) -> u64 {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
