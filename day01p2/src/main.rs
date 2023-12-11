use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn create_number_map() -> HashMap<&'static str, u8> {
    let number_map: HashMap<&'static str, u8> = [
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

fn find_index_of_extreme_value(input_array: &Vec<[i8; 2]>, mode: &str) -> usize {
    let mut extreme_value = 0;
    let mut index_of_extreme = 0;

    if mode == "min" {
        extreme_value = i8::MAX;
    }

    for (index, array) in input_array.iter().enumerate() {
        let value = array[0]; // Access the second dimension
        if (mode == "min" && value < extreme_value) || (mode == "max" && value > extreme_value) {
            extreme_value = value;
            index_of_extreme = index;
        }
    }
    index_of_extreme
}

fn main() {
    let mut file = File::open("puzzle_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut calibration_values_of_all_lines: Vec<i32> = Vec::new();

    let number_map = create_number_map();

    for line in contents.lines() {
        let mut collection_of_all_numbers_and_their_index: Vec<[i8; 2]> = Vec::new();

        // search for digits
        let mut char_index = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                collection_of_all_numbers_and_their_index.push([char_index, c.to_digit(10).unwrap().try_into().unwrap()]);
            }
            char_index += 1;
        }

        // search for digits hidden as substrings
        for key in number_map.keys() {
            if line.contains(key) {
                for (index, _) in line.match_indices(key) {
                    if let Some(&numeric_value) = number_map.get(key) {
                        let index_i8 = index as i8;
                        let numeric_value_i8 = numeric_value as i8;
                        collection_of_all_numbers_and_their_index.push([index_i8, numeric_value_i8]);
                    }
                }
            }
        }

        // Iterate through the vector and collect indices of min/max values
        let min_index = find_index_of_extreme_value(&collection_of_all_numbers_and_their_index, "min");
        let max_index = find_index_of_extreme_value(&collection_of_all_numbers_and_their_index, "max");

        let mut values_of_first_and_last_numbers_as_strings: [String; 2] = Default::default();
        values_of_first_and_last_numbers_as_strings[0] = collection_of_all_numbers_and_their_index[min_index][1].to_string();
        values_of_first_and_last_numbers_as_strings[1] = collection_of_all_numbers_and_their_index[max_index][1].to_string();

        calibration_values_of_all_lines.push(values_of_first_and_last_numbers_as_strings.join("").parse::<i32>().unwrap());
    }

    let sum: i32 = calibration_values_of_all_lines.iter().sum();
    println!("---------");
    println!("sum: {:?}", sum);
}
