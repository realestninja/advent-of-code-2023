use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("puzzle_input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let line_count = contents.lines().count();
    let mut calibration_value_digits: Vec<Vec<String>> = vec![vec![Default::default(); 2]; line_count];

    for (i, line) in contents.lines().enumerate() {
        for c in line.chars() {
            if c.is_digit(10) {
                let is_last_digit = if calibration_value_digits[i][0].is_empty() { 0 } else { 1 };
                calibration_value_digits[i][is_last_digit] = c.to_string();
            }
        }
        println!("{}", line);
        println!("first digit: {}", calibration_value_digits[i][0]);
        println!(" last digit: {}", calibration_value_digits[i][1]);
        println!("-------");
    }
}
