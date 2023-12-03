use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("puzzle_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut calibration_value_digits: Vec<i32> = Vec::new();

    let number_map: HashMap<&str, u8> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    for line in contents.lines() {
        let mut array_of_indices: [i8; 2] = [-1, -1];
        let mut array_of_first_and_last_digit: [String; 2] = Default::default();

        // remake from here
        // get indices of first and last number
        // get indices of beginning of substring

        let mut char_index = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                let array_of_indices_current_index = if array_of_indices[0] < 0 { 0 } else { 1 };
                array_of_indices[array_of_indices_current_index] = char_index;

                // old
                let digit_index = if array_of_first_and_last_digit[0].is_empty() { 0 } else { 1 };
                array_of_first_and_last_digit[digit_index] = c.to_string();
            }
            char_index += 1;
        }

        // handle case where there is only one number
        if array_of_first_and_last_digit[1].is_empty() {
            array_of_first_and_last_digit[1] = array_of_first_and_last_digit[0].clone()
        }

        // remake end
        calibration_value_digits.push(array_of_first_and_last_digit.join("").parse::<i32>().unwrap());
        println!("array_of_indices: {:?}", array_of_indices);
    }

    let sum: i32 = calibration_value_digits.iter().sum();
    println!("sum: {:?}", sum);
}
