use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn solve_day9() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day9.txt")?;
    let mut block = true;
    let mut input_blocks: Vec<String> = Vec::new();
    let mut index = 0;
    for c in input.chars() {
        let count = c.to_digit(10).unwrap();
        if block {
            for _ in 0..count {
                input_blocks.push(index.to_string());
            }
            index += 1;
        } else {
            for _ in 0..count {
                input_blocks.push(".".to_owned());
            }
        }
        block = !block;
    }

    let mut result: Vec<&str> = Vec::new();
    let mut left_index = 0;
    let mut right_index = input_blocks.len() - 1;

    while left_index <= right_index {
        if input_blocks[left_index] != "." {
            result.push(&input_blocks[left_index]);
            left_index += 1;
        } else {
            if input_blocks[right_index] != "." {
                result.push(&input_blocks[right_index]);
                left_index += 1;
            }
            right_index -= 1;
        }
    }

    let mut checksum = 0;
    for i in 0..result.len() {
        checksum += result[i].parse::<i64>().unwrap() * i as i64;
    }
    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_1() {
        println!("result: {:?}", solve_day9());
    }
}
