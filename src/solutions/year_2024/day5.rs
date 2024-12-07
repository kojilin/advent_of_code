use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::BufRead;

fn solve_day5() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day5.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut is_checking = false;
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            is_checking = true;
            continue;
        }
        if is_checking {
            // logic
            let target: Vec<i64> = line.split(",").map(|c| c.parse::<i64>().unwrap()).collect();
            let mut has_wrong = false;

            'outer: for i in 0..target.len() {
                for j in i + 1..target.len() {
                    if rules.get(&(target[i])).unwrap_or(&HashSet::new()).contains(&target[j]) {
                        has_wrong = true;
                        break 'outer;
                    }
                }
            }
            if !has_wrong {
                total += target[target.len() / 2];
            }
        } else {
            let nums: Vec<i64> = line.split("|")
                .map(|content| content.parse::<i64>().unwrap()).collect();

            rules.entry(nums[1])
                .and_modify(|e| {
                    e.insert(nums[0]);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(nums[0]);
                    set
                });
        }
    }

    Ok(total)
}

fn solve_day5_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day5.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut is_checking = false;
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            is_checking = true;
            continue;
        }
        if is_checking {
            // logic
            let mut target: Vec<i64> = line.split(",").map(|c| c.parse::<i64>().unwrap()).collect();
            let mut current_index = 0usize;
            let mut contains_fail_order = false;
            while current_index < target.len() {
                let mut has_wrong = -1;
                for j in current_index + 1..target.len() {
                    if rules.get(&(target[current_index])).unwrap_or(&HashSet::new()).contains(&target[j]) {
                        has_wrong = j as i64;
                        continue;
                    }
                }
                // put to the last
                if has_wrong != -1 {
                    contains_fail_order = true;
                    let v = target.remove(current_index);
                    target.insert(has_wrong as usize, v);
                    continue;
                }
                current_index += 1;
            }

            if contains_fail_order {
                println!("{:?}", target);
                total += target[target.len() / 2];
            } else {
                println!("no change: {:?}", target);
            }
        } else {
            let nums: Vec<i64> = line.split("|")
                .map(|content| content.parse::<i64>().unwrap()).collect();

            rules.entry(nums[1])
                .and_modify(|e| {
                    e.insert(nums[0]);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(nums[0]);
                    set
                });
        }
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day5_1() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day5()?);
        Ok(())
    }

    #[test]
    fn test_day5_2() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day5_2()?);
        Ok(())
    }
}
