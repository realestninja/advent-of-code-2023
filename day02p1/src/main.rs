use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("example_input").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    for line in contents.lines() {
        println!("");
        println!("line: {:?}", line);
    }
    println!("");
}
