use std::collections::HashSet;

pub fn part_one(rows: Vec<String>) {

    let window_size = 4;

    for row in rows {
        let found_pos = find_non_unique_pos(row, window_size);

        println!("Found pos: {}", found_pos);
    }
}

fn find_non_unique_pos(row: String, window_size: usize) -> usize {
    let row_chars = row.chars();
    let end_iter_pos: usize = row_chars.count() - window_size;
    let mut found_pos = 0;
    for pos in 0..end_iter_pos {

        let mut encountered_chars: HashSet<char> = HashSet::new();
        let curr_slice: &str = &row[pos..pos+window_size];
        for my_char in curr_slice.chars() {
            if !encountered_chars.contains(&my_char) {
                encountered_chars.insert(my_char.clone());
                // println!("{:?}", encountered_chars);
            } else {
                // FIXME
                break;
            }
        }

        if encountered_chars.len() == window_size {
            found_pos = pos + window_size;
            break;
        }
    }
    found_pos
}