use std::cmp::{max, min};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use _2019::int_code_computer_robot::IntCodeComputerRobot;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day11/bin/day11.txt")?;
    let input_codes: Vec<i64> = input.trim().split(",").map(|c| { c.parse::<i64>().unwrap() }).collect();
    let mut computer = IntCodeComputerRobot::new(&input_codes);

    // x, y, color(1(w),0(b))
    let mut floor: HashMap<(i64, i64), i32> = HashMap::new();
    let mut current_position = (0 as i64, 0 as i64);
    let mut direction = 0; // 0, 1, 2, 3 => U, R, D, L
    floor.insert((0, 0), 1);
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    while !computer.is_halt() {
        let x = floor.entry(current_position).or_insert(0);
        let vec = computer.compute(x.clone() as i64, 2);
        if vec.is_empty() {
            // last floor.
            floor.remove(&current_position);
        } else {
            *x = vec.get(0).cloned().unwrap() as i32;
            if vec.get(1).cloned().unwrap() == 1 {
                //right
                direction = (direction + 1) % 4;
            } else {
                direction = if direction == 0 {
                    direction = 3;
                    direction
                } else {
                    direction -= 1;
                    direction
                };
            }
            match direction {
                0 => current_position = (current_position.0, current_position.1 + 1),
                1 => current_position = (current_position.0 + 1, current_position.1),
                2 => current_position = (current_position.0, current_position.1 - 1),
                3 => current_position = (current_position.0 - 1, current_position.1),
                _ => panic!("wrong direction.")
            }
        }
        min_x = min(min_x, current_position.0);
        min_y = min(min_y, current_position.1);
        max_x = max(max_x, current_position.0);
        max_y = max(max_y, current_position.1);
    }
    for i in (min_y..=max_y).rev() {
        for j in min_x..=max_x {
            let color = floor.get(&(j, i)).unwrap_or(&0);
            if *color == 0 {
                print!("{}", "#");
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }
    Ok(())
}
