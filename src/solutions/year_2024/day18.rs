use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day18() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day18.txt")?;
    let falls: HashSet<(i64, i64)> = input
        .lines()
        .take(1024)
        .map(|x| {
            let row: Vec<i64> = x.split(",").map(|x1| x1.parse::<i64>().unwrap()).collect();
            // x, y
            (row[0], row[1])
        })
        .collect();
    const WIDTH: i64 = 71;
    const HEIGHT: i64 = 71;
    const DIRECTION: [i64; 5] = [-1, 0, 1, 0, -1];

    let mut current = (0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(current);
    let mut count = 0;
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while !queue.is_empty() {
            let pop = queue.pop_front().unwrap();
            if pop == (WIDTH - 1, HEIGHT - 1) {
                return Ok(count);
            }
            if visited.contains(&pop) {
                continue;
            }
            visited.insert(pop);

            for i in 0..4 {
                let next_x = pop.0 as i64 + DIRECTION[i];
                let next_y = pop.1 as i64 + DIRECTION[i + 1];

                if next_y < 0 || next_x < 0 || next_y >= HEIGHT || next_x >= WIDTH {
                    continue;
                }
                if !falls.contains(&(next_x, next_y)) && !visited.contains(&(next_x, next_y)) {
                    next_queue.push_back((next_x, next_y));
                }
            }
        }
        queue = next_queue;
        count += 1;
    }
    panic!()
}

fn solve_day18_2() -> Result<(i64, i64), Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day18.txt")?;
    let falls: Vec<(i64, i64)> = input
        .lines()
        .map(|x| {
            let row: Vec<i64> = x.split(",").map(|x1| x1.parse::<i64>().unwrap()).collect();
            // x, y
            (row[0], row[1])
        })
        .collect();

    // we know it works
    let mut using_falls: HashSet<&(i64, i64)> = falls.iter().take(1024).collect();
    for next in 1024..falls.len() {
        using_falls.insert(&falls[next]);
        if bfs(&using_falls) {
            return Ok(falls[next]);
        }
    }
    panic!()
}

fn bfs(using_falls: &HashSet<&(i64, i64)>) -> bool {
    const WIDTH: i64 = 71;
    const HEIGHT: i64 = 71;
    const DIRECTION: [i64; 5] = [-1, 0, 1, 0, -1];
    let mut current = (0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(current);
    let mut count = 0;
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while !queue.is_empty() {
            let pop = queue.pop_front().unwrap();
            if pop == (WIDTH - 1, HEIGHT - 1) {
                return false;
            }
            if visited.contains(&pop) {
                continue;
            }
            visited.insert(pop);

            for i in 0..4 {
                let next_x = pop.0 as i64 + DIRECTION[i];
                let next_y = pop.1 as i64 + DIRECTION[i + 1];

                if next_y < 0 || next_x < 0 || next_y >= HEIGHT || next_x >= WIDTH {
                    continue;
                }
                if !using_falls.contains(&(next_x, next_y)) && !visited.contains(&(next_x, next_y))
                {
                    next_queue.push_back((next_x, next_y));
                }
            }
        }
        queue = next_queue;
        count += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day18()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{:?}", solve_day18_2()?);
        Ok(())
    }
}
