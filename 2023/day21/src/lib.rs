use std::collections::VecDeque;

pub fn djikstra(grid: Vec<Vec<bool>>, start: (usize, usize)) -> Vec<(usize, usize, usize)> {
    let mut nodes: Vec<(usize, usize, usize)> = vec![(start.0, start.1, 0)];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start.0 as usize][start.1 as usize] = true;

    let mut last_visited = VecDeque::from(nodes.clone());
    while !last_visited.is_empty() {
        let (y, x, c) = last_visited.pop_front().unwrap();
        if x > 0 {
            if grid[y][x - 1] && !visited[y][x - 1] {
                nodes.push((y, x - 1, c + 1));
                last_visited.push_back((y, x - 1, c + 1));
                visited[y][x - 1] = true;
            }
        }
        if x < grid[0].len() - 1 {
            if grid[y][x + 1] && !visited[y][x + 1] {
                nodes.push((y, x + 1, c + 1));
                last_visited.push_back((y, x + 1, c + 1));
                visited[y][x + 1] = true;
            }
        }
        if y > 0 {
            if grid[y - 1][x] && !visited[y - 1][x] {
                nodes.push((y - 1, x, c + 1));
                last_visited.push_back((y - 1, x, c + 1));
                visited[y - 1][x] = true;
            }
        }
        if y < grid.len() - 1 {
            if grid[y + 1][x] && !visited[y + 1][x] {
                nodes.push((y + 1, x, c + 1));
                last_visited.push_back((y + 1, x, c + 1));
                visited[y + 1][x] = true;
            }
        }
    }
    nodes
}
