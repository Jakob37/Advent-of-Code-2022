pub fn part_one(rows: Vec<String>) {
    let mut visible_left: Vec<bool> = Vec::new();
    let mut visible_right: Vec<bool> = Vec::new();

    for row in rows {
        println!("{}", row);

        let row_chars = row[1..row.chars().count() - 1].chars();

        let mut highest_left = 0;
        let mut highest_right = 0;

        for row_char in row_chars {
            let i_digit = row_char.to_digit(10).unwrap();
            println!("{}", i_digit);
            if i_digit > highest_left {
                highest_left = i_digit;
                visible_left.push(true);
            } else {
                visible_left.push(false);
            }
        }

        for row_char in row_chars {
            let i_digit = row_char.to_digit(10).unwrap();
            println!("{}", i_digit);
            if i_digit > highest_left {
                highest_left = i_digit;
                visible_left.push(true);
            } else {
                visible_left.push(false);
            }
        }
    }
}
