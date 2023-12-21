use day05::line_to_nums;

fn main() {
    let input = include_str!("../../input.txt");

    let mut mappings = input.split("\n\n");
    let ranges: Vec<_> = line_to_nums(mappings.next().unwrap())
        .chunks(2)
        .map(|x| (x[0]..x[0] + x[1]))
        .collect();

    let output = ranges
        .into_iter()
        .flat_map(|range| {
            mappings.clone().fold(vec![range], |ranges, maps| {
                let mut out = vec![];
                let mut in_updated = ranges;
                for data in maps.lines().skip(1).map(|line| line_to_nums(line)) {
                    let in_: Vec<_> = in_updated.drain(..).collect();
                    for r in in_ {
                        let d_start = data[1];
                        let d_end = data[1] + data[2];
                        let d_dest_start = data[0];
                        // let d_dest_end = data[0] + data[2];  // Not required

                        if !(d_start >= r.end || d_end <= r.start) {
                            if d_start <= r.start {
                                let rmin = if r.end < d_end { r.end } else { d_end };
                                out.push(
                                    d_dest_start + r.start - d_start..d_dest_start + rmin - d_start,
                                );
                                if r.end > d_end {
                                    in_updated.push(d_end..r.end);
                                }
                            } else {
                                in_updated.push(r.start..d_start);
                                let rmin = if r.end < d_end { r.end } else { d_end };
                                out.push(d_dest_start..d_dest_start + rmin - d_start);
                                if r.end > d_end {
                                    in_updated.push(d_end..r.end);
                                }
                            }
                            continue;
                        }
                        in_updated.push(r);
                    }
                }
                out.append(&mut in_updated);
                out
            })
        })
        // .collect::<Vec<_>>();
        .fold(
            u64::MAX,
            |acc, n| if acc <= n.start { acc } else { n.start },
        );

    println!("{:?}", output);
}
