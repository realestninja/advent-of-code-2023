use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn create_number_map() -> std::collections::HashMap<&'static str, u8> {
    let mut number_map: std::collections::HashMap<&'static str, u8> = [
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
    number_map
}

fn main() {
    let mut file = File::open("puzzle_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut calibration_value_digits: Vec<i32> = Vec::new();

    let number_map = create_number_map();

    for line in contents.lines() {
        let mut indices_of_first_and_last_numeric_numbers: [i8; 2] = [-1, -1];
        let mut indices_of_first_and_last_string_numbers: [i8; 2] = [-1, -1];

        let mut values_of_first_and_last_numbers_as_strings: [String; 2] = Default::default();

        let mut char_index = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                let first_number_otherwise_last = if indices_of_first_and_last_numeric_numbers[0] < 0 { 0 } else { 1 };
                indices_of_first_and_last_numeric_numbers[first_number_otherwise_last] = char_index;
            }
            // to do: p2 task, find numbers written as string
            // search string for each substring

            char_index += 1;
        }

        // handle first value
        // refactor into functon
        if indices_of_first_and_last_numeric_numbers[0] >= 0 {
            let first_value = line.chars().nth(indices_of_first_and_last_numeric_numbers[0] as usize);
            let value = match first_value {
                Some(v) => v,
                None => panic!("No value found"),
            };
            values_of_first_and_last_numbers_as_strings[0] = value.to_string();
        }

        // handle second value
        // refactor into functon
        if indices_of_first_and_last_numeric_numbers[1] >= 0 {
            let second_value = line.chars().nth(indices_of_first_and_last_numeric_numbers[1] as usize);
            let value = match second_value {
                Some(v) => v,
                None => panic!("No value found"),
            };
            values_of_first_and_last_numbers_as_strings[1] = value.to_string();
        } else {
            values_of_first_and_last_numbers_as_strings[1] = values_of_first_and_last_numbers_as_strings[0].clone()
        }

        calibration_value_digits.push(values_of_first_and_last_numbers_as_strings.join("").parse::<i32>().unwrap());
    }

    let sum: i32 = calibration_value_digits.iter().sum();
    println!("sum: {:?}", sum);
    if sum == 55002 {
        println!("test passed");
    } else {
        println!("test failed");
    }
}
