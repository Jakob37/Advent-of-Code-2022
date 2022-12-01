use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    let file_path = &args[1];

    println!("Read file path {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for val in contents.split("\n") {
        println!("Reading line: {}", val);
    }
    // let first_val = split_content[0];

    // println!("With text:\n{first_val}");
}