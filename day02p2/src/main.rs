use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // let mut file = File::open("example_input").expect("File not found");
    let mut file = File::open("puzzle_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut total_sum_of_powers_of_colors = 0;

    for line in contents.lines() {
        let mut amount_of_stones_per_color_per_game: HashMap<String, i32> = HashMap::new();

        let split_by_colon = line.split(":");
        for (index, part) in split_by_colon.enumerate() {
            // collect game_id of each line
            if index == 1 {
                let split_of_each_round = part.split(";");
                // example content: " 1 green, 3 red, 6 blue"
                for part in split_of_each_round {
                    let split_by_color = part.split(",");
                    for part in split_by_color {
                        let final_split = part.split(" ");
                        // example content " 1 green" -> 3 parts
                        let mut color: String = Default::default();
                        let mut amount = 0 as i32;
                        for (index, part) in final_split.enumerate() {
                            if index == 1 {
                                amount = part.parse::<i32>().unwrap();
                            } else if index == 2 {
                                color = part.to_string();
                            }
                        }

                        // check for highest used amount
                        if let Some(previous_amount) = amount_of_stones_per_color_per_game.get(&color.to_string()) {
                            if &amount > previous_amount {
                                amount_of_stones_per_color_per_game.insert(color.to_string(), amount);
                            }
                        } else {
                            amount_of_stones_per_color_per_game.insert(color.to_string(), amount);
                        }
                    }
                }
            }
        }

        let mut power_of_colors_per_game = 1;
        for (_, value) in amount_of_stones_per_color_per_game.iter() {
            power_of_colors_per_game *= value;
        }
        total_sum_of_powers_of_colors += power_of_colors_per_game;

    }
    println!("total_sum_of_powers_of_colors: {:?}", total_sum_of_powers_of_colors);
}
