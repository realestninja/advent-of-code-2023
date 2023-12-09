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

fn save_index_and_value(arr: &mut [[i8; 2]; 2], index: usize, char_index: i8, character: char) {
    arr[index][0] = char_index;
    arr[index][1] = character.to_digit(10).unwrap().try_into().unwrap();
}

fn main() {
    let mut file = File::open("puzzle_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut calibration_value_digits: Vec<i32> = Vec::new();

    let number_map = create_number_map();
    println!("number_map: {:?}", number_map);
    println!("number_map: {:?}", number_map.keys());

    for line in contents.lines() {
        let mut indices_and_values_of_first_and_last_number: [[i8; 2]; 2] = [[-1; 2]; 2]; // two dimensional array: [index, value of number]
        let mut values_of_first_and_last_numbers_as_strings: [String; 2] = Default::default();

        // search for numeric numbers
        let mut char_index = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                println!("c: {:?}", c);
                // check if first slot is used
                if indices_and_values_of_first_and_last_number[0][0] < 0 {
                    save_index_and_value(&mut indices_and_values_of_first_and_last_number, 0, char_index, c);
                } else {
                    save_index_and_value(&mut indices_and_values_of_first_and_last_number, 1, char_index, c);
                }
            }
            char_index += 1;
        }

        // search for substring numbers
        println!("line: {:?}", line);
        for key in number_map.keys() {
            if line.contains(key) {
                match line.find(key) {
                    Some(index) => {
                        println!("The substring '{}' starts at index {}", key, index);
                        // to do: juggle numbers
                    }
                    None => {}
                }
            }
        }

        // if there was no 2nd value, clone the first value
        if indices_and_values_of_first_and_last_number[1][1] < 1 {
            indices_and_values_of_first_and_last_number[1][1] = indices_and_values_of_first_and_last_number[0][1].clone();
        }

        // convert numbers into string
        values_of_first_and_last_numbers_as_strings[0] = indices_and_values_of_first_and_last_number[0][1].to_string();
        values_of_first_and_last_numbers_as_strings[1] = indices_and_values_of_first_and_last_number[1][1].to_string();

        // add 2-digit-string to vector where all 2-digit-strings are collected
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
