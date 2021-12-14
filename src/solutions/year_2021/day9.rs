use std::error::Error;
use std::fs;

fn solve_day9_1() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day9_2() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day9.txt")?;
    let mut result = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for c in line.chars() {
            line_vec.push(c.to_digit(10).unwrap() as i32);
        }
        result.push(line_vec);
    }
    return Ok(result);
}

fn solve_1(map: Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    let direction = vec![0, 1, 0, -1, 0];
    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let mut found = true;
            for i in 0..4 {
                let new_y = y as i32 + direction[i];
                let new_x = x as i32 + direction[i + 1];

                if new_y >= 0 && new_y < map.len() as i32 && new_x >= 0 && new_x < map[0].len() as i32
                    && value >= map[new_y as usize][new_x as usize] {
                    found = false;
                    break;
                }
            }
            if found {
                answer += value + 1;
            }
        }
    }
    return answer;
}


fn solve_2(map: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let direction = vec![0, 1, 0, -1, 0];
    let mut all = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 9 || visited[y][x] {
                continue;
            }
            let mut queue = vec![];
            queue.push((y, x));
            let mut area = 0;
            while !queue.is_empty() {
                let (y, x) = queue.pop().unwrap();
                if visited[y][x] {
                    continue;
                }
                area += 1;
                visited[y][x] = true;

                for i in 0..4 {
                    let new_y = y as i32 + direction[i];
                    let new_x = x as i32 + direction[i + 1];

                    if new_y >= 0 && new_y < map.len() as i32 && new_x >= 0
                        && new_x < map[0].len() as i32
                        && map[new_y as usize][new_x as usize] < 9 {
                        queue.push((new_y as usize, new_x as usize));
                    }
                }
            }
            all.push(area);
        }
    }
    all.sort();
    all.reverse();
    let mut answer = 1;
    for &v in all[0..3].iter() {
        answer *= v;
    }
    return answer;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day9_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day9_2()?);
        Ok(())
    }
}
