use std::cmp::{max, min};

pub fn plot_map(corners: &[(i32, i32)]) {
    let min_x = corners.iter().map(|c| c.1).min().unwrap();
    let max_x = corners.iter().map(|c| c.1).max().unwrap();
    let min_y = corners.iter().map(|c| c.0).min().unwrap();
    let max_y = corners.iter().map(|c| c.0).max().unwrap();
    let mut map = vec![vec!["."; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for cs in corners.windows(2) {
        let x0 = min(cs[0].1, cs[1].1);
        let x1 = max(cs[0].1, cs[1].1);
        let y0 = min(cs[0].0, cs[1].0);
        let y1 = max(cs[0].0, cs[1].0);
        for x in x0..=x1 {
            for y in y0..=y1 {
                map[(y - min_y) as usize][(x - min_x) as usize] = "#";
            }
        }
    }
    for m in map {
        println!("{}", m.join(""));
    }
}
