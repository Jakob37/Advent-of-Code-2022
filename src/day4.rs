use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("No file found");

    let mut nbr_contained = 0;

    for row in contents.split("\n") {
        let ranges = row.split(",").collect::<Vec<&str>>();

        // Calculate ranges (how to put this into a smooth function? objects in Rust?)
        let range1_split = ranges[0].split("-");
        let range2_split = ranges[1].split("-");
        let range1 = range1_split.collect::<Vec<&str>>();
        let range2 = range2_split.collect::<Vec<&str>>();
        let range1_begin = range1[0].parse::<usize>().unwrap();
        let range1_end = range1[1].parse::<usize>().unwrap();
        let range2_begin = range2[0].parse::<usize>().unwrap();
        let range2_end = range2[1].parse::<usize>().unwrap();

        let contained = is_contained(range1_begin, range1_end, range2_begin, range2_end);

        if contained {
            nbr_contained += 1;
        }

        // let range1 = ranges[0].split("-").collect::Vec<&str>>();
        println!(
            "{} {} {} {} {}",
            range1_begin, range1_end, range2_begin, range2_end, contained
        );
    }
    println!("Contained pairs: {}", nbr_contained);
}

fn is_contained(
    range1_begin: usize,
    range1_end: usize,
    range2_begin: usize,
    range2_end: usize,
) -> bool {
    if range1_begin == range2_begin {
        true
    } else if range1_end == range2_end {
        true
    }
    else if range2_begin > range1_begin {
        range2_end <= range1_end
    } else {
        range1_end <= range2_end
    }
}
