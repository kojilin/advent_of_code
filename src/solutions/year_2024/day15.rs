use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::ops::Index;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_1() {
        println!("result: {:?}", solve_day15());
    }

    #[test]
    fn test_day15_2() {
        println!("result: {:?}", solve_day15());
    }
}
