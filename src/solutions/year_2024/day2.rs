use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day2.txt")?;
    let numbers: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    for line in numbers {
        let nums: Vec<i64> = line.split(" ").map(|c| { c.parse::<i64>().unwrap() }).collect();
        let success = match pass_the_test(nums) {
            Some(value) => total += 1,
            None => continue,
        };
    }
    Ok(total)
}

fn pass_the_test(nums: Vec<i64>) -> Option<bool> {
    if nums[0] == nums[1] {
        return None;
    }
    let increase;
    if nums[0] > nums[1] {
        increase = false;
    } else {
        increase = true;
    }

    let mut success = true;
    for i in 0..nums.len() - 1 {
        // println!("{} <> {}", nums[i], nums[i + 1]);
        if increase {
            if nums[i] >= nums[i + 1] {
                success = false;
                break;
            }
            let diff = nums[i + 1] - nums[i];
            if diff > 3 || diff < 1 {
                success = false;
                break;
            }
        }
        if !increase {
            if nums[i] <= nums[i + 1] {
                success = false;
                break;
            }
            let diff = nums[i] - nums[i + 1];
            if diff > 3 || diff < 1 {
                success = false;
                break;
            }
        }
    }
    Some(success)
}


fn solve_day2_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day2.txt")?;
    let numbers: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    for line in numbers {
        let nums: Vec<i64> = line.split(" ").map(|c| { c.parse::<i64>().unwrap() }).collect();
        for i in 0..nums.len() {
            let new_one: Vec<i64> = nums.iter().enumerate().filter_map(|(index, v)| {
                if index != i {
                    Some(*v)
                } else {
                    None
                }
            }).collect();
            match pass_the_test(new_one) {
                None => {}
                Some(success) => {
                    if success {
                        total += 1;
                        break;
                    }
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day2()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day2_2()?);
        Ok(())
    }
}
