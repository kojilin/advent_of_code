use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day1.txt")?;
    let numbers: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut lefts: Vec<i64> = Vec::new();
    let mut rights: Vec<i64> = Vec::new();
    for line in numbers {
        let nums: Vec<i64> = line.split("   ").map(|c| { c.parse::<i64>().unwrap() }).collect();
        lefts.push(nums[0]);
        rights.push(nums[1]);
    }
    lefts.sort();
    rights.sort();
    for _ in 0..lefts.len() {
        let left = lefts.pop().unwrap();
        let right = rights.pop().unwrap();
        let diff = left.sub(right).abs();
        // println!("{}-{}={}", left, right, diff);
        total += diff;
    }
    Ok(total)
}

fn solve_day1_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day1.txt")?;
    let numbers: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut lefts: Vec<i64> = Vec::new();
    let mut rights: HashMap<i64, i64> = HashMap::new();
    for line in numbers {
        let nums: Vec<i64> = line.split("   ").map(|c| { c.parse::<i64>().unwrap() }).collect();
        lefts.push(nums[0]);
        rights.entry(nums[1])
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }
    lefts.sort();
    for _ in 0..lefts.len() {
        let left = lefts.pop().unwrap();
        let right = *rights.get(&left).unwrap_or(&0);
        let diff = left.mul(right);
        // println!("{}-{}={}", left, right, diff);
        total += diff;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day1_2()?);
        Ok(())
    }
}
