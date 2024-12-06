use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::BufRead;

fn solve_day5() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day5.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut is_checking = false;
    let mut adjacents: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut queue: Vec<i64> = Vec::new();
    let mut value_index: HashMap<i64, usize> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            println!("{:?}", adjacents);
            is_checking = true;
            // build graph
            let mut visited: HashSet<i64> = HashSet::new();
            for x in adjacents.keys() {
                dfs(*x, &adjacents, &mut visited, &mut queue);
            }
            for (index, &value) in queue.iter().enumerate() {
                value_index.insert(value, index);
            }
            continue;
        }
        if is_checking {
            // logic
            println!("---------");
            let target: Vec<i64> = line.split(",").map(|c| c.parse::<i64>().unwrap()).collect();
            println!("{:?}", target);
            let map_index: Vec<usize> = target.iter().map(|&x| *value_index.get(&x).unwrap()).collect();
            println!("{:?}", map_index);
            if map_index.is_sorted() {
                total += target[target.len() / 2];
            }
        } else {
            let nums: Vec<i64> = line.split("|")
                .map(|content| content.parse::<i64>().unwrap()).collect();

            adjacents.entry(nums[0])
                .and_modify(|e| { e.push(nums[1]) })
                .or_insert(vec![nums[1]]);
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
