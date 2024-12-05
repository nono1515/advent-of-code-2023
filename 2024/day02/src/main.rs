fn parse_line<'a>(line: &'a str) -> impl Iterator<Item = i32> + Clone + 'a {
    line.split(' ').map(|x| x.parse::<i32>().unwrap())
}

fn is_report_safe<T>(report: T) -> bool
where
    T: Iterator<Item = i32> + Clone,
{
    let mut pairs = report.clone().zip(report.skip(1));
    pairs.clone().all(|(a, b)| (a - b).abs() <= 3)
        && (pairs.clone().all(|(a, b)| a < b) || pairs.all(|(a, b)| a > b))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| is_report_safe(parse_line(line)))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let report = parse_line(line);
            (0..report.clone().count()).any(|n| {
                is_report_safe(
                    report
                        .clone()
                        .enumerate()
                        .filter(|&(i, _)| i != n)
                        .map(|(_, v)| v),
                )
            })
        })
        .count()
}

fn main() {
    let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    // let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
