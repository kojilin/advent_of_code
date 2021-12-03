use std::error::Error;
use std::fs;

fn solve_day1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day1.txt")?;
    let numbers: Vec<i64> = input.lines().map(|line| line.parse::<i64>().unwrap()).collect();
    Ok(solution(numbers))
}

fn solve_day1_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day1.txt")?;
    let numbers: Vec<i64> = input.lines().map(|line| line.parse::<i64>().unwrap()).collect();
    Ok(solution2(numbers))
}


fn solution(numbers: Vec<i64>) -> i64 {
    let mut count = 0i64;
    let mut current = numbers[0];
    for &value in numbers.iter().skip(1) {
        if value > current {
            count += 1;
        }
        current = value;
    }
    return count;
}

fn solution2(numbers: Vec<i64>) -> i64 {
    let mut count = 0i64;
    let mut current = 0;
    for i in 0..numbers.len() {
        if i < 3 {
            current += numbers[i];
            continue;
        }
        let new = current - numbers[i - 3] + numbers[i];
        if new > current {
            count += 1;
        }
        current = new;
    }
    return count;
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
