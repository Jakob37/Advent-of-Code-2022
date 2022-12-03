use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("No file found");
    
    for row in contents.split("\n") {
        let (their_hand, your_hand) = row.split_at(1);

        let your_hand_transl = if your_hand == "X" {
            "A"
        } else if your_hand == "Y" {
            "B"
        } else {
            "C"
        };

        let score = calculate_score(your_hand_transl, their_hand);
        // let score = 5;
        println!("Row {} score {}", row, score);
    }
}

fn calculate_score(your_hand: &str, their_hand: &str) -> i32 {

    if your_hand == their_hand {
        1
    } else {
        0
    }
}
