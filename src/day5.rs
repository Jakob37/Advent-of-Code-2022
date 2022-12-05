pub fn part_one(rows: Vec<String>) -> isize {
    let mut stacks_done = false;

    let mut crates: Vec<Vec<String>> = Vec::new();

    for row in rows {
        if row == "" {
            println!("Divider!");

            stacks_done = true;
        } else {
            if stacks_done {
                let move_size = &row[5..6];
                let move_start = &row[12..13];
                let move_end = &row[17..18];

                println!("Move {} {} {}", move_size, move_start, move_end);
            } else {
                // println!("Stack {}", row);

                let mut pos = 0;
                for part in row.split(" ") {
    
                    // We are only interested in the [A] format
                    if part.len() == 3 {
                        if pos > crates.len() {
                            crates.push(Vec::new());
                        }

                        let crate_letter = &part[1..2];

                        crates[pos].push(crate_letter.to_string());
        
                        println!("Part: {} at pos {}", crate_letter, pos);
                    }

    
                    pos += 1;
                }
    
            }
        }
    }

    0
}

fn main() {
    println!("Main");
}
