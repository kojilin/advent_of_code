use std::error::Error;
use std::fs;

fn solve_day9() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day9.txt")?;
    let mut lines = content.lines();
    let mut result = 0;
    for line in lines {
        let nums: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap())
            .collect();
        let mut all: Vec<Vec<i64>> = Vec::new();
        all.push(nums);
        loop {
            let current = all.last().unwrap();
            let mut next: Vec<i64> = Vec::new();
            let mut all_zero = true;
            for index in 1..current.len() {
                let value = current[index] - current[index - 1];
                if value != 0 {
                    all_zero = false;
                }
                next.push(value);
            }
            if all_zero {
                // calculate it
                let mut diff = 0;
                while !all.is_empty() {
                    let last = all.pop().unwrap();
                    diff = last.last().unwrap() + diff;
                }
                result += diff;
                break;
            }
            all.push(next);
        }
    }
    Ok(result)
}

fn solve_day9_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day9.txt")?;
    let mut lines = content.lines();
    let mut result = 0;
    for line in lines {
        let nums: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap())
            .collect();
        let mut all: Vec<Vec<i64>> = Vec::new();
        all.push(nums);
        loop {
            let current = all.last().unwrap();
            let mut next: Vec<i64> = Vec::new();
            let mut all_zero = true;
            for index in 1..current.len() {
                let value = current[index] - current[index - 1];
                if value != 0 {
                    all_zero = false;
                }
                next.push(value);
            }
            if all_zero {
                // calculate it
                let mut diff = 0;
                while !all.is_empty() {
                    let last = all.pop().unwrap();
                    diff = last.first().unwrap() - diff;
                }
                result += diff;
                break;
            }
            all.push(next);
        }
    }
    Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day9() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result 1:{}", solve_day9()?);
        Ok(())
    }


    #[test]
    fn test_day9_2() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result 2:{}", solve_day9_2()?);
        Ok(())
    }
}