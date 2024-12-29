use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day25() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day25.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut index = 0;
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    while index < lines.len() {
        let mut is_lock = false;
        let mut height = vec![0; 5];
        if lines[index] == "#####" {
            is_lock = true;
        }
        for i in 1..6 {
            for (index, c) in lines[index + i].chars().enumerate() {
                if c == '#' {
                    height[index] += 1;
                }
            }
        }
        if is_lock {
            locks.push(height);
        } else {
            keys.push(height);
        }
        index += 8;
    }
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            let mut all_match = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                count += 1;
            }
        }
    }
    // println!("{:?}", locks);
    // println!("{:?}", keys);
    Ok(count)
}

fn solve_day25_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day25.txt")?;

    Ok(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day25()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day25_2()?);
        Ok(())
    }
}
