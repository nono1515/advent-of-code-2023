fn main() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
    let input = include_str!("../../input.txt");
    let mut left_n = vec![];
    let mut right_n = vec![];

    input.lines().for_each(|line| {
        let (left, right) = line.split_once("   ").unwrap();

        left_n.push(u32::from_str_radix(left, 10).unwrap());
        right_n.push(u32::from_str_radix(right, 10).unwrap());
    });

    left_n.sort();
    right_n.sort();

    println!(
        "{}",
        left_n
            .iter()
            .zip(right_n)
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u32>()
    );
}
