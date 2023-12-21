fn main() {
    let input = include_bytes!("../../input.txt");

    let width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let mut col_vals = vec![input.len() / width; width - 1];
    let mut sum = 0;

    for (i, &b) in input.iter().enumerate() {
        match b {
            b'\n' | b'.' => (),
            b'#' => col_vals[i % width] = (input.len() - 1 - i) / width,
            b'O' => {
                sum += col_vals[i % width];
                col_vals[i % width] -= 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", sum);
}
