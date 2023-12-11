use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("example_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    // let load_of_red = 12;
    // let load_of_green = 13;
    // let load_of_blue = 14;

    for line in contents.lines() {
        let mut game_id = 0;

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
                        for (index, part) in final_split.enumerate() {
                            if index == 1 {
                                // amount
                                println!("amount: {:?}", part);
                            } else if index == 2 {
                                // color
                                println!("color: {:?}", part);
                            }
                        }
                        println!("");
                    }
                }
            }
        }
        // println!("split_by_colon: {:?}", split_by_colon);
        println!("");
        println!("game_id: {:?}", game_id);
        println!("");
        // println!("line: {:?}", line);
    }
    println!("");

    let expected_example_output = 8;
    println!("expected_example_output: {:?}", expected_example_output);
}
