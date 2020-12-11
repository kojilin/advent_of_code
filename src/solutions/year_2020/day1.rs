use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn solve_day1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day1.txt")?;
    let numbers: Vec<i64> = input.lines().map(|line| line.parse::<i64>().unwrap()).collect();
    Ok(solution(numbers))
}

fn solve_day1_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day1.txt")?;
    let numbers: Vec<i64> = input.lines().map(|line| line.parse::<i64>().unwrap()).collect();
    Ok(solution2(numbers))
}


fn solution(numbers: Vec<i64>) -> i64 {
    let mut check_values: HashSet<i64> = HashSet::new();
    for value in numbers {
        if check_values.contains(&(2020 - value)) {
            return value * (2020 - value);
        } else {
            check_values.insert(value);
        }
    }
    panic!("Wrong inputs.")
}

fn solution2(numbers: Vec<i64>) -> i64 {
    // Pretend the numbers are distinct
    let candidates: HashSet<i64> = numbers.clone().into_iter().collect();

    let length = numbers.len();
    for i in 0..length {
        for j in i + 1..length {
            let target = 2020 - (numbers[i] + numbers[j]);
            if candidates.contains(&target) {
                return target * numbers[i] * numbers[j];
            }
        }
    }
    panic!("Wrong inputs.")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(solution(vec![1721, 979, 366, 299, 675, 1456]), 514579);
        println!("-----real-----");
        println!("result:{}", solve_day1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(solution2(vec![1721, 979, 366, 299, 675, 1456]), 241861950);
        println!("-----real-----");
        println!("result:{}", solve_day1_2()?);
        Ok(())
    }
}
