use std::error::Error;
use std::fs;

fn solve_day3(x_pattern: usize, y_pattern: usize) -> Result<i64, Box<dyn Error>> {
    let map_input = fs::read_to_string("src/solutions/day3.txt")?;
    let lines: Vec<&str> = map_input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut map = vec![vec![false; width]; height];
    for (r_i, row) in lines.iter().enumerate() {
        for (c_i, c) in row.chars().enumerate() {
            if c == '#' {
                map[r_i][c_i] = true;
            }
        }
    }
    let mut x = 0;
    let mut y = 0;
    let mut result = 0;
    while y <= height - 1 {
        if map[y][x] {
            result += 1;
        }
        x += x_pattern;
        x %= width;
        y += y_pattern;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day3(3, 1)?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        let patterns = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut result = 1;
        for (x, y) in patterns {
            result *= solve_day3(x, y)?
        }
        println!("result:{}", result);
        Ok(())
    }
}