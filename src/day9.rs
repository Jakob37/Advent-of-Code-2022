use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Coord(i32, i32);

enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

// struct WalkStepsResult {
//     position: Coord,
//     steps: Vec<Coord>
// }

pub fn part_one(rows: Vec<String>, rope_len: u32) {
    let mut head_position: Coord = Coord(0, 0);
    let mut tail_position: Coord = Coord(0, 0);
    let mut passed_positions: Vec<Coord> = Vec::new();
    let mut passed_tail_positions: Vec<Coord> = Vec::new();

    for row in rows {
        let str_nbrs = row.split_once(" ").map(|v| v).unwrap();
        let dir: char = str_nbrs.0.chars().next().unwrap();
        let dir_enum = get_direction_enum(dir);
        let steps: u32 = str_nbrs.1.parse().unwrap();
        // println!("{} {}", dir, steps);
        // let nbr1  =
        let new_steps = walk_steps(head_position.clone(), dir_enum, steps);

        let last = new_steps.last().unwrap();
        head_position = Coord(last.0, last.1);

        for new_step in new_steps {
            // head_position = Coord(new_step.0, new_step.1);

            tail_position = move_tail(new_step, tail_position, rope_len);

            passed_positions.push(new_step);
            passed_tail_positions.push(tail_position.clone());
        }
    }

    // println!("{:?}", passed_positions);
    // println!("{:?}", passed_tail_positions);

    let mut unique_positions:HashSet<String> = HashSet::new();

    for passed_tail_pos in passed_tail_positions {
        let x_pos = passed_tail_pos.0;
        let y_pos = passed_tail_pos.1;
        let tail_pos_str = format!("{x_pos},{y_pos}");
        unique_positions.insert(tail_pos_str);
    }

    println!("{:?}", unique_positions);
    println!("Size: {}", unique_positions.len());

}

fn get_direction_enum(dir_char: char) -> Direction {
    match dir_char {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => Direction::None,
    }
}

fn walk_steps(head_start: Coord, direction: Direction, nbr_steps: u32) -> Vec<Coord> {
    let mut steps: Vec<Coord> = Vec::new();

    let step = match direction {
        Direction::Down => Coord(0, -1),
        Direction::Up => Coord(0, 1),
        Direction::Left => Coord(-1, 0),
        Direction::Right => Coord(1, 0),
        Direction::None => Coord(0, 0),
    };

    let mut curr_pos = Coord(head_start.0, head_start.1);
    for _ in 0..nbr_steps {
        curr_pos = Coord(curr_pos.0 + step.0, curr_pos.1 + step.1);
        steps.push(Coord(curr_pos.0, curr_pos.1));
    }

    steps
}

fn move_tail(moved_head: Coord, tail_start: Coord, rope_len: u32) -> Coord {
    let x_dist: u32 = (moved_head.0 - tail_start.0).abs() as u32;
    let y_dist: u32 = (moved_head.1 - tail_start.1).abs() as u32;

    let max_diff= if x_dist >= y_dist { x_dist } else { y_dist };

    if max_diff > rope_len {
        if x_dist > y_dist {
            if moved_head.0 > tail_start.0 {
                Coord(moved_head.0 - 1, moved_head.1)
            } else {
                Coord(moved_head.0 + 1, moved_head.1)
            }
        } else {
            if moved_head.1 > tail_start.1 {
                Coord(moved_head.0, moved_head.1 - 1)
            } else {
                Coord(moved_head.0, moved_head.1 + 1)
            }
        }
    } else {
        Coord(tail_start.0, tail_start.1)
    }
}
