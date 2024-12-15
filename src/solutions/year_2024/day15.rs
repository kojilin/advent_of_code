use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::ops::Index;
use std::time::Duration;
use std::{fs, thread};

fn solve_day15() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day15.txt")?;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut commands: Vec<char> = Vec::new();
    let mut map_mode = true;
    let mut robot = (-1, -1);
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            map_mode = false;
            continue;
        }
        let mut row: Vec<char> = line.chars().collect();
        if map_mode {
            if row.contains(&'@') {
                robot = (y as i64, row.iter().position(|x| *x == '@').unwrap() as i64);
            }
            map.push(row);
        } else {
            commands.append(&mut row);
        }
    }
    for c in commands {
        match c {
            '^' => {
                try_move(&mut map, &mut robot, (-1, 0));
            }
            '>' => {
                try_move(&mut map, &mut robot, (0, 1));
            }
            'v' => {
                try_move(&mut map, &mut robot, (1, 0));
            }
            '<' => {
                try_move(&mut map, &mut robot, (0, -1));
            }
            _ => {}
        }
        // println!("Move {}:", c);
        // for y in 0..map.len() {
        //     for x in 0..map[y].len() {
        //         print!("{}", map[y][x]);
        //     }
        //     println!();
        // }
    }
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                result += 100 * y + x;
            }
            print!("{}", map[y][x]);
        }
        println!();
    }
    Ok(result as i64)
}

fn try_move(map: &mut Vec<Vec<char>>, robot: &mut (i64, i64), direction: (i64, i64)) {
    let mut next = (robot.0 + direction.0, robot.1 + direction.1);
    // just move
    if map[next.0 as usize][next.1 as usize] == '.' {
        map[robot.0 as usize][robot.1 as usize] = '.';
        robot.0 = next.0;
        robot.1 = next.1;
        map[robot.0 as usize][robot.1 as usize] = '@';
        return;
    }

    // fund if it's blocking or empty
    // found the last . and skip many box
    loop {
        if map[next.0 as usize][next.1 as usize] == '#' {
            // can't move, just return
            return;
        }
        if map[next.0 as usize][next.1 as usize] == '.' {
            break;
        }
        next = (next.0 + direction.0, next.1 + direction.1);
    }

    // leave original place of roboto
    map[robot.0 as usize][robot.1 as usize] = '.';
    // move robot one step
    robot.0 = robot.0 + direction.0;
    robot.1 = robot.1 + direction.1;
    map[robot.0 as usize][robot.1 as usize] = '@';
    // move box to final empty
    map[next.0 as usize][next.1 as usize] = 'O';
}

fn solve_day15_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day15.txt")?;
    let mut commands: Vec<char> = Vec::new();
    let mut map_mode = true;
    let mut robot = (-1, -1);
    let mut blocks = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    // We just store left part, we know it's 2 colum width
    let mut boxes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            height = y;
            map_mode = false;
            continue;
        }
        let mut row: Vec<char> = line.chars().collect();
        if map_mode {
            width = line.len() * 2;
            for (x, &c) in row.iter().enumerate() {
                if c == '#' {
                    blocks.insert((y, x * 2));
                    blocks.insert((y, x * 2 + 1));
                } else if c == '@' {
                    robot = (y as i64, (x * 2) as i64);
                } else if c == 'O' {
                    boxes.insert((y, x * 2));
                } else {
                    // we don't care blank
                }
            }
        } else {
            commands.append(&mut row);
        }
    }
    debug_map(&mut robot, &mut blocks, &mut width, &mut height, &mut boxes);

    for c in commands {
        match c {
            '^' => {
                try_move_2(&mut boxes, &mut robot, (-1, 0), &blocks);
            }
            '>' => {
                try_move_2(&mut boxes, &mut robot, (0, 1), &blocks);
            }
            'v' => {
                try_move_2(&mut boxes, &mut robot, (1, 0), &blocks);
            }
            '<' => {
                try_move_2(&mut boxes, &mut robot, (0, -1), &blocks);
            }
            _ => {}
        }
        // println!("Move {}:", c);
        // debug_map(&mut robot, &mut blocks, &mut width, &mut height, &mut boxes);
    }
    let mut result = 0;

    for (y, x) in boxes {
        result += y * 100 + x;
    }

    Ok(result as i64)
}

