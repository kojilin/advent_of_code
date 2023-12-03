use std::cmp::max;
use std::error::Error;
use std::fs;

use regex::Regex;

fn solve_day2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2023/day2.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;

    let re = Regex::new(r"Game (\d+):").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();

    for line in lines {
        if (counts(&re_green, line, 13) && counts(&re_red, line, 12) && counts(&re_blue, line, 14)) {
            if let Some(captures) = re.captures(line) {
                if let Some(digit_match) = captures.get(1) {
                    total += digit_match.as_str().parse::<i64>().unwrap();
                }
            }
        }
    }
    Ok(total)
}

fn counts(re_green: &Regex, line: &str, max: i32) -> bool {
    for captures in re_green.captures_iter(line) {
        if let Some(digit_match) = captures.get(1) {
            if (digit_match.as_str().parse::<i32>().unwrap() > max) {
                return false;
            }
        }
    }
    true
}

fn solve_day2_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2023/day2.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;

    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();

    for line in lines {
        total += min_counts(&re_green, line) * min_counts(&re_red, line) * min_counts(&re_blue, line)
    }
    Ok(total)
}

fn min_counts(re_green: &Regex, line: &str) -> i64 {
    let mut min_counts = 0i64;
    for captures in re_green.captures_iter(line) {
        if let Some(digit_match) = captures.get(1) {
            min_counts = max(digit_match.as_str().parse::<i64>().unwrap(), min_counts);
        }
    }
    min_counts
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
