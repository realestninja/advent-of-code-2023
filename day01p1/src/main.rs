use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("puzzle_input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut calibration_value_digits: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut array_of_first_and_last_digit: [String; 2] = Default::default();
        for c in line.chars() {
            if c.is_digit(10) {
                let digit_index = if array_of_first_and_last_digit[0].is_empty() { 0 } else { 1 };
                array_of_first_and_last_digit[digit_index] = c.to_string();
            }
        }
        if array_of_first_and_last_digit[1].is_empty() {
            array_of_first_and_last_digit[1] = array_of_first_and_last_digit[0].clone()
        }
        calibration_value_digits.push(array_of_first_and_last_digit.join("").parse::<i32>().unwrap());
    }

    let sum: i32 = calibration_value_digits.iter().sum();
    println!("sum: {:?}", sum);
}
