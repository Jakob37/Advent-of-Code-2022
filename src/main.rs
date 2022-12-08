// mod day1;
// mod day1_2;
// mod day2;
// mod day2_2;
// mod day3;
// mod day3_2;
// mod day4;
// mod day4_2;
// mod day5;
// mod day6;
mod day7;

mod util;

fn main() {
    // let day1_result: isize = day1::main("data/day1_sample.txt");
    // let day1_2_result: isize = day1_2::main("data/day1_sample.txt");
    // let day2_result: isize = day2::main("data/day2_sample.txt");
    // let day2_2_result: isize = day2_2::main("data/day2_sample.txt");
    // let day3_result = day3::main("data/day3_sample.txt");
    // let day3_2_result = day3_2::main("data/day3_sample.txt");
    // let day4_result = day4::main("data/day4_sample.txt");
    // let day4_2_result = day4_2::main("data/day4_sample.txt");
    // let day5_result: isize = day5::part_one(util::load_path("data/day5_sample.txt"));

    // day6::part_one(util::load_path("data/day6_full.txt"));
    // day6::part_two(util::load_path("data/day6_full.txt"));

    day7::part_one(util::load_path("data/day7_sample.txt"));    
    day7::part_one(util::load_path("data/day7_full.txt"));    
    day7::part_two(util::load_path("data/day7_sample.txt"));    
    // day7::part_two(util::load_path("data/day7_full.txt"));    


    // println!("Day 6 result: {}", day6_1_result);

    // println!("Day 5 result: {}", day5_result);


    // println!("Day 1 result: {}", day1_result);
    // println!("Day 1.2 result: {}", day1_2_result);

    // day1::test();
}
