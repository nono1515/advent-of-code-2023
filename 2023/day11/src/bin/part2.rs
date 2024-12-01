use day11::mask_of_1;
use primitive_types::U256;

fn main() {
    let input = include_bytes!("../../input.txt");

    let width = input.iter().position(|&b| b == b'\n').unwrap();
    let height = input.len() / (width + 1);

    let mut empty_rows = (U256::from(1) << width) - 1;
    let mut empty_cols = (U256::from(1) << height) - 1;

    let mut galaxy_loc = vec![];
    for (i, c) in input.iter().enumerate() {
        if c == &b'#' {
            let row = i / (width + 1);
            let col = i % (width + 1);
            empty_rows &= !(U256::from(1) << row);
            empty_cols &= !(U256::from(1) << col);
            galaxy_loc.push((row, col));
        }
    }

    let mut steps: u64 = 0;
    let expension_factor = 1_000_000;
    for i in 0..galaxy_loc.len() {
        for j in i + 1..galaxy_loc.len() {
            let gi = galaxy_loc[i];
            let gj = galaxy_loc[j];

            // columns
            steps += (gi.0 as i32 - gj.0 as i32).abs() as u64;
            let mask = (mask_of_1(gi.0) ^ mask_of_1(gj.0)) & empty_rows;
            let expension = (0..256).map(|i| mask.bit(i)).filter(|b| *b).count();
            steps += expension as u64 * (expension_factor - 1);

            // rows
            steps += (gi.1 as i32 - gj.1 as i32).abs() as u64;
            let mask = (mask_of_1(gi.1) ^ mask_of_1(gj.1)) & empty_cols;
            let expension = (0..256).map(|i| mask.bit(i)).filter(|b| *b).count();
            steps += expension as u64 * (expension_factor - 1);
        }
    }

    println!("{}", steps);
}
