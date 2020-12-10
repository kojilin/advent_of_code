use std::error::Error;
use std::fs;

fn solve_day10() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/day10.txt")?;
    let mut all = vec![];
    for line in input.lines() {
        let volt = line.parse::<usize>()?;
        all.push(volt);
    }
    all.sort();
    // 1,2,3
    let mut tmp = vec![0, 0, 0];
    let mut current = 0;

    for v in all {
        tmp[v - current - 1] += 1;
        current = v;
    }
    Ok(tmp[0] * (tmp[2] + 1))
}

fn solve_day10_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/day10.txt")?;
    let mut all = vec![];
    all.push(0);
    for line in input.lines() {
        let volt = line.parse::<usize>()?;
        all.push(volt);
    }
    all.sort();
    all.push(all.last().unwrap() + 3);

    let mut dp = vec![0; all.len()];
    dp[0] = 1;
    for i in 0..all.len() {
        for diff in 1..=3 {
            if i + diff >= all.len() {
                continue;
            }
            if all[i + diff] - all[i] <= 3 {
                dp[i + diff] += dp[i];
            }
        }
    }
    Ok(dp[dp.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result 1: {}", solve_day10()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result 2: {}", solve_day10_2()?);
        Ok(())
    }
}
