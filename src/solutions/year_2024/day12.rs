use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

fn solve_day12() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day12.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visited[y][x] {
                result += solve((y, x), &map, &mut visited);
            }
        }
    }
    Ok(result)
}

fn solve(position: (usize, usize), map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> i64 {
    const DIRECTION: [i64; 5] = [-1, 0, 1, 0, -1];
    let current_char = map[position.0][position.1];
    let mut queue = VecDeque::new();
    queue.push_back(position);

    let mut result = 0;
    let mut count = 0;
    while let Some(next) = queue.pop_front() {
        if visited[next.0][next.1] {
            continue;
        }
        visited[next.0][next.1] = true;
        count += 1;

        for i in 0..4 {
            let next_y = next.0 as i64 + DIRECTION[i];
            let next_x = next.1 as i64 + DIRECTION[i + 1];

            if next_y < 0
                || next_x < 0
                || next_y >= map.len() as i64
                || next_x >= map[0].len() as i64
            {
                // fence
                result += 1;
                continue;
            }
            if map[next_y as usize][next_x as usize] != current_char {
                result += 1;
            } else {
                queue.push_back((next_y as usize, next_x as usize));
            }
        }
    }
    // println!("{}: {} from {:?}", current_char, result * count, position);
    result * count
}

fn solve_day12_2() -> Result<usize, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_1() {
        println!("result: {:?}", solve_day12());
    }

    #[test]
    fn test_day12_2() {
        println!("result: {:?}", solve_day12_2());
    }
}
