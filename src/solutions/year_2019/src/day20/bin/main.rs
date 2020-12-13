use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day20/bin/day20.txt")?;
    let (w, h, map) = build_map(&input);
    let direction = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut warp_gates_cache: HashMap<String, (i32, i32)> = HashMap::new();
    // from -> to with +1 level or -1 level
    let mut warp_to = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if map.contains_key(&(x, y)) && map[&(x, y)] == '.' {
                for (index_of_d, d) in direction.iter().enumerate() {
                    let next_point = (x + d.0, y + d.1);
                    let next = map.get(&next_point).unwrap_or(&' ');
                    if *next as i32 >= 65 && *next as i32 <= 90 {
                        // warp gate
                        let mut warp_name = String::new();
                        if index_of_d % 2 == 0 {
                            warp_name.push(*next);
                            warp_name.push(map[&(next_point.0 + d.0, next_point.1 + d.1)]);
                        } else {
                            warp_name.push(map[&(next_point.0 + d.0, next_point.1 + d.1)]);
                            warp_name.push(*next);
                        }
                        let another_warp = warp_gates_cache.get(&warp_name);
                        if another_warp.is_none() {
                            warp_gates_cache.insert(warp_name.clone(), (x, y));
                        } else {
                            // mutual setting.
                            let wp = warp_gates_cache.remove(&warp_name).unwrap();
                            if wp.0 == 2 || wp.0 == w - 3 || wp.1 == 2 || wp.1 == h - 3 {
                                // outer
                                warp_to.insert((wp.0, wp.1), (x, y, -1));
                            } else {
                                warp_to.insert((wp.0, wp.1), (x, y, 1));
                            }

                            if x == 2 || x == w - 3 || y == 2 || y == h - 3 {
                                // outer
                                warp_to.insert((x, y), (wp.0, wp.1, -1));
                            } else {
                                warp_to.insert((x, y), (wp.0, wp.1, 1));
                            }
                        }
                    }
                }
            }
        }
    }

//    println!("warp_to: {:?}", warp_to);

    let from = warp_gates_cache.remove("AA").unwrap();
    let to = warp_gates_cache.remove("ZZ").unwrap();
    println!("{:?} => {:?}", (from.0, from.1), (to.0, to.1));

    //bfs node and count
    let mut queue: Vec<((i32, i32), i32, i32)> = vec![(from, 0, 0)];
    let mut visited = vec![];

    while !queue.is_empty() {
        let (current, step, level) = queue.remove(0);
//        println!(" get pos: {:?}, step:{}, level:{}", current, step, level);
        if level > 30 {
            continue;
        }

        visited.push((current.0, current.1, level));

        if let Some((tx, ty, t_level)) = warp_to.get(&current) {
            if level != 0 || *t_level != -1 {
                if !visited.contains(&(*tx, *ty, level + *t_level)) {
                    queue.push(((*tx, *ty), step + 1, level + *t_level));
                    visited.push((*tx, *ty, level + *t_level));
                }
            }
        }

        for d in direction.iter() {
            let next_point = (current.0 + d.0, current.1 + d.1);
            if visited.contains(&(next_point.0, next_point.1, level)) {
                continue;
            }
            let next = map.get(&next_point).unwrap_or(&' ');

            if next_point.0 == to.0 && next_point.1 == to.1 {
                //arrived
                if level == 0 {
                    println!("{}", step + 1);
                    return Ok(());
                } else {
                    continue;
                }
            }

            if *next == '.' {
//                println!(" push pos: {:?}, step:{}, level:{}", (next_point.0, next_point.1), step + 1, level);
                queue.push(((next_point.0, next_point.1), step + 1, level));
                visited.push((next_point.0, next_point.1, level));
            }
        }
    }
    panic!("Failed.")
}

fn build_map(input: &str) -> (i32, i32, HashMap<(i32, i32), char>) {
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut tmp_x = 0;
        for c in line.chars() {
            if c == '\n' {
                continue;
            }
            map.insert((tmp_x, y), c);
            tmp_x += 1;
        }
        x = max(x, tmp_x);
        y += 1;
    }
    (x, y, map)
}
