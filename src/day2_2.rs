use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("No file found");
    
    let mut sum_score = 0;
    for row in contents.split("\n") {
        let mut split = row.split(' ');
        let their_hand = split.next().unwrap();
        let expected_outcome = split.next().unwrap();

        // let your_hand_transl = if your_hand == "X" {
        //     "A"
        // } else if your_hand == "Y" {
        //     "B"
        // } else {
        //     "C"
        // };

        let score = calculate_score(expected_outcome, their_hand);
        // println!("Row {} score {}", row, score);
        sum_score += score;
    }
    println!("Final score: {}", sum_score);
}

// This is kind off brute-force, I realize that an array and the modal
// operator could solve this more elegantly
fn calculate_score(expected_outcome: &str, their_hand: &str) -> i32 {

    let used_hand;

    let win_score = if expected_outcome == "X" {
        used_hand = if their_hand == "A" {
            "C"
        } else if their_hand == "B" {
            "A"
        } else {
            "B"
        };
        0
    } else if expected_outcome == "Y" {
        used_hand = their_hand;
        3
    } else {
        used_hand = if their_hand == "A" {
            "B"
        } else if their_hand == "B" {
            "C"
        } else {
            "A"
        };
        6
    };

    let hand_score = if used_hand == "A" {
        1
    } else if used_hand == "B" {
        2
    } else {
        3
    };

    // println!("hand_score {} win_score {}", hand_score, win_score);
    return hand_score + win_score;
}
