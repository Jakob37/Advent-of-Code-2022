use std::collections::HashSet;
// use std::env;
use std::fs;

fn main(file_path: &str) {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("No file found");
    let mut tot_score = 0;

    for backpack in contents.split("\n") {
        let backpack_length = backpack.chars().count() / 2;

        let first_backpack = &backpack[..backpack_length];
        let second_backpack = &backpack[backpack_length..];

        let first_set: HashSet<char> = first_backpack.chars().collect();

        let mut shared: char = '.';
        for second_char in second_backpack.chars() {
            if first_set.contains(&second_char) {
                shared = second_char;
                break;
            }
        }

        let a_lower = 'a' as u16; // 97
        let a_upper = 'A' as u16; // 65
        let shared_char = shared as u16;

        let score: u16 = if shared_char > a_lower {
            shared_char - a_lower + 1
        } else {
            shared_char - a_upper + 27
        };

        println!(
            "Row: {} length: {} bp1 {} bp2 {} shared {} score {}",
            backpack, backpack_length, first_backpack, second_backpack, shared, score
        );

        tot_score += score;
    }

    println!("Total score: {}", tot_score);
}
