use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day11() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day11.txt")?;
    let mut nums: Vec<i64> = input
        .split(" ")
        .map(|x| (x).parse::<i64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_nums: Vec<i64> = Vec::new();
        for &str in nums.iter() {
            let to_str = str.to_string();
            if str == 0 {
                new_nums.push(1);
            } else if to_str.len() % 2 == 0 {
                new_nums.push(to_str[0..to_str.len() / 2].parse::<i64>().unwrap());
                new_nums.push(
                    (&to_str[to_str.len() / 2..to_str.len()])
                        .parse::<i64>()
                        .unwrap(),
                );
            } else {
                new_nums.push(str * 2024);
            }
        }
        nums = new_nums;
    }

    Ok(nums.len() as i64)
}

fn solve_day11_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day11.txt")?;
    let nums: Vec<i64> = input
        .split(" ")
        .map(|x| (x).parse::<i64>().unwrap())
        .collect();

    let mut result = 0;
    let mut cache = HashMap::new();
    for num in nums {
        result += dp(num, 75, &mut cache);
    }

    Ok(result)
}

fn dp(num: i64, round: i32, cache: &mut HashMap<String, i64>) -> i64 {
    if round == 0 {
        return 1;
    }
    let key = &(num.to_string() + "_" + &round.to_string());
    if cache.contains_key(key) {
        return *cache.get(key).unwrap();
    }
    let mut result = 0;
    let to_str = num.to_string();
    if num == 0 {
        result = dp(1, round - 1, cache);
    } else if to_str.len() % 2 == 0 {
        result += dp(
            to_str[0..to_str.len() / 2].parse::<i64>().unwrap(),
            round - 1,
            cache,
        );
        result += dp(
            (to_str[to_str.len() / 2..to_str.len()])
                .parse::<i64>()
                .unwrap(),
            round - 1,
            cache,
        );
    } else {
        result = dp(num * 2024, round - 1, cache);
    }

    cache.insert(key.to_string(), result);
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
