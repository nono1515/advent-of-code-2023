use day21::djikstra;

fn main() {
    let input = include_str!("../../input.txt");
    const STEPS: usize = 26501365;

    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c != '#').collect())
        .collect();
    // println!("{} x {}", grid.len(), grid[0].len());

    let start: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(
                move |(j, c)| {
                    if c == 'S' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();


    // This problem can be solved much easier due to its geometric property
    // If you go and have a look at the input, you can see that the starting point
    // is on a fully "unblocked" row and column, which means that any neighboring map
    // is accesible in (grid.len() / 2) steps, and the next one in grid.len() steps.
    // Also, the final number of steps is right at the end of one map:
    // 26501365 = 65 + 131 * 202300
    // So what we can do is to compute the possible destinations for the case where
    // we are able to go to 
    // - Only the curent map
    // - the first neightboring map
    // - the 2nd one
    // And the extrapole quadratically

    // First we compute the solution for 65, 65 + 131 and 65 + 2 * 131 steps
    let mut ys = vec![];
    for n in 0..3 {
        let mut big_grid = vec![vec![false; grid[0].len() * (2 * n + 1)]; grid.len() * (2 * n + 1)];
        for i in 0..2 * n + 1 {
            for j in 0..2 * n + 1 {
                for y in 0..grid.len() {
                    for x in 0..grid[0].len() {
                        big_grid[i * grid.len() + y][j * grid[0].len() + x] = grid[y][x];
                    }
                }
            }
        }

        let big_start = (start[0].0 + n * grid.len(), start[0].1 + n * grid[0].len());
        let nodes = djikstra(big_grid, big_start);

        let s = 65 + n * 131;
        ys.push(
            nodes
                .iter()
                .filter(|(_, _, c)| c % 2 == s % 2 && *c <= s)
                .count(),
        );
    }

    let x_interp = ((STEPS - 65) / 131) as i64;
    // println!("{}", x_interp);
    // println!("{:?}", ys);

    // Now we can interpolate using Lagrange Interpolation
    // https://en.wikipedia.org/wiki/Polynomial_interpolation#Lagrange_Interpolation
    let xs = vec![0, 1, 2];
    let n = ys
        .iter()
        .enumerate()
        .map(|(i, y)| {
            xs.iter()
                .enumerate()
                .filter_map(|(j, x)| {
                    if i != j {
                        Some((x_interp as i64 - *x as i64) / (xs[i] as i64 - *x as i64))
                    } else {
                        None
                    }
                })
                .product::<i64>()
                * *y as i64
        })
        .sum::<i64>();
    // The next one is the same, in the particular case of 2nd degree interpolation
    // let n = (x_interp - 1) * (x_interp - 2) / (0 - 1) / (0 - 2) * ys[0] as i64
    //     + (x_interp - 0) * (x_interp - 2) / (1 - 0) / (1 - 2) * ys[1] as i64
    //     + (x_interp - 0) * (x_interp - 1) / (2 - 1) / (2 - 0) * ys[2] as i64;

    println!("{}", n);
}
