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

        let mut char_index = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                let array_of_indices_current_index = if array_of_indices[0] < 0 { 0 } else { 1 };
                array_of_indices[array_of_indices_current_index] = char_index;
            }
            char_index += 1;
        }

        // handle first value
        if array_of_indices[0] >= 0 {
            let first_value = line.chars().nth(array_of_indices[0] as usize);
            let value = match first_value {
                Some(v) => v,
                None => panic!("No value found"),
            };
            println!("value: {:?}", value);
            array_of_first_and_last_digit[0] = value.to_string();
        }

        // handle second value
        if array_of_indices[1] >= 0 {
            let second_value = line.chars().nth(array_of_indices[1] as usize);
            let value = match second_value {
                Some(v) => v,
                None => panic!("No value found"),
            };
            println!("value: {:?}", value);
            array_of_first_and_last_digit[1] = value.to_string();
        } else {
            array_of_first_and_last_digit[1] = array_of_first_and_last_digit[0].clone()
        }

        println!("array_of_first_and_last_digit: {:?}", array_of_first_and_last_digit);

        calibration_value_digits.push(array_of_first_and_last_digit.join("").parse::<i32>().unwrap());
    }

    let sum: i32 = calibration_value_digits.iter().sum();
    println!("sum: {:?}", sum);
}
