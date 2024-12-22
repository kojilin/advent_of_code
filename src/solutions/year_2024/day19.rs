use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day19() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day19.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let colors: HashSet<&str> = lines[0].split(", ").collect();
    let max_len = colors.iter().map(|&x| x.len()).max().unwrap();

    let mut count = 0;
    for &row in lines.iter().skip(2) {
        if solve(row, max_len, &colors, &mut HashSet::new()) {
            count = count + 1;
        }
    }
    Ok(count)
}

// dp is the index we can't achieve
fn solve(target: &str, max_len: usize, colors: &HashSet<&str>, dp: &mut HashSet<usize>) -> bool {
    if dp.contains(&target.len()) {
        return false;
    }
    if target.is_empty() {
        return true;
    }
    for len in 1..=min(max_len, target.len()) {
        let sub = &target[0..len];
        if colors.contains(sub) {
            if solve(&target[len..target.len()], max_len, colors, dp) {
                return true;
            } else {
                // not working
                dp.insert(target.len() - len);
            }
        }
    }
    false
}

fn solve_day19_2() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day19.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let colors: HashSet<&str> = lines[0].split(", ").collect();
    let max_len = colors.iter().map(|&x| x.len()).max().unwrap();

    let mut count = 0;
    for &row in lines.iter().skip(2) {
        count = count + solve_2(row, max_len, &colors, &mut HashMap::new());
    }
    Ok(count)
}

// dp is the index we can't achieve
fn solve_2(
    target: &str,
    max_len: usize,
    colors: &HashSet<&str>,
    dp: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(cached) = dp.get(&target.len()) {
        return *cached;
    }

    if target.is_empty() {
        return 1;
    }
    let mut count = 0;
    for len in 1..=min(max_len, target.len()) {
        let sub = &target[0..len];
        if colors.contains(sub) {
            count = count + solve_2(&target[len..target.len()], max_len, colors, dp);
        }
    }
    dp.insert(target.len(), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day19()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day19_2()?);
        Ok(())
    }
}
