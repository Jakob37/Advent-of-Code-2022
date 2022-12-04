use std::fs;

fn main(file_path: &str) -> isize {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("No file found");
    
    let mut sum_score: isize = 0;
    for row in contents.split("\n") {
        let mut split = row.split(' ');
        let their_hand = split.next().unwrap();
        let your_hand = split.next().unwrap();

        let your_hand_transl = if your_hand == "X" {
            "A"
        } else if your_hand == "Y" {
            "B"
        } else {
            "C"
        };

        let score = calculate_score(your_hand_transl, their_hand);
        // println!("Row {} score {}", row, score);
        sum_score += score;
    }
    // println!("Final score: {}", sum_score);
    sum_score
}

fn calculate_score(your_hand: &str, their_hand: &str) -> isize {

    let hand_score = if your_hand == "A" {
        1
    } else if your_hand == "B" {
        2
    } else {
        3
    };

    let is_win = (their_hand == "A" && your_hand == "B") ||
        (their_hand == "B" && your_hand == "C") ||
        (their_hand == "C" && your_hand == "A");

    let win_score = if is_win {
        6
    } else if your_hand == their_hand {
        3
    } else {
        0
    };

    return hand_score + win_score;
}
