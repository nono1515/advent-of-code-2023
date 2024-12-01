fn main() {
    let input = include_str!("../../input.txt");
    let mut sum = 0;

    for pattern in input.split("\n\n").map(|p| p.as_bytes()) {
        let width = pattern.iter().position(|&c| c == b'\n').unwrap() + 1;
        let height = if pattern[pattern.len() - 1] == b'\n' {
            pattern.len() / width // Last list has an additionnal newline
        } else {
            pattern.len() / width + 1
        };

        // row mirroring
        let mut row_val = 0;
        'row_loop: for i in 1..height {
            for (i0, i1) in (0..=i - 1).rev().zip(i..height) {
                if pattern[i0 * width..i0 * width + width - 1]
                    != pattern[i1 * width..i1 * width + width - 1]
                {
                    continue 'row_loop;
                }
            }
            row_val = i;
            break;
        }

        // column mirroring
        let mut col_val = 0;
        'col_loop: for i in 1..width - 1 {
            for (i0, i1) in (0..=i - 1).rev().zip(i..width - 1) {
                for row_index in 0..height {
                    if pattern[row_index * width + i0] != pattern[row_index * width + i1] {
                        continue 'col_loop;
                    }
                }
            }
            col_val = i;
            break;
        }

        sum += col_val + 100 * row_val;
    }

    println!("sum: {}", sum);
}
