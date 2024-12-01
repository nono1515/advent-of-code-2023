fn main() {
    let input = include_str!("../../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starting_point = grid
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(j, &c)| c == 'S')
                .map(move |(j, c)| (i, j))
        })
        .next()
        .unwrap();

    let mut pos = starting_point;
    let mut dir;
    if starting_point.0 > 0 && "|7F".contains(grid[pos.0 - 1][pos.1]) {
        dir = Dir::Up;
        pos = (pos.0 - 1, pos.1);
    } else if starting_point.0 <= grid.len() - 2 && "|LJ".contains(grid[pos.0 + 1][pos.1]) {
        dir = Dir::Down;
        pos = (pos.0 + 1, pos.1);
    } else {
        dir = Dir::Right;
        pos = (pos.0, pos.1 + 1);
    }
    let initial_dir = dir;

    let mut c = grid[pos.0][pos.1];
    let mut steps = 1;
    let mut corners = vec![];
    loop {
        if "F7JL".contains(c) {
            corners.push(pos);
        }
        match (dir, c) {
            (Dir::Up, '|') => {
                pos = (pos.0 - 1, pos.1);
            }
            (Dir::Up, 'F') => {
                // corners.push((pos.0 + 0.5, pos.1 + 0.5));
                pos = (pos.0, pos.1 + 1);
                dir = Dir::Right;
            }
            (Dir::Up, '7') => {
                // corners.push((pos.0 + 0.5, pos.1 + 0.5));
                pos = (pos.0, pos.1 - 1);
                dir = Dir::Left;
            }
            (Dir::Down, '|') => {
                pos = (pos.0 + 1, pos.1);
            }
            (Dir::Down, 'J') => {
                pos = (pos.0, pos.1 - 1);
                dir = Dir::Left;
            }
            (Dir::Down, 'L') => {
                pos = (pos.0, pos.1 + 1);
                dir = Dir::Right;
            }
            (Dir::Left, '-') => {
                pos = (pos.0, pos.1 - 1);
            }
            (Dir::Left, 'L') => {
                pos = (pos.0 - 1, pos.1);
                dir = Dir::Up;
            }
            (Dir::Left, 'F') => {
                pos = (pos.0 + 1, pos.1);
                dir = Dir::Down;
            }
            (Dir::Right, '-') => {
                pos = (pos.0, pos.1 + 1);
            }
            (Dir::Right, '7') => {
                pos = (pos.0 + 1, pos.1);
                dir = Dir::Down;
            }
            (Dir::Right, 'J') => {
                pos = (pos.0 - 1, pos.1);
                dir = Dir::Up;
            }
            (_, _) => {
                unreachable!("invalid input");
            }
        }

        steps += 1;
        c = grid[pos.0][pos.1];
        if c == 'S' {
            if initial_dir != dir {
                corners.push(pos);
            }
            break;
        }
    }

    // OK so this is actually much smarter than the other solution where I was just
    // checking whether we crossed a vertical wall to change a flag to count or not.
    // This is using two theorem:
    //    - https://en.wikipedia.org/wiki/Shoelace_formula
    //    - https://en.wikipedia.org/wiki/Pick%27s_theorem
    // The first one is used to compute the area of a polygon
    // And the second one can be used to compute the inner points of a polygon from its
    // area, computed with the 1st theorem

    // Shoelace formula to compute the area: area = 1/2 * sum(x_i*y_(i+1) - x_(i+1)*y_i)
    corners.push(corners[0]);
    let area = corners.windows(2).fold(0 as i64, |acc, pair| {
        acc + (pair[0].0 * pair[1].1) as i64 - (pair[0].1 * pair[1].0) as i64
    }).abs() / 2;

    // Pick's Theorem: area = i + b / 2 - 1 <=> i = area - b / 2 + 1
    // where i is the number of inner points and 
    // b is the number of inter points on the polygon boundary
    
    let i = area - steps / 2 + 1;
    println!("inner points: {}", i);
    println!("steps: {}", steps);
    println!("area: {}", area);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}
