use day09::parse_line;

fn main() {
    let input = include_str!("../../input.txt");

    let sum = input
        .lines()
        .map(parse_line)
        .map(|history| {
            let mut diffs = vec![history];
            let mut last = diffs.last().unwrap().clone();
            
            while !last.iter().all(|&d| d == 0) {
                last = last.windows(2).map(|a| a[1] - a[0]).collect();
                diffs.push(last.clone());
            }
            // One-line equivalent
            // while !diffs.last().unwrap().iter().all(|&d| d == 0) {
            //     diffs.push(diffs.last().unwrap().windows(2).map(|a| { a[1] - a[0] }).collect());
            // }

            let mut last_diff: Option<&mut Vec<i64>> = None;
            for diff in diffs.iter_mut().rev() {
                if let Some(last_diff) = last_diff {
                    diff.insert(0, diff[0] - last_diff[0]);
                } else {
                    diff.insert(0, 0);
                }
                last_diff = Some(diff);
            }
            let last_diff = last_diff;
            last_diff.unwrap()[0]
        })
        .fold(0, |acc, x| acc + x);

    println!("{:?}", sum);
}


