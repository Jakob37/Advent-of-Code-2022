use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Read file path {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut max_elf_calories: [i32; 3] = [0, 0, 0];
    let mut curr_elf_calories = 0;

    for calories_str in contents.split("\n") {

        if calories_str == "" {
            println!("Done with elf! Calories: {}", curr_elf_calories);
            if curr_elf_calories > max_elf_calories[0] {
                max_elf_calories[0] = curr_elf_calories;
                max_elf_calories.sort();
            }
            curr_elf_calories = 0;
        } else {
            println!("Reading line: {}", calories_str);
            let calories: i32 = calories_str.parse().unwrap();
            curr_elf_calories += calories;
        }
    }
 
    let summed_max_calories = max_elf_calories[0] + max_elf_calories[1] + max_elf_calories[2];

    println!("Max individual calories {} {} {}", max_elf_calories[0], max_elf_calories[1], max_elf_calories[2]);
    println!("Max sum calories: {}", summed_max_calories);
}