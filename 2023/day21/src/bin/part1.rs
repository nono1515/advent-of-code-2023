use day21::djikstra;

fn main() {
    let input = include_str!("../../input.txt");
    const STEPS: usize = 64;

    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c != '#').collect())
        .collect();

    let start: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == 'S' { Some((i, j)) } else { None })
        })
        .collect();

    let nodes = djikstra(grid, start[0]);

    println!(
        "{}",
        nodes
            .iter()
            .filter(|(_, _, c)| c % 2 == STEPS % 2 && *c <= STEPS)
            .count()
    );
}
