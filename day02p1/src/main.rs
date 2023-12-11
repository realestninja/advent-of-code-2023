use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // let mut file = File::open("example_input").expect("File not found");
    let mut file = File::open("puzzle_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let limit_per_color: HashMap<String, i32> = HashMap::from_iter(vec![
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    let mut sum_of_game_ids = 0;

    for line in contents.lines() {
        let mut game_id = 0;
        let mut amount_of_colors_per_game: HashMap<String, i32> = HashMap::new();

        let split_by_colon = line.split(":");
        for (index, part) in split_by_colon.enumerate() {
            // collect game_id of each line
            if index == 0 {
                // example content: "Game 5"
                let split_by_space = part.split(" ");
                for (index, part) in split_by_space.enumerate() {
                    if index == 1 {
                        game_id = part.parse::<i32>().unwrap();
                    }
                }
            } else {
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

                        if let Some(current_amount) = amount_of_colors_per_game.get(&color.to_string()) {
                            amount_of_colors_per_game.insert(color.to_string(), current_amount + amount);
                        } else {
                            amount_of_colors_per_game.insert(color.to_string(), amount);
                        }
                    }
                }
            }
        }
        println!("");
        println!("game_id: {:?}", game_id);
        println!("amount_of_colors_per_game: {:?}", amount_of_colors_per_game);

        let mut game_is_valid = true;
        for key in limit_per_color.keys() {
            let limit_for_this_color = limit_per_color.get(key);
            let amount_of_this_color = amount_of_colors_per_game.get(key);
            if amount_of_this_color > limit_for_this_color {
                game_is_valid = false;
            }
        }
        println!("game_is_valid: {:?}", game_is_valid);

        if game_is_valid {
            sum_of_game_ids += game_id;
        }
    }
    println!("");
    println!("sum_of_game_ids: {:?}", sum_of_game_ids);
}
