use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use regex::Regex;

fn solve_day16() -> Result<i64, Box<dyn Error>> {
    let range_regex = Regex::new(r"^[^:]*: (\d*)-(\d*) or (\d*)-(\d*)$")?;
    let input = fs::read_to_string("src/solutions/year_2020/day16.txt")?;
    let mut other_tickets = false;
    // every number is less 1000
    let mut flags = vec![false; 1000];
    let mut count = 0;
    for line in input.lines() {
        if range_regex.is_match(line) {
            for c in range_regex.captures_iter(line) {
                let &min1 = &c[1].parse::<i64>()?;
                let &max1 = &c[2].parse::<i64>()?;
                for i in min1..=max1 {
                    flags[i as usize] = true;
                }
                let &min2 = &c[3].parse::<i64>()?;
                let &max2 = &c[4].parse::<i64>()?;
                for i in min2..=max2 {
                    flags[i as usize] = true;
                }
            }
        } else if line.eq("your ticket:") {
            //
        } else if line.eq("nearby tickets:") {
            other_tickets = true;
        } else if !line.is_empty() && other_tickets {
            let tickets: Vec<i64> = line.split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            for t in tickets {
                if !flags[t as usize] {
                    count += t;
                    break;
                }
            }
        }
    }
    Ok(count)
}

fn solve_day16_2() -> Result<i64, Box<dyn Error>> {
    let range_regex = Regex::new(r"^([^:]*): (\d*)-(\d*) or (\d*)-(\d*)$")?;
    let input = fs::read_to_string("src/solutions/year_2020/day16.txt")?;
    let mut other_tickets = false;
    // every number is less 1000
    let mut flags: Vec<(String, Vec<bool>)> = vec![];
    let mut total_flag = vec![false; 1000];
    // 20 columns ticket
    let mut titles = HashMap::new();
    for i in 0..20 {
        titles.insert(i, HashSet::new());
    }
    let mut yourticket = vec![];
    for line in input.lines() {
        if range_regex.is_match(line) {
            for c in range_regex.captures_iter(line) {
                let mut flag = vec![false; 1000];
                let &min1 = &c[2].parse::<i64>()?;
                let &max1 = &c[3].parse::<i64>()?;
                for i in min1..=max1 {
                    flag[i as usize] = true;
                    total_flag[i as usize] = true;
                }
                let &min2 = &c[4].parse::<i64>()?;
                let &max2 = &c[5].parse::<i64>()?;
                for i in min2..=max2 {
                    flag[i as usize] = true;
                    total_flag[i as usize] = true;
                }
                let title = c[1].to_owned();
                flags.push((title.clone(), flag));
                for (_, set) in titles.iter_mut() {
                    set.insert(title.clone());
                }
            }
        } else if line.eq("your ticket:") {} else if line.eq("nearby tickets:") {
            other_tickets = true;
        } else if !line.is_empty() {
            if other_tickets {
                let tickets: Vec<i64> = line.split(",")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                let mut valid = true;
                for &t in &tickets {
                    if !total_flag[t as usize] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    // do filtering.
                    for (index, &t) in tickets.iter().enumerate() {
                        for (title, flag) in &flags {
                            if !flag[t as usize] {
                                let target_set = titles.get_mut(&index).unwrap();
                                target_set.remove(title);
                            }
                        }
                    }
                }
            } else {
                yourticket = line.split(",")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
            }
        }
    }
    let mut result = 1;
    let mut used: HashSet<String> = HashSet::new();
    for count in 1..=20 {
        for title_map in titles.iter_mut() {
            if title_map.1.len() == count as usize {
                for remove in &used {
                    title_map.1.remove(remove);
                }
                let remain = title_map.1.iter().next().unwrap();
                used.insert(remain.to_owned());
                if remain.starts_with("departure") {
                    result *= yourticket[*title_map.0];
                }
                break;
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day16()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day16_2()?);
        Ok(())
    }
}
