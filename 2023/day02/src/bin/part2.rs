use regex::Regex;
use std::fs;

fn main() {
    let cubes_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let input = fs::read_to_string("input.txt").expect("couldn't read file");
    let mut power_sum = 0;

    for line in input.lines() {
        let games = line.split(|c| c == ':' || c == ';');
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for (i, game) in games.into_iter().enumerate() {
            if i == 0 {
                continue;
            } else {
                let cubes = game.split(',');
                for cube in cubes {
                    let Some(cubes_cap) = cubes_re.captures(cube) else {
                        println!("Game not matching");
                        println!("{}", game);
                        break;
                    };
                    let (_, [count, type_]) = cubes_cap.extract();
                    let count: i32 = count.parse().unwrap();
                    
                    match type_ {
                        "red" => {
                            if count > min_red {
                                min_red = count;
                            }
                        }
                        "green" => {
                            if count > min_green {
                                min_green = count;
                            }
                        }
                        "blue" => {
                            if count > min_blue {
                                min_blue = count;
                            }
                        }
                        _ => {
                            panic!("Cubes color {type_} not in red, green or blue")
                        }
                    }
                }
            }
        }
        power_sum += min_red * min_green * min_blue;
    }

    println!("The sum is {power_sum}");
}
