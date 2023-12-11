use std::error::Error;
use std::fs;

fn solve_day6() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day6.txt")?;
    let mut lines = content.lines();
    let times: Vec<i64> = lines.next().unwrap().split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok()).collect();

    let distances: Vec<i64> = lines.next().unwrap().split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok()).collect();

    let mut result = 1;
    for i in 0..times.len() {
        let target = distances[i];
        let mut count = 0;
        for speed in 1..times[i] {
            if (times[i] - speed) * speed > target {
                count += 1;
            }
        }
        result *= count;
    }
    Ok(result)
}

fn solve_day6_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day6.txt")?;
    let mut lines = content.lines();
    let time: i64 = lines.next().unwrap().split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        })
        .parse()?;

    let distance: i64 = lines.next().unwrap().split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        })
        .parse()?;
    let target = distance;
    let mut count = 0;
    for speed in 1..time {
        if (time - speed) * speed > target {
            count += 1;
        }
    }
    Ok(count)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day6()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day6_2()?);
        Ok(())
    }
}