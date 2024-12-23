use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

const DIRECTION: [i64; 5] = [-1, 0, 1, 0, -1];

fn solve_day20() -> Result<i64, Box<dyn Error>> {
    let (map, path_counter) = prepare_map()?;

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                result = result + calculate_shortcut((y as i64, x as i64), &map, &path_counter);
            }
        }
    }

    Ok(result)
}

fn prepare_map() -> Result<(Vec<Vec<char>>, HashMap<(i64, i64), i32>), Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day20.txt")?;
    let mut map = Vec::new();
    for line in input.lines() {
        let row_chars: Vec<char> = line.chars().collect();
        map.push(row_chars);
    }
    let mut start = (-1, -1);
    let mut goal = (-1, -1);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start = (y as i64, x as i64);
            } else if map[y][x] == 'S' {
                goal = (y as i64, x as i64);
            }
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut visited = HashSet::new();
    let mut path_counter = HashMap::new();
    path_counter.insert(start, 0);
    let mut count = 0;

    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while !queue.is_empty() {
            let pop = queue.pop_front().unwrap();
            if pop == goal {
                break;
            }
            if visited.contains(&pop) {
                continue;
            }
            visited.insert(pop);
            path_counter.insert(pop, count);
            for i in 0..4 {
                let next_y = pop.0 as i64 + DIRECTION[i];
                let next_x = pop.1 as i64 + DIRECTION[i + 1];

                if next_y < 0
                    || next_x < 0
                    || next_y >= map.len() as i64
                    || next_x >= map[pop.1 as usize].len() as i64
                    || map[next_y as usize][next_x as usize] == '#'
                {
                    continue;
                }
                next_queue.push_back((next_y, next_x));
            }
        }
        queue = next_queue;
        count += 1;
    }
    Ok((map, path_counter))
}

fn calculate_shortcut(
    current: (i64, i64),
    map: &Vec<Vec<char>>,
    path_counter: &HashMap<(i64, i64), i32>,
) -> i64 {
    let mut counts = Vec::new();
    for i in 0..4 {
        let next_y = current.0 + DIRECTION[i];
        let next_x = current.1 + DIRECTION[i + 1];

        if next_y < 0
            || next_x < 0
            || next_y >= map.len() as i64
            || next_x >= map[current.1 as usize].len() as i64
            || map[next_y as usize][next_x as usize] == '#'
        {
            continue;
        }
        counts.push(path_counter.get(&(next_y, next_x)).unwrap())
    }
    counts.sort();
    let mut result = 0;
    for i in 0..counts.len() {
        for j in i + 1..counts.len() {
            if (counts[i] - counts[j]).abs() - 2 >= 100 {
                result += 1;
            }
        }
    }
    // println!("{:?} save {:?} => {}", current, counts, result);
    result
}

fn solve_day20_2() -> Result<i64, Box<dyn Error>> {
    let (map, path_counter) = prepare_map()?;
    let mut result = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '.' || map[y][x] == 'S' || map[y][x] == 'E' {
                result = result + calculate_shortcut_2((y as i64, x as i64), &map, &path_counter);
            }
        }
    }
    Ok(result)
}
fn calculate_shortcut_2(
    current: (i64, i64),
    map: &Vec<Vec<char>>,
    path_counter: &HashMap<(i64, i64), i32>,
) -> i64 {
    let mut result = 0;
    for i in -20..=20_i64 {
        for j in -20..=20_i64 {
            if i.abs() + j.abs() <= 20 {
                let next_y = current.0 + i;
                let next_x = current.1 + j;
                if next_y < 0
                    || next_x < 0
                    || next_y >= map.len() as i64
                    || next_x >= map[current.1 as usize].len() as i64
                    || map[next_y as usize][next_x as usize] == '#'
                {
                    continue;
                }

                let diff = (*path_counter.get(&current).unwrap()
                    - *path_counter.get(&(next_y, next_x)).unwrap());
                // to avoid counting twice
                if diff <= 0 {
                    continue;
                }

                if diff.abs() as i64 - (i.abs() + j.abs()) >= 100 {
                    // println!("{:?}>>>>{:?} save {}", current, (next_y, next_x), diff);
                    result += 1;
                }
            }
        }
    }
    // println!("{:?} save {:?} => {}", current, counts, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day20()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day20_2()?);
        Ok(())
    }
}
