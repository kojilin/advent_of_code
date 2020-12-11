use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // 5x5
    let input = fs::read_to_string("src/day24/bin/day24.txt")?;
    let map: Vec<i32> = input.chars()
        .filter(|&c| c == '.' || c == '#')
        .map(|c| {
            if c == '#' {
                1
            } else {
                0
            }
        }).collect();

    let mut map_int = 0;
    for (i, &x) in map.iter().enumerate() {
        if x == 1 {
            map_int |= 1 << i as i32;
        }
    }
    let mut maps: HashMap<i32, i32> = HashMap::new();
    maps.insert(0, map_int);

    for i in 0..200 {
        if i % 2 == 0 {
            maps.insert(i / 2 + 1, 0);
            maps.insert(-(i / 2 + 1), 0);
        }
        calculate(&mut maps);
    }
    let mut count = 0;
    for (&level, &x) in maps.iter() {
        for i in 0..25 {
            if 1 << i & x == 1 << i {
                count += 1;
            }
        }
//        println!("======{}=====", level);
//        for i in 0..5 {
//            for j in 0..5 {
//                if x & (1 << i * 5 + j) == (1 << i * 5 + j) {
//                    print!("#");
//                } else { print!("."); }
//            }
//            println!();
//        }
    }
    println!("{}", count);
    Ok(())
}

fn calculate(maps: &mut HashMap<i32, i32>) {
    let mut new_maps = HashMap::new();
    for (&level, map_int) in maps.iter() {
        let mut next_map = 0;
        let outer_map = maps.get(&(level - 1)).unwrap_or(&0);
        let inner_map = maps.get(&(level + 1)).unwrap_or(&0);
        for i in 0..25 {
            let mut bug_count = 0;
            if i == 12 {
                continue;
            }
            // left
            if i % 5 == 0 {
                // outer 12
                if *outer_map & 1 << 11 == 1 << 11 {
                    bug_count += 1;
                }
            } else if i == 13 {
                // 14th
                for inner_i in 1..=5 {
                    let index = 5 * inner_i - 1;
                    if *inner_map & 1 << index == 1 << index {
                        bug_count += 1;
                    }
                }
            } else {
                if (1 << i - 1 & *map_int) == (1 << i - 1) {
                    bug_count += 1;
                }
            }
            // right
            if i % 5 == 4 {
                // outer 14
                if *outer_map & 1 << 13 == 1 << 13 {
                    bug_count += 1;
                }
            } else if i == 11 {
                // 12th
                for inner_i in 1..=5 {
                    let index = 5 * inner_i - 5;
                    if *inner_map & 1 << index == 1 << index {
                        bug_count += 1;
                    }
                }
            } else {
                if (1 << i + 1 & *map_int) == (1 << i + 1) {
                    bug_count += 1;
                }
            }

            // down
            if i / 5 == 4 {
                // outer 18
                if *outer_map & 1 << 17 == 1 << 17 {
                    bug_count += 1;
                }
            } else if i == 7 {
                // 8th
                for index in 0..5 {
                    if *inner_map & 1 << index == 1 << index {
                        bug_count += 1;
                    }
                }
            } else {
                if (1 << i + 5 & *map_int) == (1 << i + 5) {
                    bug_count += 1;
                }
            }

            // up
            if i / 5 == 0 {
                // outer 8
                if *outer_map & 1 << 7 == 1 << 7 {
                    bug_count += 1;
                }
            } else if i == 17 {
                // 17th
                for index in 20..25 {
                    if *inner_map & 1 << index == 1 << index {
                        bug_count += 1;
                    }
                }
            } else {
                if (1 << i - 5 & *map_int) == (1 << i - 5) {
                    bug_count += 1;
                }
            }

            // counting bugs.
            if 1 << i & *map_int == 1 << i {
                if bug_count == 1 {
                    next_map |= 1 << i;
                }
            } else {
                if bug_count == 1 || bug_count == 2 {
                    next_map |= 1 << i;
                }
            }
        }
        new_maps.insert(level, next_map);
    }
    maps.clone_from(&new_maps);
}