fn debug_map(
    robot: &mut (i64, i64),
    blocks: &mut HashSet<(usize, usize)>,
    width: &mut usize,
    height: &mut usize,
    boxes: &mut HashSet<(usize, usize)>,
) {
    let mut skip = false;
    for y in 0..*height {
        for x in 0..*width {
            if skip {
                skip = false;
                continue;
            }
            if boxes.contains(&(y, x)) {
                print!("[]");
                skip = true;
            } else if blocks.contains(&(y, x)) {
                print!("#");
            } else if robot.0 == y as i64 && robot.1 == x as i64 {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn try_move_2(
    boxes: &mut HashSet<(usize, usize)>,
    robot: &mut (i64, i64),
    direction: (i64, i64),
    blocks: &HashSet<(usize, usize)>,
) {
    let mut next = (robot.0 + direction.0, robot.1 + direction.1);
    if is_empty(next, blocks, boxes, direction) {
        robot.0 = next.0;
        robot.1 = next.1;
        return;
    }
    if is_block(next, blocks) {
        return;
    }

    let mut boxes_at_next = is_box(next, boxes, direction);
    let mut queue = VecDeque::new();
    for box_at_next in boxes_at_next {
        queue.push_back((box_at_next.0 as i64, box_at_next.1 as i64));
    }

    let mut need_to_move = HashSet::new();
    while !queue.is_empty() {
        let position_of_box = queue.pop_front().unwrap();
        need_to_move.insert(position_of_box);
        match direction {
            (1, 0) | (-1, 0) => {
                for i in 0..=1 {
                    let next = (
                        position_of_box.0 + direction.0,
                        position_of_box.1 + i + direction.1,
                    );
                    if is_block(next, blocks) {
                        return;
                    }
                    let next_boxes = is_box(next, boxes, direction);
                    if !next_boxes.is_empty() {
                        for box_at_next in next_boxes {
                            queue.push_back((box_at_next.0 as i64, box_at_next.1 as i64));
                        }
                    }
                }
            }
            (0, 1) | (0, -1) => {
                let next = (
                    position_of_box.0 + direction.0,
                    position_of_box.1 + (if direction.1 == 1 { 2 } else { -1 }),
                );
                if is_block(next, blocks) {
                    return;
                }
                let next_boxes = is_box(next, boxes, direction);
                if !next_boxes.is_empty() {
                    for box_at_next in next_boxes {
                        queue.push_back((box_at_next.0 as i64, box_at_next.1 as i64));
                    }
                }
            }
            (_, _) => {}
        }
    }

    for (y, x) in &need_to_move {
        boxes.remove(&(*y as usize, *x as usize));
    }
    for (y, x) in &need_to_move {
        boxes.insert(((*y + direction.0) as usize, (*x + direction.1) as usize));
    }
    //move robot
    robot.0 = robot.0 + direction.0;
    robot.1 = robot.1 + direction.1;
}

fn is_empty(
    target: (i64, i64),
    blocks: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
    direction: (i64, i64),
) -> bool {
    if is_block(target, blocks) {
        return false;
    }
    if !is_box(target, boxes, direction).is_empty() {
        return false;
    }
    true
}

// Also return boxes
fn is_box(
    target: (i64, i64),
    boxes: &HashSet<(usize, usize)>,
    direction: (i64, i64),
) -> Vec<(usize, usize)> {
    let mut founds = Vec::new();
    match direction {
        (-1, 0) => {
            if boxes.contains(&(target.0 as usize, target.1 as usize)) {
                founds.push((target.0 as usize, target.1 as usize));
            }

            if boxes.contains(&(target.0 as usize, (target.1 - 1) as usize)) {
                founds.push((target.0 as usize, (target.1 - 1) as usize));
            }
        }
        (0, 1) => {
            if boxes.contains(&(target.0 as usize, target.1 as usize)) {
                founds.push((target.0 as usize, target.1 as usize));
            }
        }
        (1, 0) => {
            if boxes.contains(&(target.0 as usize, target.1 as usize)) {
                founds.push((target.0 as usize, target.1 as usize));
            }

            if boxes.contains(&(target.0 as usize, (target.1 - 1) as usize)) {
                founds.push((target.0 as usize, (target.1 - 1) as usize));
            }
        }
        (0, -1) => {
            if boxes.contains(&(target.0 as usize, (target.1 - 1) as usize)) {
                founds.push((target.0 as usize, (target.1 - 1) as usize));
            }
        }
        (_, _) => {}
    }
    founds
}

fn is_block(target: (i64, i64), blocks: &HashSet<(usize, usize)>) -> bool {
    return blocks.contains(&(target.0 as usize, target.1 as usize));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_1() {
        println!("result: {:?}", solve_day15());
    }

    #[test]
    fn test_day15_2() {
        println!("result: {:?}", solve_day15_2());
    }
}
