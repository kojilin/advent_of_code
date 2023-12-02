use std::error::Error;
use std::fs;

fn solve_day1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2023/day1.txt")?;
    let numbers: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    for line in numbers {
        let all_digit: Vec<u32> = line.chars().filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap()).collect();
        if let (Some(first), Some(last)) = (all_digit.first(), all_digit.last()) {
            total += *first as i64 * 10 + *last as i64;
        }
    }
    Ok(total)
}

fn solve_day1_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2023/day1.txt")?;
    let numbers: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    for line in numbers {
        let all_digit: Vec<u32> =
            line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap()).collect();
        if let (Some(first), Some(last)) = (all_digit.first(), all_digit.last()) {
            total += *first as i64 * 10 + *last as i64;
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
