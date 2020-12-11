use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day6/bin/day6.txt")?;
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let pair: Vec<String> = line.split(')').map(|s| s.to_string()).collect();
        if edges.contains_key(&pair[0]) {
            edges.get_mut(&pair[0]).unwrap().push(pair[1].clone());
        } else {
            edges.insert(pair[0].clone(), vec![pair[1].clone()]);
        }
    }
    //part1(&edges);
    part2(&edges);
    Ok(())
}

fn part1(edges: &HashMap<String, Vec<String>>) {
    let mut queue: Vec<&str> = Vec::new();
    let mut result: HashMap<&str, i32> = HashMap::new();
    queue.push("COM");
    result.insert("COM", 0);
    while !queue.is_empty() {
        let current = queue.remove(0);
        let count = result.get(current).unwrap() + 1;
        if edges.contains_key(current) {
            for orbit in edges.get(current).unwrap() {
                result.insert(orbit, count);
                queue.push(orbit);
            };
        }
    }
    println!("{}", result.values().sum::<i32>());
}

fn part2(edges: &HashMap<String, Vec<String>>) {
    let path_to_you = dfs(edges, "COM", "YOU");
    let path_to_san = dfs(edges, "COM", "SAN");
    let mut index = 0;
    loop {
        if path_to_you[index] != path_to_san[index] {
            // exclude self and start from 1.
            // ORB-A-B-C-YOU, 5-1-2 if different from A
            //      \
            //       D-SAN,   4-1-2
            println!("{}", path_to_you.len() - 1 - index + path_to_san.len() - 1 - index);
            break;
        }
        index += 1;
    }
}

fn dfs(edges: &HashMap<String, Vec<String>>, current: &str, target: &str) -> Vec<String> {
    if !edges.contains_key(current) {
        return vec![];
    }
    let next_nodes = edges.get(current).unwrap();
    if next_nodes.is_empty() {
        return vec![];
    }
    if next_nodes.iter().any(|s| s == target) {
        return vec![String::from(current), String::from(target)];
    }

    for x in next_nodes {
        let mut found = dfs(edges, x, target);
        if !found.is_empty() {
            let mut result: Vec<String> = Vec::new();
            result.push(String::from(current));
            result.append(&mut found);
            return result;
        }
    }

    return vec![];
}

