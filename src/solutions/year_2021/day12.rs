use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day12_1() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day12_2() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day12.txt")?;
    let mut result = HashMap::new();
    for line in input.lines() {
        let path: Vec<&str> = line.split('-').collect();
        if line.contains("start") {
            let vec = result.entry("start".to_owned()).or_insert(vec![]);
            if path[0].eq("start") {
                vec.push(path[1].to_owned());
            } else {
                vec.push(path[0].to_owned());
            }
        } else if line.contains("end") {
            if path[0].eq("end") {
                let vec = result.entry(path[1].to_owned()).or_insert(vec![]);
                vec.push("end".to_owned());
            } else {
                let vec = result.entry(path[0].to_owned()).or_insert(vec![]);
                vec.push("end".to_owned());
            }
        } else {
            result.entry(path[0].to_owned()).or_insert(vec![]).push(path[1].to_owned());
            result.entry(path[1].to_owned()).or_insert(vec![]).push(path[0].to_owned());
        }
    }

    return Ok(result);
}

fn solve_1(path: HashMap<String, Vec<String>>) -> i64 {
    // only put lowercase & start
    let mut visited: HashMap<String, bool> = HashMap::new();
    visited.insert("start".to_owned(), true);
    let mut debug_path = vec![];
    return dfs("start", &path, &mut visited, &mut debug_path);
}

fn dfs(current: &str, path: &HashMap<String, Vec<String>>, visited: &mut HashMap<String, bool>, debug_path: &mut Vec<String>) -> i64 {
    debug_path.push(current.to_owned());
    if current == "end" {
        debug_path.pop();
        return 1;
    }

    let mut result = 0;
    for adjs in path.get(current) {
        for adj in adjs {
            let adj = adj.as_str();
            if adj.chars().nth(0).unwrap().is_ascii_lowercase() {
                if let Some(&value) = visited.get(adj) {
                    if value {
                        continue;
                    }
                }
                visited.insert(adj.to_owned(), true);
                result += dfs(adj, path, visited, debug_path);
                visited.insert(adj.to_owned(), false);
            } else {
                result += dfs(adj, path, visited, debug_path);
            }
        }
    }
    debug_path.pop();
    result
}


fn solve_2(path: HashMap<String, Vec<String>>) -> i64 {
    // only put lowercase & start
    let mut visited: HashMap<String, bool> = HashMap::new();
    visited.insert("start".to_owned(), true);
    let mut debug_path = vec![];
    let twice = None;
    return dfs2("start", &path, &mut visited, &mut debug_path, &twice);
}

fn dfs2(current: &str, path: &HashMap<String, Vec<String>>, visited: &mut HashMap<String, bool>,
        debug_path: &mut Vec<String>, twice: &Option<String>) -> i64 {
    debug_path.push(current.to_owned());
    if current == "end" {
        // println!("{:?} and twice:{:?} and visited:{:?}", debug_path, twice, visited);
        debug_path.pop();
        return 1;
    }

    let mut result = 0;
    for adjs in path.get(current) {
        for adj in adjs {
            let adj = adj.as_str();
            if adj.chars().nth(0).unwrap().is_ascii_lowercase() {
                let mut new_twice = twice.clone();
                if let Some(&value) = visited.get(adj) {
                    if value {
                        if twice.is_none() {
                            new_twice = Some(adj.to_owned());
                        } else {
                            continue;
                        }
                    }
                }
                visited.insert(adj.to_owned(), true);
                result += dfs2(adj, path, visited, debug_path, &new_twice);
                if let Some(tmp) = new_twice {
                    if tmp.eq(&adj) {
                        continue;
                    }
                }
                visited.insert(adj.to_owned(), false);
            } else {
                result += dfs2(adj, path, visited, debug_path, twice);
            }
        }
    }
    debug_path.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day12_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day12_2()?);
        Ok(())
    }
}
