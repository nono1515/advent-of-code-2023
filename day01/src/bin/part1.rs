use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut sum = 0;

    for line in input.lines() {
        // Here's my first trial
        // println!("{line}");
        // let digits: Vec<&str> = line.rmatches(char::is_numeric).collect();
        // println!("{digits:?}");
        // let last_digit: i32 = digits[0].parse().unwrap();
        // let first_digit: i32 = digits[digits.len() - 1].parse().unwrap();

        // Now, I watned a version that does not store every int along the way
        let (first_digit, last_digit) = line.chars().fold((None, None), |acc, c| {
            if c.is_numeric() {
                (
                    acc.0.or(Some(c.to_digit(10).unwrap())), // Set first_digit if it's None
                    Some(c.to_digit(10).unwrap()),           // Always update last_digit
                )
            } else {
                acc
            }
        });
        let first_digit = first_digit.unwrap_or(0);
        let last_digit = last_digit.unwrap_or(0);
        sum += first_digit * 10 + last_digit;
    }

    println!("The sum is {sum}");
}
