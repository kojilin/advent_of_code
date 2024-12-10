use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn solve_day10() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day10.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    // (height, count)
    let mut map: Vec<Vec<(i64, Option<HashSet<(i64, i64)>>)>> = Vec::new();
    for (i, &line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push((c.to_digit(10).unwrap() as i64, None));
        }
        map.push(row);
    }

    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j].0 == 0 {
                result += dfs(&mut map, (i as i64, j as i64)).len();
            }
        }
    }
    Ok(result as i64)
}

// store possible goals for path
fn dfs(
    map: &mut Vec<Vec<(i64, Option<HashSet<(i64, i64)>>)>>,
    // current y, x
    current: (i64, i64),
) -> HashSet<(i64, i64)> {
    let width = map[0].len();
    let height = map.len() as i64;
    let current_height = &mut map[current.0 as usize][current.1 as usize];
    let next_height = current_height.0 + 1;

    if current_height.1 != None {
        return current_height.1.as_ref().unwrap().clone();
    }
    if current_height.0 == 9 {
        // println!("??????");
        let mut goals = HashSet::new();
        goals.insert(current);
        map[current.0 as usize][current.1 as usize].1 = Some(goals);
        return map[current.0 as usize][current.1 as usize]
            .1
            .as_ref()
            .unwrap()
            .clone();
    }
    const DIRECTION: [i64; 5] = [0, -1, 0, 1, 0];
    let mut goals = HashSet::new();
    for i in 0..4 {
        let next_y = current.0 + DIRECTION[i as usize];
        let next_x = current.1 + DIRECTION[(i + 1) as usize];

        if next_x < 0 || next_x >= width as i64 || next_y < 0 || next_y >= height {
            //out of map
            continue;
        }
        let next_target = map[next_y as usize][next_x as usize].0;
        // println!("{} == {}", next_target, next_height);
        if next_target == next_height {
            goals.extend(dfs(map, (next_y, next_x)));
        }
    }
    map[current.0 as usize][current.1 as usize].1 = Some(goals);

    map[current.0 as usize][current.1 as usize]
        .1
        .as_ref()
        .unwrap()
        .clone()
}

fn solve_day10_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day10.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    // (height, count)
    let mut map: Vec<Vec<(i64, i64)>> = Vec::new();
    for (i, &line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push((c.to_digit(10).unwrap() as i64, -1));
        }
        map.push(row);
    }

    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j].0 == 0 {
                result += dfs_2(&mut map, (i as i64, j as i64));
            }
        }
    }
    Ok(result as i64)
}

// store possible path
fn dfs_2(
    map: &mut Vec<Vec<(i64, i64)>>,
    // current y, x
    current: (i64, i64),
) -> i64 {
    let width = map[0].len();
    let height = map.len() as i64;
    let current_height = &mut map[current.0 as usize][current.1 as usize];
    let next_height = current_height.0 + 1;

    if current_height.1 != -1 {
        return current_height.1;
    }
    if current_height.0 == 9 {
        current_height.1 = 1;
        return 1;
    }
    const DIRECTION: [i64; 5] = [0, -1, 0, 1, 0];
    let mut goals = 0;
    for i in 0..4 {
        let next_y = current.0 + DIRECTION[i as usize];
        let next_x = current.1 + DIRECTION[(i + 1) as usize];

        if next_x < 0 || next_x >= width as i64 || next_y < 0 || next_y >= height {
            //out of map
            continue;
        }
        let next_target = map[next_y as usize][next_x as usize].0;
        // println!("{} == {}", next_target, next_height);
        if next_target == next_height {
            goals += dfs_2(map, (next_y, next_x));
        }
    }
    map[current.0 as usize][current.1 as usize].1 = goals;
    goals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_1() {
        println!("result: {:?}", solve_day10());
    }

    #[test]
    fn test_day10_2() {
        println!("result: {:?}", solve_day10_2());
    }
}
