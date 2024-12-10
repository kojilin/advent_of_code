use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn solve_day6() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day6.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut map: Vec<Vec<char>> = Vec::new();

    let mut from = (0, 0);
    for (y, &line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (x, &c) in chars.iter().enumerate() {
            if c == '^' {
                from = (y, x);
            }
        }
        map.push(chars);
    }
    let mut set = HashSet::new();
    scan_map(&map, (from.0 as i64, from.1 as i64), 0, &mut set);
    Ok(set.len() as i64)
}

fn scan_map(
    map: &Vec<Vec<char>>,
    mut current: (i64, i64),
    mut current_direction: usize,
    visited: &mut HashSet<(i64, i64)>,
) {
    const DIRECTION: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    loop {
        visited.insert(current);
        let mut next = (
            current.0 + DIRECTION[current_direction].0,
            current.1 + DIRECTION[current_direction].1,
        );
        if next.0 < 0 || next.1 < 0 || next.0 >= map.len() as i64 || next.1 >= map[0].len() as i64 {
            return;
        }
        while map[next.0 as usize][next.1 as usize] == '#' {
            current_direction += 1;
            current_direction %= 4;
            next = (
                current.0 + DIRECTION[current_direction].0,
                current.1 + DIRECTION[current_direction].1,
            );
            // won't be stuck in a hole
        }
        current = next;
    }
}

fn solve_day6_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day6.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut map: Vec<Vec<char>> = Vec::new();

    let mut from = (0, 0);
    for (y, &line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (x, &c) in chars.iter().enumerate() {
            if c == '^' {
                from = (y, x);
            }
        }
        map.push(chars);
    }
    let mut set = HashSet::new();
    scan_map(&map, (from.0 as i64, from.1 as i64), 0, &mut set);

    for blockable_point in set {
        // ignore start
        if blockable_point.0 == from.0 as i64 && blockable_point.1 == from.1 as i64 {
            continue;
        }
        if check_loop(&map, (from.0 as i64, from.1 as i64, 0), 0, blockable_point) {
            total += 1;
        }
    }
    Ok(total)
}

fn check_loop(
    map: &Vec<Vec<char>>,
    // including direction
    mut current: (i64, i64, usize),
    mut current_direction: usize,
    new_block: (i64, i64),
) -> bool {
    let mut visited = HashSet::new();
    const DIRECTION: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    loop {
        if visited.contains(&current) {
            return true;
        }
        visited.insert(current);
        let mut next = (
            current.0 + DIRECTION[current_direction].0,
            current.1 + DIRECTION[current_direction].1,
            current_direction,
        );
        if next.0 < 0 || next.1 < 0 || next.0 >= map.len() as i64 || next.1 >= map[0].len() as i64 {
            return false;
        }
        while map[next.0 as usize][next.1 as usize] == '#'
            || (new_block.0 == next.0 && new_block.1 == next.1)
        {
            current_direction += 1;
            current_direction %= 4;
            next = (
                current.0 + DIRECTION[current_direction].0,
                current.1 + DIRECTION[current_direction].1,
                current_direction,
            );
            // won't be stuck in the hole
        }
        current = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6() {
        println!("{:?}", solve_day6());
    }

    #[test]
    fn test_day6_2() {
        println!("{:?}", solve_day6_2());
    }
}
