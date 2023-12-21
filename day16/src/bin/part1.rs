fn main() {
    let input = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    let input = include_str!("../../input.txt");

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited_places = vec![
        vec![
            Tile {
                up: false,
                down: false,
                left: false,
                right: false,
            };
            map[0].len()
        ];
        map.len()
    ];

    let mut light_beams = vec![LightBeam {
        x: -1,
        y: 0,
        vx: 1, // Going right
        vy: 0,
    }];

    while !light_beams.is_empty() {
        let beam = light_beams.pop().unwrap();

        // check for out of bounds
        if beam.x + beam.vx < 0
            || beam.x + beam.vx >= map[0].len() as i8
            || beam.y + beam.vy < 0
            || beam.y + beam.vy >= map.len() as i8
        {
            continue;
        }
        // Update position
        let x = beam.x + beam.vx;
        let y = beam.y + beam.vy;
        // check if we have already been there with the same direction
        // and update the tile we're on
        if beam.vx == 1 {
            if visited_places[y as usize][x as usize].right {
                continue;
            }
            visited_places[y as usize][x as usize].right = true;
        }
        if beam.vx == -1 {
            if visited_places[y as usize][x as usize].left {
                continue;
            }
            visited_places[y as usize][x as usize].left = true;
        }
        if beam.vy == 1 {
            if visited_places[y as usize][x as usize].down {
                continue;
            }
            visited_places[y as usize][x as usize].down = true;
        }
        if beam.vy == -1 {
            if visited_places[y as usize][x as usize].up {
                continue;
            }
            visited_places[y as usize][x as usize].up = true;
        }
        // Get the next beam(s)
        // print!("{}", map[y as usize][x as usize]);
        match map[y as usize][x as usize] {
            '.' => light_beams.push(LightBeam {
                x: x,
                y: y,
                vx: beam.vx,
                vy: beam.vy,
            }),
            '|' => {
                if beam.vx != 0 {
                    light_beams.push(LightBeam {
                        x: x,
                        y: y,
                        vx: 0,
                        vy: 1,
                    });
                    light_beams.push(LightBeam {
                        x: x,
                        y: y,
                        vx: 0,
                        vy: -1,
                    });
                } else {
                    light_beams.push(LightBeam {
                        x: x,
                        y: y,
                        vx: beam.vx,
                        vy: beam.vy,
                    })
                }
            }
            '-' => {
                if beam.vy != 0 {
                    light_beams.push(LightBeam {
                        x: x,
                        y: y,
                        vx: 1,
                        vy: 0,
                    });
                    light_beams.push(LightBeam {
                        x: x,
                        y: y,
                        vx: -1,
                        vy: 0,
                    })
                } else {
                    light_beams.push(LightBeam {
                        x: x,
                        y: y,
                        vx: beam.vx,
                        vy: beam.vy,
                    })
                }
            }
            '\\' => light_beams.push(LightBeam {
                x: x,
                y: y,
                vx: beam.vy,
                vy: beam.vx,
            }),
            '/' => light_beams.push(LightBeam {
                x: x,
                y: y,
                vx: -beam.vy,
                vy: -beam.vx,
            }),
            _ => panic!("Unknown tile type: {}", map[y as usize][x as usize]),
        }
    }

    // print_visited_places(&visited_places);
    println!(
        "{}",
        visited_places
            .iter()
            .map(|row| row
                .iter()
                .map(
                    |place| if place.up || place.down || place.left || place.right {
                        1
                    } else {
                        0
                    }
                )
                .sum::<u32>())
            .sum::<u32>()
    );
}

fn print_visited_places(visited_places: &Vec<Vec<Tile>>) {
    for row in visited_places {
        for place in row {
            print!(
                "{}",
                if place.up || place.down || place.left || place.right {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!("");
    }
}

struct LightBeam {
    x: i8,
    y: i8,
    vx: i8,
    vy: i8,
}

#[derive(Clone, Copy)]
struct Tile {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
