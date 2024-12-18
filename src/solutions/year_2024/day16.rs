use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Formatter;
use std::io::Write;
use std::{fmt, fs};
use Facing::{Down, Left, Right, Up};

#[derive(Debug, Clone, Copy)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Up => {
                write!(f, "Up")
            }
            Down => {
                write!(f, "Down")
            }
            Left => {
                write!(f, "Left")
            }
            Right => {
                write!(f, "Right")
            }
        }
    }
}

fn solve_day16() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day16.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut start = (-1, -1);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start = (y as i64, x as i64);
                break;
            }
        }
    }

    // key is position and direction
    let mut dp = HashMap::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut result = i64::MAX;
    result = min(result, dfs(start, Right, &map, &mut visited, &mut dp));
    println!("result: {:?}", dp);
    Ok(result)
}

fn dfs(current: (i64, i64), direction: Facing, map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>,
       dp: &mut HashMap<String, i64>) -> i64 {
    // println!("{}: {:?}", direction, current);
    let key = format!("{}-{}-{}", current.0, current.1, direction);
    // if dp.contains_key(&key) {
    //     return dp[&key];
    // }
    if map[current.0 as usize][current.1 as usize] == 'E' {
        return 0;
    }

    let mut result = i64::MAX;
    // Up, Right, Down, Left
    const DIRECTION: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    visited[current.0 as usize][current.1 as usize] = true;
    for i in 0..4 {
        let next = (current.0 + DIRECTION[i].0, current.1 + DIRECTION[i].1);
        if next.0 < 0 || next.1 < 0 || next.0 >= map.len() as i64 || next.1 >= map[0].len() as i64 {
            continue;
        }
        if map[next.0 as usize][next.1 as usize] == '#' {
            continue;
        }
        if visited[next.0 as usize][next.1 as usize] {
            continue;
        }

        // Up
        if i == 0 {
            let next_result = dfs(next, Up, map, visited, dp);
            if next_result == i64::MAX {
                continue;
            }
            if matches!(direction, Up) {
                result = min(result, 1 + next_result);
            } else {
                result = min(result, 1000 + 1 + next_result)
            }
        } else if i == 1 {
            let next_result = dfs(next, Right, map, visited, dp);
            if next_result == i64::MAX {
                continue;
            }
            if matches!(direction, Right) {
                result = min(result, 1 + next_result);
            } else {
                result = min(result, 1000 + 1 + next_result)
            }
        } else if i == 2 {
            let next_result = dfs(next, Down, map, visited, dp);
            if next_result == i64::MAX {
                continue;
            }
            if matches!(direction, Down) {
                result = min(result, 1 + next_result);
            } else {
                result = min(result, 1000 + 1 + next_result)
            }
        } else if i == 3 {
            let next_result = dfs(next, Left, map, visited, dp);
            if next_result == i64::MAX {
                continue;
            }
            if matches!(direction, Left) {
                result = min(result, 1 + next_result);
            } else {
                result = min(result, 1000 + 1 + next_result)
            }
        }
    }
    visited[current.0 as usize][current.1 as usize] = false;
    dp.insert(key, result);
    result
}

fn solve_day16_2() -> i64 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_1() {
        println!("result: {:?}", solve_day16());
    }

    #[test]
    fn test_day16_2() {
        println!("result: {:?}", solve_day16_2());
    }
}
