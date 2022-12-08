pub fn part_one(rows: Vec<String>) {
    let mut visible_left: Vec<Vec<bool>> = Vec::new();
    let mut visible_right: Vec<Vec<bool>> = Vec::new();
    let mut visible_top: Vec<Vec<bool>> = Vec::new();
    let mut visible_bottom: Vec<Vec<bool>> = Vec::new();

    let mut row_matrix: Vec<Vec<char>> = Vec::new();
    let mut col_matrix: Vec<Vec<char>> = Vec::new();

    let mut row_index = 0;
    for row in rows {
        let mut col_index = 0;
        for row_char in row.chars() {
            if row_index == 0 {
                col_matrix.push(Vec::new());
            }
            col_matrix[col_index].push(row_char);

            if col_index == 0 {
                row_matrix.push(Vec::new());
            }

            row_matrix[row_index].push(row_char);

            col_index += 1;
        }
        row_index += 1;
    }

    for row_chars in row_matrix {
        // let row_chars = row[1..row.chars().count() - 1].chars().collect();

        let highest_from_left = are_positions_hidden(&row_chars, false);
        let highest_from_right = are_positions_hidden(&row_chars, true);

        visible_left.push(highest_from_left);
        visible_right.push(highest_from_right);
    }

    for col_chars in col_matrix {
        // let row_chars = row[1..row.chars().count() - 1].chars().collect();

        let highest_from_top = are_positions_hidden(&col_chars, false);
        let highest_from_bottom = are_positions_hidden(&col_chars, true);

        visible_top.push(highest_from_top);
        visible_bottom.push(highest_from_bottom);
    }

    let mut visible_all = 0;
    for i in 0..visible_left.len() {
        for j in 0..visible_left[0].len() {
            let visible_any = visible_left[i][j]
                || visible_right[i][j]
                || visible_top[j][i]
                || visible_bottom[j][i];

            if visible_any {
                visible_all += 1;
            }
        }
    }

    println!("{}", visible_all);
}

fn are_positions_hidden(digit_chars: &Vec<char>, check_reverse: bool) -> Vec<bool> {

    let mut highest: i32 = -1;
    let mut pos_are_hidden: Vec<bool> = Vec::new();

    let check_chars: Vec<char>;
    if check_reverse {
        check_chars = digit_chars.to_vec().into_iter().rev().collect();
    } else {
        check_chars = digit_chars.to_vec();
    }

    for digit_char in check_chars {
        let digit: i32 = digit_char.to_digit(10).unwrap() as i32;
        if digit > highest {
            pos_are_hidden.push(true);
            highest = digit;
        } else {
            pos_are_hidden.push(false);
        }
    }
    if check_reverse {
        pos_are_hidden.reverse();
    }
    pos_are_hidden
}
