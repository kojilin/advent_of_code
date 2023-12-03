use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::Add;

fn solve_day3() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day3.txt")?;
    let map: Vec<Vec<char>> = content.lines().map(|line| line.trim().chars().collect()).collect();

    let mut result = 0;
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for y in 0..map.len() {
        let mut visited_row = Vec::new();
        for _ in 0..map[y].len() {
            visited_row.push(false);
        }
        visited.push(visited_row);
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            result += calculate(y, x, &map, &mut visited);
        }
    }
    Ok(result)
}

fn calculate(y: usize, x: usize, map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> i64 {
    if map[y][x] == '.' {
        return 0;
    }
    if !map[y][x].is_ascii_digit() {
        return 0;
    }
    if visited[y][x] {
        return 0;
    }
    visited[y][x] = true;

    let is_near_symbol = near_symbol(y, x, map);

    trace_number(map[y][x].to_string(), y, x + 1, map, visited, is_near_symbol)
}

fn near_symbol(y: usize, x: usize, map: &Vec<Vec<char>>) -> bool {
    let directions: [[i32; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1],
        [0, -1], [0, 1],
        [1, -1], [1, 0], [1, 1]];
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    for direction in directions {
        let new_y = y as i32 + direction[0];
        let new_x = x as i32 + direction[1];
        if (new_y < height && new_y >= 0 && new_x >= 0 && new_x < width) {
            if (!map[new_y as usize][new_x as usize].is_ascii_digit()
                && map[new_y as usize][new_x as usize] != '.') {
                return true;
            }
        }
    }
    false
}

fn trace_number(mut current: String, y: usize, x: usize, map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, is_near_symbol: bool) -> i64 {
    let width = map[0].len();
    if (x >= width || !map[y][x].is_ascii_digit()) {
        return if is_near_symbol { current.parse::<i64>().unwrap() } else { 0 };
    }
    visited[y][x] = true;
    if (map[y][x].is_ascii_digit()) {
        let is_near_symbol = is_near_symbol || near_symbol(y, x, map);
        return trace_number(current.add(&map[y][x].to_string()), y, x + 1, map, visited, is_near_symbol);
    }
    0
}

fn solve_day3_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day3.txt")?;
    let map: Vec<Vec<char>> = content.lines().map(|line| line.trim().chars().collect()).collect();

    let mut result = 0;
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for y in 0..map.len() {
        let mut visited_row = Vec::new();
        for _ in 0..map[y].len() {
            visited_row.push(false);
        }
        visited.push(visited_row);
    }

    let mut symbol_value = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            result += calculate2(y, x, &map, &mut visited, &mut symbol_value);
        }
    }
    Ok(result)
}

fn calculate2(y: usize, x: usize, map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>,
              symbol_value: &mut HashMap<(usize, usize), i64>) -> i64 {
    if map[y][x] == '.' {
        return 0;
    }
    if !map[y][x].is_ascii_digit() {
        return 0;
    }
    if visited[y][x] {
        return 0;
    }
    visited[y][x] = true;

    let is_near_symbol = near_symbol2(y, x, map);

    trace_number2(map[y][x].to_string(), y, x + 1, map, visited, is_near_symbol, symbol_value)
}

fn near_symbol2(y: usize, x: usize, map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let directions: [[i32; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1],
        [0, -1], [0, 1],
        [1, -1], [1, 0], [1, 1]];
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    for direction in directions {
        let new_y = y as i32 + direction[0];
        let new_x = x as i32 + direction[1];
        if (new_y < height && new_y >= 0 && new_x >= 0 && new_x < width) {
            if (!map[new_y as usize][new_x as usize].is_ascii_digit()
                && map[new_y as usize][new_x as usize] == '*') {
                return Some((new_y as usize, new_x as usize));
            }
        }
    }
    None
}

fn trace_number2(mut current: String, y: usize, x: usize, map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>,
                 is_near_symbol: Option<(usize, usize)>,
                 symbol_value: &mut HashMap<(usize, usize), i64>) -> i64 {
    let width = map[0].len();
    if (x >= width || !map[y][x].is_ascii_digit()) {
        return if let Some(symbol_position) = is_near_symbol {
            if let Some(value) = symbol_value.get(&symbol_position) {
                *value * current.parse::<i64>().unwrap()
            } else {
                symbol_value.insert(symbol_position, current.parse::<i64>().unwrap());
                0
            }
        } else {
            0
        };
    }
    visited[y][x] = true;
    if (map[y][x].is_ascii_digit()) {
        let is_near_symbol = if is_near_symbol == None { near_symbol2(y, x, map) } else { is_near_symbol };
        return trace_number2(current.add(&map[y][x].to_string()), y, x + 1, map, visited, is_near_symbol, symbol_value);
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day3()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day3_2()?);
        Ok(())
    }
}