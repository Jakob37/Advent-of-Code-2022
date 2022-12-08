pub fn part_two(rows: Vec<String>) {
    let mut row_matrix: Vec<Vec<u32>> = Vec::new();
    let mut nbr_rows = 0;
    for row in rows {
        let mut row_vect: Vec<u32> = Vec::new();
        let row_chars = row.chars();
        for row_char in row_chars {
            let row_nbr = row_char.to_digit(10).unwrap();
            row_vect.push(row_nbr);
        }
        row_matrix.push(row_vect);
        nbr_rows += 1;
    }

    let mut highest_score = 0;
    for i in 0..row_matrix[0].len() {
        for j in 0..nbr_rows {
            let coord = (i, j);
            let curr_score = scenic_score(&row_matrix, coord);
            if curr_score > highest_score {
                highest_score = curr_score;
            }
        }
    }

    println!("Highest {}", highest_score);
}

fn scenic_score(rows: &Vec<Vec<u32>>, coord: (usize, usize)) -> u32 {
    let mut top_view = 0;
    let mut bottom_view = 0;
    let mut left_view = 0;
    let mut right_view = 0;

    let coord_val = rows[coord.0][coord.1];

    let width = rows[0].len();
    let height = rows.len();

    println!("Will iterate {:?}", coord);
    if coord.1 > 0 {
        for col_i in (coord.1 - 1)..0 {
            println!("Iterating at {}", col_i);
            let val = rows[coord.0][col_i];
            if val <= coord_val {
                left_view += 1;
            }

            if val >= coord_val {
                break;
            }
        }
    }

    if coord.1 < width {
        for col_i in coord.1..width {
            let val = rows[coord.0][col_i];
            if val <= coord_val {
                right_view += 1;
            }

            if val >= coord_val {
                break;
            }
        }
    }

    if coord.0 > 0 {
        for row_i in (coord.0 - 1)..0 {
            let val = rows[row_i][coord.1];
            if val <= coord_val {
                top_view += 1;
            }

            if val >= coord_val {
                break;
            }
        }
    }

    if coord.0 < height {
        for row_i in coord.0..height {
            let val = rows[row_i][coord.1];
            if val <= coord_val {
                bottom_view += 1;
            }

            if val >= coord_val {
                break;
            }
        }
    }

    // for row_i in 0..rows.len() {
    //     for col_i in 0..rows[0].len() {

    //     }
    // }
    println!("{} {} {} {}", top_view, bottom_view, left_view, right_view);
    top_view * bottom_view * left_view * right_view
}

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
        let highest_from_left = are_positions_hidden(&row_chars, false);
        let highest_from_right = are_positions_hidden(&row_chars, true);

        visible_left.push(highest_from_left);
        visible_right.push(highest_from_right);
    }

    for col_chars in col_matrix {
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
