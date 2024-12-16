use std::{
    fs::File,
    io::Write,
    str::{Chars, FromStr},
    time::Instant,
};

fn parse_num<T: FromStr>(chars: &mut Chars) -> Result<T, <T as FromStr>::Err> {
    chars
        .skip_while(|c| !c.is_numeric() && *c != '-')
        .take_while(|c| c.is_numeric() || *c == '-')
        .collect::<String>()
        .parse::<T>()
}

fn parse_line(line: &str) -> (i16, i16, i16, i16) {
    let mut it = line.chars();

    let x = parse_num(&mut it).unwrap();
    let y = parse_num(&mut it).unwrap();
    let vx = parse_num(&mut it).unwrap();
    let vy = parse_num(&mut it).unwrap();

    (x, y, vx, vy)
}

fn part1(input: &str, grid_width: i16, grid_height: i16) -> usize {
    let mut quadrants = [0; 4];
    let half_width = grid_width / 2;
    let half_height = grid_height / 2;

    for line in input.lines() {
        let (mut x, mut y, vx, vy) = parse_line(line);
        // println!("start: {} {} {} {}", x, y, vx, vy);
        for _ in 0..100 {
            x = (x + vx).rem_euclid(grid_width);
            y = (y + vy).rem_euclid(grid_height);
        }
        // println!("finished: {} {} {} {}", x, y, vx, vy);
        if y < half_height {
            if x < half_width {
                quadrants[0] += 1;
            } else if x > half_width {
                quadrants[1] += 1;
            }
        } else if y > half_height {
            if x < half_width {
                quadrants[2] += 1;
            } else if x > half_width {
                quadrants[3] += 1;
            }
        }
    }

    quadrants.iter().product()
}

fn part2(input: &str, grid_width: i16, grid_height: i16) -> usize {
    let mut robots = input.lines().map(parse_line).collect::<Vec<_>>();

    for i in 1..10000 {
        let mut grid = vec![vec![false; grid_width as usize]; grid_height as usize];
        for robot in &mut robots {
            let (x, y, vx, vy) = &mut *robot;
            *x = (*x + *vx).rem_euclid(grid_width);
            *y = (*y + *vy).rem_euclid(grid_height);
            grid[*y as usize][*x as usize] = true;
        }

        // Definitely not what I was expecting but it got the job done
        let in_tree = grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let r = (i as f32 / grid_height as f32 * grid_width as f32 / 2.0) as usize;
                let half_width = grid_width as usize / 2;
                row[(half_width - r)..(half_width + r)]
                    .iter()
                    .filter(|b| **b)
                    .count()
            })
            .sum::<usize>();

        if in_tree > 4 * robots.len() / 5 {
            println!("{}: {} / {}", i, in_tree, robots.len());
            let mut file = File::create("img/{}.txt".replace("{}", &i.to_string())).unwrap();
            let grid_str = grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|b| if *b { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>();
            file.write_all(grid_str.join("\n").as_bytes()).unwrap();

            return i;
        }
    }

    0
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input, 101, 103), now.elapsed());

    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(&input, 101, 103), now.elapsed());
}

#[test]
fn test_example() {
    let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    assert_eq!(part1(&input, 11, 7), 12);

    // The test will not work and returns 0 (but this should be an Error)
    assert_eq!(part2(&input, 11, 7), 0);
}
