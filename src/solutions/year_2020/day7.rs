use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Edge {
    color: String,
    count: usize,
}

fn solve_day7_1() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day7.txt")?;
    let mut edges = HashMap::new();
    let from_regex = Regex::new(r"^(.*) bags contain")?;
    let content_regex = Regex::new(r"(\d+) ([^,]*) bag")?;
    for line in input.lines() {
        let mut from = String::new();
        for c in from_regex.captures_iter(line) {
            from = c[1].to_owned();
        };
        for c in content_regex.captures_iter(line) {
            let edges = edges.entry(c[2].to_owned()).or_insert_with(|| vec![]);
            edges.push(Edge {
                color: from.clone(),
                count: c[1].parse::<usize>()?,
            });
        };
    }
    let mut dp = HashSet::new();
    dfs("shiny gold", &edges, &mut dp);
    Ok(dp.len() - 1)
}

fn dfs(current: &str, edges: &HashMap<String, Vec<Edge>>, dp: &mut HashSet<String>) {
    dp.insert(current.to_owned());
    if let Some(edges_vec) = edges.get(current) {
        for edge in edges_vec {
            if !dp.contains(&edge.color) {
                dfs(&edge.color, edges, dp);
            }
        }
    }
}

fn solve_day7_2() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day7.txt")?;
    let mut edges = HashMap::new();
    let from_regex = Regex::new(r"^(.*) bags contain")?;
    let content_regex = Regex::new(r"(\d+) ([^,]*) bag")?;
    for line in input.lines() {
        let mut from = String::new();
        for c in from_regex.captures_iter(line) {
            from = c[1].to_owned();
        };
        let edges = edges.entry(from.clone()).or_insert_with(|| vec![]);
        for c in content_regex.captures_iter(line) {
            edges.push(Edge {
                color: c[2].to_owned(),
                count: c[1].parse::<usize>()?,
            });
        };
    }
    let mut dp = HashMap::new();
    let result = dfs2("shiny gold", &edges, &mut dp);
    Ok(result)
}

fn dfs2(current: &str, edges: &HashMap<String, Vec<Edge>>, dp: &mut HashMap<String, usize>) -> usize {
    dp.insert(current.to_owned(), 0);
    // we should not have cycle
    let mut count = 0;
    if let Some(edges_vec) = edges.get(current) {
        for edge in edges_vec {
            let mut sum = 1;
            if let Some(&sub_count) = dp.get(&edge.color) {
                sum += sub_count;
            } else {
                sum += dfs2(&edge.color, edges, dp);
            }
            count += sum * edge.count;
        }
    }
    dp.insert(current.to_owned(), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("{}", solve_day7_1()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("{}", solve_day7_2()?);
        Ok(())
    }
}
