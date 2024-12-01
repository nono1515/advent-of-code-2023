use day10::{Path, Pipe};
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input.txt");
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(Pipe::parse).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut starting_row = 0;
    let mut starting_col = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, pipe) in line.iter().enumerate() {
            if let Pipe::StartingPoint = pipe {
                starting_row = i;
                starting_col = j;
            };
        }
    }

    let mut in_ = VecDeque::new();
    in_.push_back(Path {
        row: starting_row,
        col: starting_col,
        steps: 0,
        pipe: grid[starting_row][starting_col],
    });
    let mut visited = vec![];
    let mut out = vec![];

    while !in_.is_empty() {
        let p = in_.pop_front().unwrap();
        let mut north_pipe = false;
        let mut south_pipe = false;
        let mut west_pipe = false;
        let mut east_pipe = false;
        match p.pipe {
            Pipe::StartingPoint => {
                north_pipe = true;
                south_pipe = true;
                west_pipe = true;
                east_pipe = true;
            }
            Pipe::Pipe {
                north,
                south,
                east,
                west,
            } => {
                north_pipe = north;
                south_pipe = south;
                west_pipe = west;
                east_pipe = east;
            }
            Pipe::NoPipe => {
                panic!("no pipe")
            }
        }
        if north_pipe && p.row >= 1 {
            if !visited.contains(&(p.row - 1, p.col)) {
                let pipe = grid[p.row - 1][p.col];
                if let Pipe::Pipe { south, .. } = pipe {
                    if south {
                        in_.push_back(Path {
                            row: p.row - 1,
                            col: p.col,
                            steps: p.steps + 1,
                            pipe: pipe,
                        });
                    }
                };
            }
            north_pipe = false;
        }
        if south_pipe && p.row < grid.len() - 1 {
            if !visited.contains(&(p.row + 1, p.col)) {
                let pipe = grid[p.row + 1][p.col];
                if let Pipe::Pipe { north, .. } = pipe {
                    if north {
                        in_.push_back(Path {
                            row: p.row + 1,
                            col: p.col,
                            steps: p.steps + 1,
                            pipe: pipe,
                        });
                    }
                };
            }
            south_pipe = false;
        }
        if west_pipe && p.col >= 1 {
            if !visited.contains(&(p.row, p.col - 1)) {
                let pipe = grid[p.row][p.col - 1];
                if let Pipe::Pipe { east, .. } = pipe {
                    if east {
                        in_.push_back(Path {
                            row: p.row,
                            col: p.col - 1,
                            steps: p.steps + 1,
                            pipe: grid[p.row][p.col - 1],
                        });
                    }
                }
            }
            west_pipe = false;
        }
        if east_pipe && p.col < grid[0].len() - 1 {
            if !visited.contains(&(p.row, p.col + 1)) {
                let pipe = grid[p.row][p.col + 1];
                if let Pipe::Pipe { west, .. } = pipe {
                    if west {
                        in_.push_back(Path {
                            row: p.row,
                            col: p.col + 1,
                            steps: p.steps + 1,
                            pipe: grid[p.row][p.col + 1],
                        });
                    }
                }
            }
            east_pipe = false;
        }

        if let Pipe::StartingPoint = p.pipe {
            grid[p.row][p.col] = Pipe::Pipe {
                north: !north_pipe,
                south: !south_pipe,
                east: !east_pipe,
                west: !west_pipe,
            };
        }

        visited.push((p.row, p.col));
        out.push(p);
    }

    let mut count = 0;
    for (i, line) in grid.iter().enumerate() {
        let mut counting = false;
        for (j, pipe) in line.iter().enumerate() {
            if visited.contains(&(i, j)) {
                if let Pipe::Pipe { north, .. } = pipe {
                    if *north {
                        counting = !counting;
                    }
                }
            } else if counting {
                count += 1;
            }
        }
    }

    // for p in &out {
    //     println!("{} {} {} {:?}", p.row, p.col, p.steps, p.pipe);
    // }
    println!("{}", count);
}
