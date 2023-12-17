use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::ops::Not;

fn solve_day10() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day10.txt")?;
    let mut lines = content.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (row, col);
                break;
            }
        }
        map.push(line.chars().collect());
    }

    // BFS
    let mut visited = HashSet::new();
    let mut nexts = Vec::new();
    let mut steps_map = Vec::new();
    for _ in 0..map.len() {
        let mut vec = Vec::new();
        for _ in 0..map[0].len() {
            vec.push(-1);
        }
        steps_map.push(vec);
    }

    // println!("{:?}", next_of(start.clone(), &map));

    nexts.push(start.clone());
    let mut count = 0;
    let mut result = 0;
    while !nexts.is_empty() {
        for &p in &nexts {
            let around_p = next_of(p, &map);
            // let map1: Vec<i64> = around_p.iter().map(|&point| steps_map[point.0][point.1]).collect();
            // println!("{:?}, {}->{:?}", p, count, map1);
            if around_p.len() == 2 && around_p.iter().all(|&point| steps_map[point.0][point.1] == count - 1) {
                result = count;
            }
        }

        let mut next_steps = Vec::new();
        while !nexts.is_empty() {
            let next = nexts.pop().unwrap();
            if visited.contains(&next) {
                continue;
            }
            steps_map[next.0][next.1] = count;
            visited.insert(next.clone());
            for next_loop in next_of(next, &map) {
                if !visited.contains(&next_loop) {
                    next_steps.push(next_loop);
                }
            }
        }


        nexts = next_steps;
        count += 1;
    }
    // println!("{:?}", steps_map);
    Ok(result)
}

fn next_of(point: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let &c = map.get(point.0).unwrap().get(point.1).unwrap();
    let mut vec = Vec::new();
    match c {
        'S' => {
            // The input's S is not on the edge
            let dirs = [1, 0, -1, 0, 1];
            for offset in 0..4 {
                let next_y = point.0 as i64 + dirs[offset];
                let next_x = point.1 as i64 + dirs[offset + 1];
                if next_x < 0 || next_y < 0 || next_x >= map[0].len() as i64 || next_y >= map.len() as i64 {
                    continue;
                }
                let next = (next_y as usize, next_x as usize);
                if next_of(next, map).contains(&point) {
                    vec.push(next);
                }
            }
        }
        '.' => {}
        '|' => {
            if point.0 != 0 && ['|', '7', 'F', 'S'].contains(&map[point.0 - 1][point.1]) {
                vec.push((point.0 - 1, point.1));
            }
            if point.0 != map.len() - 1 && ['|', 'J', 'L', 'S'].contains(&map[point.0 + 1][point.1]) {
                vec.push((point.0 + 1, point.1));
            }
        }
        '-' => {
            if point.1 != 0 && ['-', 'F', 'L', 'S'].contains(&map[point.0][point.1 - 1]) {
                vec.push((point.0, point.1 - 1));
            }
            if point.1 != map[0].len() - 1 && ['-', 'J', '7', 'S'].contains(&map[point.0][point.1 + 1]) {
                vec.push((point.0, point.1 + 1));
            }
        }
        'L' => {
            if point.0 != 0 && ['|', '7', 'F', 'S'].contains(&map[point.0 - 1][point.1]) {
                vec.push((point.0 - 1, point.1));
            }
            if point.1 != map[0].len() - 1 && ['-', 'J', '7', 'S'].contains(&map[point.0][point.1 + 1]) {
                vec.push((point.0, point.1 + 1));
            }
        }
        'J' => {
            if point.0 != 0 && ['|', '7', 'F', 'S'].contains(&map[point.0 - 1][point.1]) {
                vec.push((point.0 - 1, point.1));
            }
            if point.1 != 0 && ['-', 'F', 'L', 'S'].contains(&map[point.0][point.1 - 1]) {
                vec.push((point.0, point.1 - 1));
            }
        }
        '7' => {
            if point.0 != map.len() - 1 && ['|', 'J', 'L', 'S'].contains(&map[point.0 + 1][point.1]) {
                vec.push((point.0 + 1, point.1));
            }
            if point.1 != 0 && ['-', 'F', 'L', 'S'].contains(&map[point.0][point.1 - 1]) {
                vec.push((point.0, point.1 - 1));
            }
        }
        'F' => {
            if point.0 != map.len() - 1 && ['|', 'J', 'L', 'S'].contains(&map[point.0 + 1][point.1]) {
                vec.push((point.0 + 1, point.1));
            }
            if point.1 != map[0].len() - 1 && ['-', 'J', '7', 'S'].contains(&map[point.0][point.1 + 1]) {
                vec.push((point.0, point.1 + 1));
            }
        }
        _ => panic!("wrong")
    };
    vec
}


