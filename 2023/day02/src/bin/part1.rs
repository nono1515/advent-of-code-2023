use regex::Regex;
use std::fs;

fn main() {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let cubes_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let input = fs::read_to_string("input.txt").expect("couldn't read file");
    let mut sum = 0;

    'game_loop: for (i, line) in input.lines().enumerate() {
        for group in cubes_re.captures_iter(line) {
            let n = group.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let color = group.get(2).unwrap().as_str();

            match color {
                "red" => {
                    if n > red_cubes {
                        continue 'game_loop;
                    }
                }
                "green" => {
                    if n > green_cubes {
                        continue 'game_loop;
                    }
                }
                "blue" => {
                    if n > blue_cubes {
                        continue 'game_loop;
                    }
                }
                _ => {
                    panic!("Cubes color {color} not in red, green or blue")
                }
            }
        }
        // println!("sum += {}", i + 1);
        sum += i + 1;
    }

    println!("{}", sum);
}
