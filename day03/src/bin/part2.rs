use day03::{Number, States};
use std::{fs, vec};

struct LonelyStar {
    index: usize,
    row: usize,
    value: u32,
}

#[derive(Debug)]
struct Star {
    index: usize,
    row: usize,
    values: Vec<u32>,
}

fn push_star(stars: &mut Vec<Star>, star: LonelyStar) {
    for s in stars.iter_mut() {
        if s.index == star.index && s.row == star.row {
            s.values.push(star.value);
            return;
        }
    }
    stars.push(Star {
        index: star.index,
        row: star.row,
        values: vec![star.value],
    });
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    let mut stars = vec![];

    let lines = input.lines().collect::<Vec<_>>();

    for i in 0..=lines.len() - 1 {
        process_line(&lines, i, &mut stars)
    }

    // println!("The stars are {:?}", stars);
    for s in stars {
        if s.values.len() == 2 {
            sum += s.values[0] * s.values[1];
        }
    }
    println!("The sum is {}", sum);
}

fn process_line(lines: &[&str], row: usize, stars: &mut Vec<Star>) {
    let state = &mut States::NoDigit;
    for (i, c) in lines[row].chars().enumerate() {
        if let Some(res) = state.process_next_char(c, i) {
            if row == 0 {
                let temp_stars = process_result(res, &lines[..2], row);
                for s in temp_stars {
                    push_star(stars, s);
                }
            } else if row == lines.len() - 1 {
                let temp_stars = process_result(res, &lines[lines.len() - 2..], row - 1);
                for s in temp_stars {
                    push_star(stars, s);
                }
            } else {
                let temp_stars = process_result(res, &lines[row - 1..=row + 1], row - 1);
                for s in temp_stars {
                    push_star(stars, s);
                }
            }
        }
    }
}

fn process_result(res: Number, lines: &[&str], row: usize) -> Vec<LonelyStar> {
    let mut stars = vec![];
    for (j, line) in lines.iter().enumerate() {
        let start = {
            if res.get_start_i() > 0 {
                res.get_start_i() - 1
            } else {
                0
            }
        };
        let end = {
            if res.get_end_i() < line.len() - 1 {
                res.get_end_i() + 1
            } else {
                line.len() - 1
            }
        };
        for (i, c) in line[start..=end].chars().enumerate() {
            if c == '*' {
                stars.push(LonelyStar {
                    index: start + i,
                    row: row + j,
                    value: res.get_value(),
                })
            }
        }
    }
    stars
}
