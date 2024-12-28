use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::ops::{Mul, Sub};

fn solve_day23() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day23.txt")?;
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut link: Vec<&str> = line.split("-").collect();
        let from = link[0];
        let to = link[1];
        links
            .entry(from)
            .and_modify(|others| {
                others.push(to);
            })
            .or_insert(vec![to]);
        links
            .entry(to)
            .and_modify(|others| {
                others.push(from);
            })
            .or_insert(vec![from]);
    }

    let mut result = HashSet::new();
    for &key in links.keys() {
        let mut visited = HashSet::new();
        visited.insert(key);
        if let Some(list) = dfs(key, key, &links, 3, &mut visited) {
            for combo in list {
                let mut sorted_vec: Vec<&str> = combo.into_iter().collect();
                sorted_vec.sort();
                //must be 3 items
                result.insert((sorted_vec[0], sorted_vec[1], sorted_vec[2]));
            }
        }
        visited.remove(key);
    }
    let mut count = 0;
    for lan in result {
        if lan.0.starts_with("t") || lan.1.starts_with("t") || lan.2.starts_with("t") {
            count += 1;
        }
    }
    Ok(count)
}

fn dfs<'b>(
    from: &'b str,
    current: &'b str,
    links: &'b HashMap<&str, Vec<&str>>,
    hop: i8,
    visited: &mut HashSet<&'b str>,
) -> Option<Vec<HashSet<&'b str>>> {
    if hop == 0 {
        return None;
    }

    if let Some(others) = links.get(current) {
        let mut result = Vec::new();
        for to in others {
            if hop == 1 && *to == from {
                result.push(HashSet::from_iter([current, *to]));
                continue;
            }

            if visited.contains(to) {
                continue;
            }
            visited.insert(*to);
            if let Some(mut founds) = dfs(from, to, links, hop - 1, visited) {
                for mut v in founds {
                    v.insert(current);
                    result.push(v);
                }
            }
            visited.remove(*to);
        }
        return Some(result);
    }
    None
}

fn solve_day23_2() -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day23.txt")?;
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut link: Vec<&str> = line.split("-").collect();
        let from = link[0];
        let to = link[1];
        links
            .entry(from)
            .and_modify(|others| {
                others.push(to);
            })
            .or_insert(Vec::from_iter([to]));
        links
            .entry(to)
            .and_modify(|others| {
                others.push(from);
            })
            .or_insert(Vec::from_iter([from]));
    }

    let mut result = String::new();
    for &key in links.keys() {
        result = std::cmp::max_by_key(result, check(key, &links), |path| path.len());
    }
    Ok(result)
}

fn check<'b>(
    from: &'b str,
    // using set is better, but...
    links: &'b HashMap<&str, Vec<&str>>,
) -> String {
    let mut result = Vec::new();
    // looks like others len is 13
    let mut link_to = links.get(from).unwrap();
    let variation = (1 << link_to.len()) - 1;
    for v in 1..=variation {
        // picks the items
        let mut for_checking = Vec::new();
        for i in 0..13 {
            if v & (1 << i) != 0 {
                for_checking.push(link_to[i]);
            }
        }

        let mut all_match = true;
        for i in 0..for_checking.len() {
            for j in i + 1..for_checking.len() {
                if !links
                    .get(for_checking[i])
                    .unwrap()
                    .contains(&for_checking[j])
                {
                    all_match = false;
                    break;
                }
            }
            if !all_match {
                break;
            }
        }
        // println!("{}>>>>>{:?} result:{}", from, for_checking, all_match);
        if all_match && for_checking.len() > result.len() {
            result = for_checking;
        }
    }
    result.push(from);
    result.sort();
    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day23()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day23_2()?);
        Ok(())
    }
}
