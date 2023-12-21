use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let numbers_str = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        let mut current_word = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                first_digit = Some(c.to_digit(10).unwrap());
            }
            current_word.push_str(&c.to_string());
            for (i, word) in numbers_str.iter().enumerate() {
                if current_word.contains(word) {
                    first_digit = Some(u32::try_from(i).unwrap());
                }
            }
            if first_digit.is_some() {
                break;
            }
        }
        let first_digit = first_digit.unwrap();

        let mut current_word = String::new();
        for c in line.chars().rev() {
            if c.is_numeric() {
                last_digit = Some(c.to_digit(10).unwrap());
            }
            current_word.insert_str(0, &c.to_string());
            for (i, word) in numbers_str.iter().enumerate() {
                if current_word.contains(word) {
                    last_digit = Some(u32::try_from(i).unwrap());
                }
            }
            if last_digit.is_some() {
                break;
            }
        }
        let last_digit = last_digit.unwrap();

        sum += last_digit + first_digit * 10;
    }

    println!("The sum is {sum}");
}
