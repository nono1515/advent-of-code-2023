use day16::{count_energized, get_path, LightBeam};

fn main() {
    let input = include_str!("../../input.txt");

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let init_beam = LightBeam {
        x: -1,
        y: 0,
        vx: 1, // Going right
        vy: 0,
    };
    let visited_places = get_path(&map, &init_beam);
    // print_visited_places(&visited_places);
    println!("{}", count_energized(&visited_places));
}
