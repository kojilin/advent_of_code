use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn solve_day11() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day11.txt")?;
    let mut map = vec![];
    let mut count = vec![];
    for line in input.lines() {
        let mut row = vec![];
        let mut c_row = vec![];
        for c in line.chars() {
            row.push(c);
            c_row.push(0);
        }
        count.push(c_row);
        map.push(row);
    }

    let mut seats_occupied = 0;
    loop {
        let mut change_count = 0;
        let mut next_c = count.clone();
        let mut next_map = map.clone();
        for y in 0..count.len() {
            for x in 0..count[y].len() {
                if count[y][x] == 0 && map[y][x] == 'L' {
                    change_count += 1;
                    next_map[y][x] = '#';
                    increase_adjacent(&mut count, &mut next_c, y, x, 1);
                    seats_occupied += 1;
                } else if count[y][x] >= 4 && map[y][x] == '#' {
                    change_count += 1;
                    next_map[y][x] = 'L';
                    increase_adjacent(&mut count, &mut next_c, y, x, -1);
                    seats_occupied -= 1;
                }
            }
        }
        if change_count == 0 {
            break;
        }
        count = next_c;
        map = next_map;
    }
    Ok(seats_occupied)
}

fn increase_adjacent(count: &mut Vec<Vec<i32>>, next_c: &mut Vec<Vec<i32>>, y: usize, x: usize, diff: i32) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let ny = y as i64 + i;
            if ny < 0 || ny >= count.len() as i64 {
                continue;
            }
            let nx = x as i64 + j;
            if nx < 0 || nx >= count[y].len() as i64 {
                continue;
            }
            next_c[ny as usize][nx as usize] += diff;
        }
    }
}

fn solve_day11_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day11.txt")?;
    let mut map = vec![];
    let mut count = vec![];
    for line in input.lines() {
        let mut row = vec![];
        let mut c_row = vec![];
        for c in line.chars() {
            row.push(c);
            c_row.push(0);
        }
        count.push(c_row);
        map.push(row);
    }

    let mut relations = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '.' {
                continue;
            }
            let nearby = relations.entry((y, x)).or_insert_with(|| HashSet::new());

            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let mut step = 1;
                    loop {
                        let ny = y as i64 + i * step;
                        if ny < 0 || ny >= count.len() as i64 {
                            break;
                        }
                        let nx = x as i64 + j * step;
                        if nx < 0 || nx >= count[y].len() as i64 {
                            break;
                        }
                        if map[ny as usize][nx as usize] != '.' {
                            nearby.insert((ny, nx));
                            break;
                        }
                        step += 1;
                    }
                }
            }
        }
    }

    let mut seats_occupied = 0;
    loop {
        let mut change_count = 0;
        let mut next_c = count.clone();
        let mut next_map = map.clone();
        for y in 0..count.len() {
            for x in 0..count[y].len() {
                if count[y][x] == 0 && map[y][x] == 'L' {
                    change_count += 1;
                    next_map[y][x] = '#';
                    increase_adjacent2(&mut next_c, &relations, y, x, 1);
                    seats_occupied += 1;
                } else if count[y][x] >= 5 && map[y][x] == '#' {
                    change_count += 1;
                    next_map[y][x] = 'L';
                    increase_adjacent2(&mut next_c, &relations, y, x, -1);
                    seats_occupied -= 1;
                }
            }
        }
        if change_count == 0 {
            break;
        }
        count = next_c;
        map = next_map;
    }
    Ok(seats_occupied)
}

fn increase_adjacent2(next_c: &mut Vec<Vec<i32>>, relations: &HashMap<(usize, usize), HashSet<(i64, i64)>>, y: usize, x: usize, diff: i32) {
    for (y, x) in relations.get(&(y, x)).unwrap() {
        next_c[*y as usize][*x as usize] += diff;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result 1: {}", solve_day11()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result 2: {}", solve_day11_2()?);
        Ok(())
    }
}
