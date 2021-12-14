use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day6(n: i32) -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day6.txt")?;
    let fish: Vec<i32> = input.lines().nth(0).unwrap()
        .split(",")
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.parse::<i32>().unwrap()
        }).collect();

    return Ok(solve(fish, n));
}

fn solve(fish: Vec<i32>, n: i32) -> i64 {
    let mut sum = fish.len() as i64;
    let mut dp = HashMap::new();
    for x in fish {
        sum += do_solve(n + 7 - x, &mut dp);
    }
    sum
}

fn do_solve(day: i32, dp: &mut HashMap<i32, i64>) -> i64 {
    if dp.contains_key(&day) {
        return *dp.get(&day).unwrap();
    }
    if day < 7 {
        // println!("dp({})={}", day, 0);
        dp.insert(day, 0);
        return 0;
    }
    let mut result = do_solve(day - 7, dp) + do_solve(day - 9, dp);
    if day > 7 {
        result += 1;
    }
    dp.insert(day, result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day6(80)?);
        Ok(())
    }


    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day6(256)?);
        Ok(())
    }
}
