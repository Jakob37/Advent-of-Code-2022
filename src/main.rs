mod day1;
mod day1_2;
mod day2;
mod day2_2;
mod day3;
mod day3_2;
mod day4;
mod day4_2;

fn main() {
    println!("Hello world!");

    let day1_result: isize = day1::main("data/day1_sample.txt");
    let day1_2_result: isize = day1_2::main("data/day1_sample.txt");
    let day2_result: isize = day2::main("data/day2_sample.txt");
    let day2_2_result: isize = day2_2::main("data/day2_sample.txt");
    // let day3_result = day3::main("data/day3_sample.txt");
    // let day3_2_result = day3_2::main("data/day3_sample.txt");
    // let day4_result = day4::main("data/day4_sample.txt");
    // let day4_2_result = day4_2::main("data/day4_sample.txt");



    println!("Day 1 result: {}", day1_result);
    println!("Day 1.2 result: {}", day1_2_result);

    // day1::test();
}
