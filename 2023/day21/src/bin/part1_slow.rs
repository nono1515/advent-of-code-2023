fn main() {
    let input = include_str!("../../input.txt");
    const STEPS: usize = 64;
    // const STEPS: usize = 6;

    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c != '#').collect())
        .collect();
    // println!("{} x {}", grid.len(), grid[0].len());

    let mut destinations: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == 'S' { Some((i, j)) } else { None })
        })
        .collect();
    // println!("{:?}", destinations);

    for _ in 0..STEPS {
        let mut new_destinations = vec![];
        while !destinations.is_empty() {
            let (y, x) = destinations.pop().unwrap();
            if x > 0 {
                if grid[y][x - 1] && !new_destinations.contains(&(y, x - 1)) {
                    new_destinations.push((y, x - 1));
                }
            }
            if x < grid[0].len() - 1 {
                if grid[y][x + 1] && !new_destinations.contains(&(y, x + 1)) {
                    new_destinations.push((y, x + 1));
                }
            }
            if y > 0 {
                if grid[y - 1][x] && !new_destinations.contains(&(y - 1, x)) {
                    new_destinations.push((y - 1, x));
                }
            }
            if y < grid.len() - 1 {
                if grid[y + 1][x] && !new_destinations.contains(&(y + 1, x)) {
                    new_destinations.push((y + 1, x));
                }
            }
        }
        destinations = new_destinations;
    }

    println!("{}", destinations.len());
}
