use day03::{Number, States};
use std::{fs, vec};

const SYMBOLS: [char; 10] = ['*', '@', '/', '+', '$', '=', '&', '-', '#', '%'];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    find_symbols(&input);

    let mut state = States::NoDigit;
    let mut sum = 0;

    let mut last_three_lines = input.lines().take(3).collect::<Vec<_>>();
    // process first line
    sum += process_line(&last_three_lines, 0, &mut state);

    for line in input.lines().skip(3) {
        // process middle line
        sum += process_line(&last_three_lines, 1, &mut state);
        last_three_lines.remove(0);
        last_three_lines.push(line);
    }

    // process the 2 last line
    sum += process_line(&last_three_lines, 1, &mut state);
    sum += process_line(&last_three_lines, 2, &mut state);

    println!("The sum is {}", sum);
}

fn process_line(lines: &[&str], index: usize, state: &mut States) -> u32 {
    let mut sum = 0;
    for (i, c) in lines[index].chars().enumerate() {
        if let Some(res) = state.process_next_char(c, i) {
            match index {
                0 => sum += process_result(res, &lines[..2]),
                1 => sum += process_result(res, &lines),
                2 => sum += process_result(res, &lines[1..]),
                _ => panic!("Index must be <= 2"),
            }
        }
    }
    sum
}

fn process_result(res: Number, lines: &[&str]) -> u32 {
    for line in lines {
        let start = {
            let start_i = res.get_start_i();
            if start_i > 0 {
                start_i - 1
            } else {
                0
            }
        };
        let end = {
            let end_i = res.get_end_i();
            if end_i < line.len() - 1 {
                end_i + 1
            } else {
                line.len() - 1
            }
        };
        let chars = &line[start..=end];
        for s in SYMBOLS {
            if chars.contains(s) {
                // println!("Found {} in {:?}", res.value, lines);
                return res.get_value();
            }
        }
    }
    0
}

fn find_symbols(input: &str) {
    let mut symbols = vec![];
    for line in input.lines() {
        for c in line.chars() {
            if !symbols.contains(&c) && !c.is_numeric() {
                symbols.push(c);
            }
        }
    }
    println!("{:?}", symbols);
}
