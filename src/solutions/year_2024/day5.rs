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

fn dfs(current: i64, adjacents: &HashMap<i64, Vec<i64>>, visited: &mut HashSet<i64>, queue: &mut Vec<i64>) {
    if visited.contains(&current) {
        return;
    }
    visited.insert(current);;
    adjacents.get(&current).unwrap_or(&vec![]).iter().for_each(|&x| {
        dfs(x, adjacents, visited, queue);
    });
    queue.insert(0, current);
}

fn solve_day5_2() -> Result<i64, Box<dyn Error>> {
    Ok(0)
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
}
