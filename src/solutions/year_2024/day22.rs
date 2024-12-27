use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day22() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day22.txt")?;
    let mut result = 0;
    for line in input.lines() {
        let mut price = line.parse::<i64>().unwrap();
        for _ in 0..2000 {
            price = calculate(price);
        }
        result += price;
    }
    Ok(result)
}

fn solve_day22_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day22.txt")?;
    let mut pattern_of_all = HashMap::new();
    for line in input.lines() {
        // for each buyer
        let mut price = line.parse::<i64>().unwrap();
        let mut prev = price % 10;
        let mut queue = VecDeque::new();
        let mut pattern_of_seller = HashMap::new();
        for _ in 0..2000 {
            price = calculate(price);
            let one_digit_price = price % 10;
            queue.push_back(one_digit_price - prev);
            prev = one_digit_price;
            if queue.len() > 4 {
                queue.pop_front();
                // this is the pattern
                let key: Vec<i64> = queue.iter().map(|x| *x).collect();
                pattern_of_seller
                    .entry(key)
                    .or_insert(one_digit_price);
            }
        }

        for (key, value) in pattern_of_seller {
            pattern_of_all
                .entry(key)
                .and_modify(|v| {
                    *v += value;
                })
                .or_insert(value);
        }
    }
    let result = pattern_of_all
        .iter()
        .max_by(|x1, x2| x1.1.cmp(x2.1))
        .unwrap();

    println!("{:?}", result);

    Ok(*result.1)
}

fn calculate(secret_number: i64) -> i64 {
    let step1 = ((secret_number * 64) ^ secret_number) % 16777216;
    let step2 = ((step1 / 32) ^ step1) % 16777216;
    let step3 = ((step2 * 2048) ^ step2) % 16777216;
    step3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day22()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day22_2()?);
        Ok(())
    }
}
