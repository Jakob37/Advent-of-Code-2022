use std::fs;

pub fn load_path(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file from path");

    let rows = contents.split("\n").map(|s| s.to_string()).collect();
    rows       
}

// my_string.split("something").map(|s| s.to_string()).collect();