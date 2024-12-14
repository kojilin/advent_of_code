use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day11() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day11.txt")?;
    let mut nums: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..25 {
        nums = nums
            .into_iter()
            .flat_map(|num| {
                let num_str = num.to_string();
                if num == 0 {
                    vec![1]
                } else if num_str.len() % 2 == 0 {
                    let (left, right) = num_str.split_at(num_str.len() / 2);
                    vec![left.parse().unwrap(), right.parse().unwrap()]
                } else {
                    vec![num * 2024]
                }
            })
            .collect();
    }

    Ok(nums.len())
}

fn solve_day11_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day11.txt")?;
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();
    Ok(nums.into_iter().map(|num| dp(num, 75, &mut cache)).sum())
}

fn dp(num: i64, round: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if round == 0 {
        return 1;
    }
    let key = (num, round);
    if let Some(&result) = cache.get(&key) {
        return result;
    }
    let result = if num == 0 {
        dp(1, round - 1, cache)
    } else if num.to_string().len() % 2 == 0 {
        let num_str = num.to_string();
        let (left, right) = num_str.split_at(num_str.len() / 2);
        dp(left.parse().unwrap(), round - 1, cache) + dp(right.parse().unwrap(), round - 1, cache)
    } else {
        dp(num * 2024, round - 1, cache)
    };

    cache.insert(key, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_1() {
        println!("result: {:?}", solve_day11());
    }

    #[test]
    fn test_day11_2() {
        println!("result: {:?}", solve_day11_2());
    }
}
