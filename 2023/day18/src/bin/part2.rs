fn main() {
    let input = include_str!("../../input.txt");

    let mut corners = vec![(0, 0)];
    let mut border = 0.;

    input.lines().for_each(|line| {
        let (_dir, groups) = line.split_once(' ').unwrap();
        let (_steps, color) = groups.split_once(' ').unwrap();
        // println!("{} {}", &color, &color[1..color.len() - 1]);
        let steps = i64::from_str_radix(&color[2..color.len() - 2], 16).unwrap();
        let dir = color.chars().nth(color.len() - 2).unwrap();
        border += steps as f64;
        match dir {
            '0' => corners.push((corners.last().unwrap().0, corners.last().unwrap().1 + steps)),
            '1' => corners.push((corners.last().unwrap().0 + steps, corners.last().unwrap().1)),
            '2' => corners.push((corners.last().unwrap().0, corners.last().unwrap().1 - steps)),
            '3' => corners.push((corners.last().unwrap().0 - steps, corners.last().unwrap().1)),
            _ => panic!("Unknown direction: {}", dir),
        }
    });

    let mut area = 0.5
        * corners
            .windows(2)
            .fold(0., |acc, cs| {
                acc + (cs[0].0 * cs[1].1 - cs[0].1 * cs[1].0) as f64
            })
            .abs();

    area += 0.5 * border + 1.;
    println!("Part 1: {}", area);
}
