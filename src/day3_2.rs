use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("No file found");
    let mut tot_score = 0;

    let contents_iter = contents.split("\n");

    let mut index = 0;

    // let row2 = contents_iter.next();
    // let row3 = contents_iter.next();

    let mut row1 = "";
    let mut row2 = "";

    for row in contents_iter {
        if index % 3 == 0 {
            row1 = row;
        } else if index % 3 == 1 {
            row2 = row;
        } else {
            let row3 = row;

            let shared_char = get_shared_char(row1, row2, row3);
            let char_score = get_char_score(shared_char);
            println!("Shared: {} Score: {}", shared_char, char_score);

            tot_score += char_score;
        }

        index += 1;
    }

    println!("Total score: {}", tot_score);
}

fn get_shared_char(row1: &str, row2: &str, row3: &str) -> char {
    let first_row_set: HashSet<char> = row1.chars().collect();
    let mut first_second_shared = HashSet::new();

    for second_row_char in row2.chars() {
        if first_row_set.contains(&second_row_char) {
            first_second_shared.insert(second_row_char.clone());
        }
    }

    let mut three_shared = '.';
    for third_row_char in row3.chars() {
        if first_second_shared.contains(&third_row_char) {
            // return &third_row_char;
            three_shared = third_row_char;
        }
    }

    return three_shared;
}

fn get_char_score(target_char: char) -> u16 {
    let a_lower = 'a' as u16; // 97
    let a_upper = 'A' as u16; // 65
    let shared_char = target_char as u16;

    let score: u16 = if shared_char > a_lower {
        shared_char - a_lower + 1
    } else {
        shared_char - a_upper + 27
    };

    return score;
}
