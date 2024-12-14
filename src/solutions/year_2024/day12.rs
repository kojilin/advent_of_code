use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;

fn solve_day12() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day12.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visited[y][x] {
                result += solve((y, x), &map, &mut visited);
            }
        }
    }
    Ok(result)
}

fn solve(position: (usize, usize), map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> i64 {
    const DIRECTION: [i64; 5] = [-1, 0, 1, 0, -1];
    let current_char = map[position.0][position.1];
    let mut queue = VecDeque::new();
    queue.push_back(position);

    let mut fence = 0;
    let mut count = 0;
    while let Some(next) = queue.pop_front() {
        if visited[next.0][next.1] {
            continue;
        }
        visited[next.0][next.1] = true;
        count += 1;

        for i in 0..4 {
            let next_y = next.0 as i64 + DIRECTION[i];
            let next_x = next.1 as i64 + DIRECTION[i + 1];

            if next_y < 0
                || next_x < 0
                || next_y >= map.len() as i64
                || next_x >= map[0].len() as i64
            {
                // fence
                fence += 1;
                continue;
            }
            if map[next_y as usize][next_x as usize] != current_char {
                fence += 1;
            } else {
                queue.push_back((next_y as usize, next_x as usize));
            }
        }
    }
    // println!("{}: {} from {:?}", current_char, result * count, position);
    fence * count
}

fn solve_day12_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day12.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visited[y][x] {
                result += solve_2((y, x), &map, &mut visited);
            }
        }
    }
    Ok(result)
}

fn solve_2(position: (usize, usize), map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> i64 {
    const DIRECTION: [i64; 5] = [-1, 0, 1, 0, -1];
    let current_char = map[position.0][position.1];
    let mut queue = VecDeque::new();
    queue.push_back(position);

    let mut fences_h = HashSet::new();
    let mut fences_v = HashSet::new();
    let mut count = 0;
    while let Some(next) = queue.pop_front() {
        if visited[next.0][next.1] {
            continue;
        }
        visited[next.0][next.1] = true;
        count += 1;

        for i in 0..4 {
            let next_y = next.0 as i64 + DIRECTION[i];
            let next_x = next.1 as i64 + DIRECTION[i + 1];

            if next_y < 0 {
                fences_h.insert((next.0 as i64, next.1 as i64));
                continue;
            } else if next_x < 0 {
                fences_v.insert((next.0 as i64, next.1 as i64));
                continue;
            } else if next_y >= map.len() as i64 {
                fences_h.insert((next.0 as i64 + 1, next.1 as i64));
                continue;
            } else if next_x >= map[0].len() as i64 {
                fences_v.insert((next.0 as i64, next.1 as i64 + 1));
                continue;
            }

            if map[next_y as usize][next_x as usize] != current_char {
                if DIRECTION[i] == 0 {
                    fences_v.insert((
                        (next.0 as i64 + next_y + 1) / 2,
                        (next.1 as i64 + next_x + 1) / 2,
                    ));
                } else {
                    fences_h.insert((
                        (next.0 as i64 + next_y + 1) / 2,
                        (next.1 as i64 + next_x + 1) / 2,
                    ));
                }
            } else {
                queue.push_back((next_y as usize, next_x as usize));
            }
        }
    }

    let mut side = 0;
    // For checking if it's the same side
    let fences_v_origin = fences_v.clone();
    let fences_h_origin = fences_h.clone();

    while !fences_v.is_empty() {
        side += 1;
        let &fence = fences_v.iter().next().clone().unwrap();
        fences_v.remove(&fence);

        let mut step = 1;
        loop {
            if !fences_v.contains(&(fence.0 + step, fence.1)) {
                break;
            }
            // for finding the cross
            if fences_h_origin.contains(&(fence.0 + step, fence.1))
                && fences_h_origin.contains(&(fence.0 + step, fence.1 - 1))
            {
                break;
            }

            if !fences_v.remove(&(fence.0 + step, fence.1)) {
                break;
            }
            step += 1;
        }
        step = 1;
        loop {
            if !fences_v.contains(&(fence.0 - step, fence.1)) {
                break;
            }
            if fences_h_origin.contains(&(fence.0 - step + 1, fence.1))
                && fences_h_origin.contains(&(fence.0 - step + 1, fence.1 - 1))
            {
                break;
            }
            if !fences_v.remove(&(fence.0 - step, fence.1)) {
                break;
            }
            step += 1;
        }
    }
    while !fences_h.is_empty() {
        side += 1;
        let &fence = fences_h.iter().next().clone().unwrap();
        fences_h.remove(&fence);

        let mut step = 1;
        loop {
            if !fences_h.contains(&(fence.0, fence.1 + step)) {
                break;
            }
            if fences_v_origin.contains(&(fence.0, fence.1 + step))
                && fences_v_origin.contains(&(fence.0 - 1, fence.1 + step))
            {
                break;
            }

            if !fences_h.remove(&(fence.0, fence.1 + step)) {
                break;
            }
            step += 1;
        }
        step = 1;
        loop {
            if !fences_h.contains(&(fence.0, fence.1 - step)) {
                break;
            }
            if fences_v_origin.contains(&(fence.0, fence.1 - step + 1))
                && fences_v_origin.contains(&(fence.0 - 1, fence.1 - step + 1))
            {
                break;
            }
            if !fences_h.remove(&(fence.0, fence.1 - step)) {
                break;
            }
            step += 1;
        }
    }

    println!(
        "{}: {}x{}= {} from {:?}",
        current_char,
        side,
        count,
        side * count,
        position
    );
    side * count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_1() {
        println!("result: {:?}", solve_day12());
    }

    #[test]
    fn test_day12_2() {
        println!("result: {:?}", solve_day12_2());
    }
}
