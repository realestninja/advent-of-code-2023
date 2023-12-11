use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn create_number_map() -> HashMap<&'static str, u8> {
    let mut number_map: HashMap<&'static str, u8> = [
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
    println!("number_map: {:?}", number_map);
    println!("number_map: {:?}", number_map.keys());

    for line in contents.lines() {
        let mut collection_of_all_values_and_their_index: Vec<[i8; 2]> = Vec::new();

        // search for numeric numbers
        let mut char_index = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                collection_of_all_values_and_their_index.push([char_index, c.to_digit(10).unwrap().try_into().unwrap()]);
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

                        if let Some(&numeric_value) = number_map.get(key) {
                            let index_i8 = index as i8;
                            let numeric_value_i8 = numeric_value as i8;
                            collection_of_all_values_and_their_index.push([index_i8, numeric_value_i8]);
                        }
                    }
                    None => {}
                }
            }
        }

        // add 2-digit-string to vector where all 2-digit-strings are collected
        println!("collection_of_all_values_and_their_index: {:?}", collection_of_all_values_and_their_index);
        println!("collection_of_all_values_and_their_index: {:?}", collection_of_all_values_and_their_index.iter());

        // Iterate through the vector and find the minimum value in the first dimension (index 0)
        let mut min_value = i8::MAX; // Initialize with the maximum possible value
        let mut max_value = 0;
        let mut min_index = 0;
        let mut max_index = 0;

        for (index, array) in collection_of_all_values_and_their_index.iter().enumerate() {
            let value = array[0]; // Access the second dimension
            if value < min_value {
                min_value = value;
                min_index = index;
            }
        }

        for (index, array) in collection_of_all_values_and_their_index.iter().enumerate() {
            let value = array[0]; // Access the second dimension
            if value > max_value {
                max_value = value;
                max_index = index;
            }
        }

        let min_element = collection_of_all_values_and_their_index[min_index];
        let max_element = collection_of_all_values_and_their_index[max_index];

        println!("Element with the minimum value: {:?}", min_element[1]);
        println!("Element with the maximum value: {:?}", max_element[1]);

        let mut values_of_first_and_last_numbers_as_strings: [String; 2] = Default::default();
        values_of_first_and_last_numbers_as_strings[0] = collection_of_all_values_and_their_index[min_index][1].to_string();
        values_of_first_and_last_numbers_as_strings[1] = collection_of_all_values_and_their_index[max_index][1].to_string();

        println!("values_of_first_and_last_numbers_as_strings: {:?}", values_of_first_and_last_numbers_as_strings);
        calibration_value_digits.push(values_of_first_and_last_numbers_as_strings.join("").parse::<i32>().unwrap());
        println!("------------------------------------");
    }

    let sum: i32 = calibration_value_digits.iter().sum();
    println!("sum: {:?}", sum);
}
