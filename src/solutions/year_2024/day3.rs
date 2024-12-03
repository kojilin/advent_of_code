use regex::Regex;
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day3() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day3.txt")?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0i64;

    let x: Vec<&str> = input.lines().collect();
    for line in x {
        println!("{:?}", line);
        for caps in re.captures_iter(line) {
            let num1 = caps.get(1).unwrap().as_str();
            let num2 = caps.get(2).unwrap().as_str();
            match (num1.parse::<i64>(), num2.parse::<i64>()) {
                (Ok(n1), Ok(n2)) => {
                    result += n1 * n2;
                }
                _ => {}
            }
        }
    }
    Ok(result)
}

fn solve_day3_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day3.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0i64;
    let mut can_do = true;
    for line in lines {
        for caps in re.captures_iter(line) {
            let target = caps.get(0).unwrap().as_str();
            if target == "don't()" {
                can_do = false;
            } else if target == "do()" {
                can_do = true;
            } else if can_do {
                for num_caps in re2.captures_iter(target) {
                    let num1 = num_caps.get(1).unwrap().as_str();
                    let num2 = num_caps.get(2).unwrap().as_str();
                    match (num1.parse::<i64>(), num2.parse::<i64>()) {
                        (Ok(n1), Ok(n2)) => {
                            total += n1 * n2;
                        }
                        _ => {}
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
        println!("result:{}", solve_day3()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day3_2()?);
        Ok(())
    }
}
