use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day4() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day4.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            total += dfs(&map, i as i64, j as i64, 0, 0);
        }
    }
    Ok(total)
}

const WORDS: [char; 4] = ['X', 'M', 'A', 'S'];

fn dfs(map: &Vec<Vec<char>>, x: i64, y: i64, c_index: usize, direction: usize) -> i64 {
    if x < 0 || y < 0 || x >= map.len() as i64 || y >= map[0].len() as i64 {
        return 0;
    }

    if map[x as usize][y as usize] != WORDS[c_index] {
        return 0;
    }

    if c_index == 3 {
        return 1;
    }
    let mut total = 0;
    const DIRECTIONS: [(i64, i64); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

    if c_index == 0 {
        for (index, offset) in DIRECTIONS.iter().enumerate() {
            total += dfs(map, x + offset.0, y + offset.1, c_index + 1, index);
        }
    } else {
        total += dfs(map, x + DIRECTIONS[direction].0, y + DIRECTIONS[direction].1, c_index + 1, direction);
    }
    total
}


fn solve_day4_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day4.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            total += find_x_mas(&map, i as i64, j as i64);
        }
    }
    Ok(total)
}

fn find_x_mas(map: &Vec<Vec<char>>, x: i64, y: i64) -> i64 {
    if map[x as usize][y as usize] != 'A' {
        return 0;
    }

    // Left-Up
    if x - 1 < 0 || y - 1 < 0 {
        return 0;
    }

    if map[(x - 1) as usize][(y - 1) as usize] != 'M' && map[(x - 1) as usize][(y - 1) as usize] != 'S' {
        return 0;
    } else {
        // Right-Down
        if x + 1 >= map.len() as i64 || y + 1 >= map[0].len() as i64 {
            return 0;
        }

        if map[(x - 1) as usize][(y - 1) as usize] == 'M' && map[(x + 1) as usize][(y + 1) as usize] != 'S' {
            return 0;
        }

        if map[(x - 1) as usize][(y - 1) as usize] == 'S' && map[(x + 1) as usize][(y + 1) as usize] != 'M' {
            return 0;
        }
    }

    // Right-Up
    if x + 1 >= map.len() as i64 || y - 1 < 0 {
        return 0;
    }

    if map[(x + 1) as usize][(y - 1) as usize] != 'M' && map[(x + 1) as usize][(y - 1) as usize] != 'S' {
        return 0;
    } else {
        // Left-Down
        if x - 1 < 0 || y + 1 >= map[0].len() as i64 {
            return 0;
        }

        if map[(x + 1) as usize][(y - 1) as usize] == 'M' && map[(x - 1) as usize][(y + 1) as usize] != 'S' {
            return 0;
        }

        if map[(x + 1) as usize][(y - 1) as usize] == 'S' && map[(x - 1) as usize][(y + 1) as usize] != 'M' {
            return 0;
        }
    }
    1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day4()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day4_2()?);
        Ok(())
    }
}
