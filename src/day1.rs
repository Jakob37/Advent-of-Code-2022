// use std::env;
use std::fs;

pub fn main(file_path: &str) -> isize {

    // let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[1];

    // println!("Read file path {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut max_elf_calories: isize = 0;
    let mut curr_elf_calories = 0;

    for calories_str in contents.split("\n") {

        if calories_str == "" {
            // println!("Done with elf! Calories: {}", curr_elf_calories);
            if curr_elf_calories > max_elf_calories {
                max_elf_calories = curr_elf_calories;
            }
            curr_elf_calories = 0;
        } else {
            // println!("Reading line: {}", calories_str);
            let calories: isize = calories_str.parse().unwrap();
            curr_elf_calories += calories;
        }
    }
 
    max_elf_calories
    // println!("Max calories: {}", max_elf_calories)
    // println!("With text:\n{first_val}");
}