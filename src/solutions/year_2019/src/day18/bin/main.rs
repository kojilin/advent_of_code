use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day18/bin/day18.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();
    let mut keys = HashMap::new();
    let mut roots = Vec::new();
    let mut remain_keys = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '.' | '#' => {}
                '@' => roots.push((i as i32, j as i32)),
                c => {
                    let ascii = c as i32;
                    if ascii > 90 {
                        let code = 1 << (ascii - 97);
                        remain_keys |= code;
                        keys.insert(code, (i as i32, j as i32));
                    }
                }
            }
        }
    }

    let mut now = SystemTime::now();
    let mut reach_map = HashMap::new();
    for root in &roots {
        reach_map.extend(build_reach_map(*root, &map));
    }
//    println!("reach map: {:?}", reach_map);
    println!("calculate reach using {}ms", now.elapsed()?.as_millis());


    let cache_distances = build_cache_distances(&roots, &keys, &map);
    println!("calculate distances using {}ms", now.elapsed()?.as_millis());

    now = SystemTime::now();
    let mut calculation_cache = HashMap::new();
    println!("{:?} using {}ms", calculate(&roots, remain_keys, &keys, &map, &mut calculation_cache,
                                          &reach_map, &cache_distances),
             now.elapsed()?.as_millis());
    Ok(())
}

fn calculate(froms: &Vec<(i32, i32)>, remain_keys: i32, keys: &HashMap<i32, (i32, i32)>, map: &Vec<Vec<char>>,
             calculation_cache: &mut HashMap<(Vec<(i32, i32)>, i32), i32>,
             reach_map: &HashMap<(i32, i32), i32>,
             cache_distances: &HashMap<((i32, i32), (i32, i32)), i32>) -> i32 {
    if remain_keys == 0 {
        return 0;
    }

    if let Some(v) = calculation_cache.get(&(froms.to_vec(), remain_keys)) {
        return v.clone();
    }

    let mut result = std::i32::MAX;

    for root in froms.iter().enumerate() {
        for i in 0..26 {
            let code = 1 << i;
            if (code & remain_keys) == code {
                let to = keys[&code];
                let reach_result = reach_map.get(&to);
                if reach_result == None {
                    continue;
                }
                let needed_keys = reach_result.unwrap();
                if (remain_keys & *needed_keys) != 0 {
                    continue;
                }
                let mut new_froms = froms.to_vec();
                std::mem::replace(&mut new_froms[root.0], to);

                let distance = cache_distances.get(&(*root.1, to)).unwrap();
                // not reachable.
                if *distance == std::i32::MAX {
                    result = min(result, std::i32::MAX);
                    continue;
                }
                let other_distance = calculate(&new_froms, remain_keys ^ code, keys, map,
                                               calculation_cache, reach_map, cache_distances);
                // not reachable.
                if other_distance == std::i32::MAX {
                    result = min(result, std::i32::MAX);
                } else {
                    result = min(result, distance + other_distance);
                }
            }
        }
    }
    calculation_cache.insert((froms.to_vec(), remain_keys), result);
    result
}

// target position with it's required key.
fn build_reach_map(from: (i32, i32), map: &Vec<Vec<char>>) -> HashMap<(i32, i32), i32> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[from.0 as usize][from.1 as usize] = true;

    let mut queue: Vec<(i32, i32, i32)> = vec![(from.0, from.1, 0)];
    let ways = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut result = HashMap::new();
    while !queue.is_empty() {
        let point = queue.remove(0);
        visited[point.0 as usize][point.1 as usize] = true;
        for way in ways.iter() {
            let maybe_to = (way.0 + point.0, way.1 + point.1);
            if visited[maybe_to.0 as usize][maybe_to.1 as usize] {
                continue;
            }
            let point_c = map[maybe_to.0 as usize][maybe_to.1 as usize];
            if point_c == '#' {
                // no
            } else if point_c as i32 >= 65 && point_c as i32 <= 90 {
                let door = 1 << (point_c as i32 - 65);
                queue.push((maybe_to.0, maybe_to.1, door | point.2))
            } else {
                // . or other key or @.
                queue.push((maybe_to.0, maybe_to.1, point.2));
                if point_c as i32 >= 97 && point_c as i32 <= 122 {
                    result.insert((maybe_to.0, maybe_to.1), point.2);
                }
            }
        }
    }
    result
}

fn build_cache_distances(roots: &Vec<(i32, i32)>, keys: &HashMap<i32, (i32, i32)>, map: &Vec<Vec<char>>) -> HashMap<((i32, i32), (i32, i32)), i32> {
    let mut result = HashMap::new();
    for k1 in keys {
        for k2 in keys {
            if *(k1.0) != *(k2.0) {
                let distance = distance_bfs_v2(*k1.1, *k2.1, map).unwrap();
                result.insert((*k1.1, *k2.1), distance);
                result.insert((*k2.1, *k1.1), distance);
            }
        }
    }
    for root in roots {
        for k1 in keys {
            let distance = distance_bfs_v2(*root, *k1.1, map).unwrap();
            result.insert((*root, *k1.1), distance);
            result.insert((*k1.1, *root), distance);
        }
    }
    result
}

fn distance_bfs_v2(from: (i32, i32), to: (i32, i32), map: &Vec<Vec<char>>) -> Option<i32> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[from.0 as usize][from.1 as usize] = true;

    let mut queue: Vec<(i32, i32, i32)> = vec![(from.0, from.1, 0)];
    let ways = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while !queue.is_empty() {
        let point = queue.remove(0);
        visited[point.0 as usize][point.1 as usize] = true;

        for way in ways.iter() {
            let maybe_to = (way.0 + point.0, way.1 + point.1);
            if visited[maybe_to.0 as usize][maybe_to.1 as usize] {
                continue;
            }

            let point_c = map[maybe_to.0 as usize][maybe_to.1 as usize];
            if maybe_to.0 == to.0 && maybe_to.1 == to.1 {
                //arrived
                //cache.insert((from, to, remain_keys), Some(point.2 + 1));
                return Some(point.2 + 1);
            } else if point_c == '#' {
                // no
            } else {
                // . or other key or @.
                queue.push((maybe_to.0, maybe_to.1, point.2 + 1));
            }
        }
    }
    Some(std::i32::MAX)
}
