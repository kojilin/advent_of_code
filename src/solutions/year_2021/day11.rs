use std::error::Error;
use std::fs;

fn solve_day11_1() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}


fn solve_day11_2() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}


fn parse_input() -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day11.txt")?;
    let mut result = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for c in line.chars() {
            line_vec.push(c.to_digit(10).unwrap());
        }
        result.push(line_vec);
    }
    return Ok(result);
}


fn solve_1(mut map: Vec<Vec<u32>>) -> i64 {
    let mut result = 0;
    for _ in 0..100 {
        let mut queue = vec![];
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                map[y][x] += 1;
                if map[y][x] > 9 {
                    result += 1;
                    queue.push((y, x));
                    map[y][x] = 0;
                }
            }
        }
        while !queue.is_empty() {
            let poll = queue.remove(0);
            for oy in -1..=1 {
                for ox in -1..=1 {
                    if oy == 0 && ox == 0 {
                        continue;
                    }
                    let ny = poll.0 as i32 + oy;
                    let nx = poll.1 as i32 + ox;
                    if ny >= 0 && ny < map[0].len() as i32 && nx >= 0 && nx < map.len() as i32
                        && map[ny as usize][nx as usize] <= 9
                        && map[ny as usize][nx as usize] != 0 {
                        map[ny as usize][nx as usize] += 1;
                        if map[ny as usize][nx as usize] == 10 {
                            result += 1;
                            map[ny as usize][nx as usize] = 0;
                            queue.push((ny as usize, nx as usize));
                        }
                    }
                }
            }
        }
    }
    return result;
}

fn solve_2(mut map: Vec<Vec<u32>>) -> i64 {
    let target = (map.len() * map[0].len()) as i64;
    let mut step = 0;
    loop {
        step += 1;
        let mut queue = vec![];
        let mut snapshot = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                map[y][x] += 1;
                if map[y][x] > 9 {
                    snapshot += 1;
                    queue.push((y, x));
                    map[y][x] = 0;
                }
            }
        }
        while !queue.is_empty() {
            let poll = queue.remove(0);
            for oy in -1..=1 {
                for ox in -1..=1 {
                    if oy == 0 && ox == 0 {
                        continue;
                    }
                    let ny = poll.0 as i32 + oy;
                    let nx = poll.1 as i32 + ox;
                    if ny >= 0 && ny < map[0].len() as i32 && nx >= 0 && nx < map.len() as i32
                        && map[ny as usize][nx as usize] <= 9
                        && map[ny as usize][nx as usize] != 0 {
                        let ny = ny as usize;
                        let nx = nx as usize;
                        map[ny][nx] += 1;
                        if map[ny][nx] == 10 {
                            snapshot += 1;
                            map[ny][nx] = 0;
                            queue.push((ny, nx));
                        }
                    }
                }
            }
        }
        if snapshot == target {
            return step;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day11_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day11_2()?);
        Ok(())
    }
}
