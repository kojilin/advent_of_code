use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use _2019::int_code_computer_robot::IntCodeComputerRobot;

///     1
///    3 4
///     2
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day15/bin/day15.txt")?;
    let input_codes: Vec<i64> = input.trim().split(",").map(|c| { c.parse::<i64>().unwrap() }).collect();

    let mut computer = IntCodeComputerRobot::new(&input_codes);
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    // build map
    dfs(&mut map, &mut computer, -1, (0, 0));
    // shortest path
    println!("{}", bfs(&map, (0, 0)));
    map.insert((0, 0), 1);
    let result: Vec<(&(i64, i64), &i64)> = map.iter().filter(|x| { x.1.clone() == 2 }).collect();
    let from = result.get(0).unwrap();
    let from = from.0;

    println!("{}", bfs(&map, (from.0, from.1)));
    Ok(())
}

fn bfs(map: &HashMap<(i64, i64), i64>, from: (i64, i64)) -> i32 {
    let mut movements: HashMap<i64, (i64, i64)> = HashMap::new();
    movements.insert(1, (0, 1));
    movements.insert(2, (0, -1));
    movements.insert(3, (-1, 0));
    movements.insert(4, (1, 0));
    // x, y, from, step
    let mut queue: Vec<(i64, i64, i32, i32)> = vec![];
    let mut max_step = 0;
    queue.push((from.0, from.1, -1, 0));
    while !queue.is_empty() {
        let current_pos = queue.remove(0);
        max_step = max(max_step, current_pos.3);
        for i in 1..=4 as i64 {
            if i as i32 == current_pos.2 {
                continue;
            }
            let new_x = current_pos.0 + movements.get(&i).unwrap().0;
            let new_y = current_pos.1 + movements.get(&i).unwrap().1;
            let status = map.get(&(new_x, new_y)).unwrap().clone();
            if status == 1 {
                queue.push((new_x, new_y, reverse(i as i64) as i32, current_pos.3 + 1));
            } else if status == 2 {
                return current_pos.3 + 1;
            }
        }
    }
    max_step
}

fn dfs(map: &mut HashMap<(i64, i64), i64>, computer: &mut IntCodeComputerRobot, from: i64, position: (i64, i64)) {
    for i in 1..=4 {
        if i != from {
            let new_position = to(position, i);
            let status = computer.compute(i as i64, 1).get(0).unwrap().clone();
            map.insert(new_position, status);
            if status != 0 {
                dfs(map, computer, reverse(i), new_position);
                // should be pass
                computer.compute(reverse(i), 1);
            }
        }
    }
}

fn to(position: (i64, i64), direction: i64) -> (i64, i64) {
    match direction {
        1 => (position.0, position.1 + 1),
        2 => (position.0, position.1 - 1),
        3 => (position.0 - 1, position.1),
        4 => (position.0 + 1, position.1),
        _ => panic!("error")
    }
}

fn reverse(direction: i64) -> i64 {
    match direction {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("error")
    }
}

