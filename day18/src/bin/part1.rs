use day18::plot_map;

fn main() {
    let input = include_str!("../../input.txt");

    let mut corners = vec![(0, 0)];
    let mut border = 0.;

    input.lines().for_each(|line| {
        let (dir, groups) = line.split_once(' ').unwrap();
        let (steps, _color) = groups.split_once(' ').unwrap();
        let steps = steps.parse::<i32>().unwrap();
        border += steps as f32;
        match dir {
            "R" => corners.push((corners.last().unwrap().0, corners.last().unwrap().1 + steps)),
            "D" => corners.push((corners.last().unwrap().0 + steps, corners.last().unwrap().1)),
            "L" => corners.push((corners.last().unwrap().0, corners.last().unwrap().1 - steps)),
            "U" => corners.push((corners.last().unwrap().0 - steps, corners.last().unwrap().1)),
            _ => panic!("Unknown direction: {}", dir),
        }
    });

    plot_map(&corners);

    let mut area = 0.5
        * corners
            .windows(2)
            .fold(0., |acc, cs| {
                acc + (cs[0].0 * cs[1].1 - cs[0].1 * cs[1].0) as f32
            })
            .abs();

    area += 0.5 * border + 1.;
    println!("Part 1: {}", area);
}
