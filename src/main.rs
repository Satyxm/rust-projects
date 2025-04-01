use std::env;
use std::fs;

fn main() {
    let pattern: String = std::env::args().nth(1).expect("Usage: pattern");
    let path: String = std::env::args().nth(2).expect("Usage: path"); 

    let content = fs::read_to_string(&path);
        .expect("Could not read file");

    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

}