fn solve_day10_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day10.txt")?;
    let mut lines = content.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (row, col);
                break;
            }
        }
        map.push(line.chars().collect());
    }

    let mut expand_map: Vec<Vec<char>> = vec![vec!['.'; map[0].len() * 3]; map.len() * 3];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                '.' => {
                    expand_map[i * 3 + 1][j * 3 + 1] = '*';
                }
                'S' => {
                    expand_map[i * 3 + 1][j * 3 + 1] = 'S';
                    if i != 0 && ['|', '7', 'F'].contains(&map[i - 1][j]) {
                        expand_map[i * 3][j * 3 + 1] = '|';
                    }
                    if i != map.len() - 1 && ['|', 'J', 'L'].contains(&map[i + 1][j]) {
                        expand_map[i * 3 + 2][j * 3 + 1] = '|';
                    }
                    if j != 0 && ['-', 'L', 'F'].contains(&map[i][j - 1]) {
                        expand_map[i * 3 + 1][j * 3] = '-';
                    }
                    if j != map[0].len() - 1 && ['-', 'J', '7'].contains(&map[i][j + 1]) {
                        expand_map[i * 3 + 1][j * 3 + 2] = '-';
                    }
                }
                '|' => {
                    expand_map[i * 3][j * 3 + 1] = '|';
                    expand_map[i * 3 + 1][j * 3 + 1] = '|';
                    expand_map[i * 3 + 2][j * 3 + 1] = '|';
                }
                '-' => {
                    expand_map[i * 3 + 1][j * 3] = '-';
                    expand_map[i * 3 + 1][j * 3 + 1] = '-';
                    expand_map[i * 3 + 1][j * 3 + 2] = '-';
                }
                'L' => {
                    expand_map[i * 3][j * 3 + 1] = '|';
                    expand_map[i * 3 + 1][j * 3 + 1] = 'L';
                    expand_map[i * 3 + 1][j * 3 + 2] = '-';
                }
                'J' => {
                    expand_map[i * 3][j * 3 + 1] = '|';
                    expand_map[i * 3 + 1][j * 3 + 1] = 'J';
                    expand_map[i * 3 + 1][j * 3] = '-';
                }
                '7' => {
                    expand_map[i * 3 + 1][j * 3] = '-';
                    expand_map[i * 3 + 1][j * 3 + 1] = '7';
                    expand_map[i * 3 + 2][j * 3 + 1] = '|';
                }
                'F' => {
                    expand_map[i * 3 + 1][j * 3 + 2] = '-';
                    expand_map[i * 3 + 1][j * 3 + 1] = 'F';
                    expand_map[i * 3 + 2][j * 3 + 1] = '|';
                }
                _ => {}
            }
        }
    }

    for x in &expand_map {
        println!("{:?}", x);
    }


    // BFS
    let mut visited = HashSet::new();
    let mut nexts = Vec::new();
    let mut line_map = vec![vec![false; expand_map[0].len()]; expand_map.len()];

    nexts.push((start.0 * 3 + 1, start.1 * 3 + 1));
    while !nexts.is_empty() {
        let mut next_steps = Vec::new();
        while !nexts.is_empty() {
            let next = nexts.pop().unwrap();
            if visited.contains(&next) {
                continue;
            }
            line_map[next.0][next.1] = true;
            visited.insert(next.clone());
            for next_loop in next_of(next, &expand_map) {
                if !visited.contains(&next_loop) {
                    next_steps.push(next_loop);
                }
            }
        }
        nexts = next_steps;
    }

    // nest
    visited.clear();
    let mut result = 0;
    // BFS draw the nest
    for i in 0..expand_map.len() {
        for j in 0..expand_map[i].len() {
            if line_map[i][j] || visited.contains(&(i, j)) {
                continue;
            }
            result += draw(&mut visited, (i, j), &expand_map, &line_map);
        }
    }
    Ok(result)
}

fn draw(visited: &mut HashSet<(usize, usize)>, from: (usize, usize), map: &Vec<Vec<char>>, is_line: &Vec<Vec<bool>>) -> i64 {
    let mut nexts = Vec::new();
    nexts.push(from);
    let mut is_nest = true;
    let mut count = 0;
    let mut all = vec![];
    while !nexts.is_empty() {
        let next = nexts.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next.clone());
        if next.0 % 3 == 1 && next.1 % 3 == 1 {
            all.push(next);
            count += 1;
        }
        let dirs = [1, 0, -1, 0, 1];
        for offset in 0..4 {
            let next_y = next.0 as i64 + dirs[offset];
            let next_x = next.1 as i64 + dirs[offset + 1];
            if next_x < 0 || next_y < 0 || next_x >= (map[0].len() - 1) as i64 || next_y >= (map.len() - 1) as i64 {
                is_nest = false;
                continue;
            }

            if is_line[next_y as usize][next_x as usize] {
                continue;
            }
            nexts.push((next_y as usize, next_x as usize));
        }
    }

    if is_nest {
        // println!("{:?}", all);
        count
    } else {
        0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day10() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result 1:{}", solve_day10()?);
        Ok(())
    }

    #[test]
    fn test_day10_2() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result 2:{}", solve_day10_2()?);
        Ok(())
    }
}