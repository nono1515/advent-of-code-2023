use day16::{count_energized, get_path, LightBeam};

fn main() {
    let input = include_str!("../../input.txt");

    let mut max = 0;

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for start_i in -1..=map.len() as i8 {
        for start_j in -1..=map[0].len() as i8 {
            if (start_i == -1 || start_i == map.len() as i8)
                && (start_j == -1 || start_j == map[0].len() as i8)
            {
                continue;
            }
            if start_i != -1
                && start_j != -1
                && start_i != map.len() as i8
                && start_j != map[0].len() as i8
            {
                continue;
            }

            let init_beam = LightBeam {
                x: start_j,
                y: start_i,
                vx: {
                    if start_j == -1 {
                        1
                    } else if start_j == map[0].len() as i8 {
                        -1
                    } else {
                        0
                    }
                },
                vy: {
                    if start_i == -1 {
                        1
                    } else if start_i == map.len() as i8 {
                        -1
                    } else {
                        0
                    }
                },
            };
            let visited_places = get_path(&map, &init_beam);
            // print_visited_places(&visited_places);
            let count = count_energized(&visited_places);

            if count > max {
                max = count;
            }
        }
    }

    println!("{}", max);
}

    