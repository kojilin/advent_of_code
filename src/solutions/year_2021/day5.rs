use std::cmp::{max, min};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day5_1() -> Result<i32, Box<dyn Error>> {
    let lines = parse_input()?.into_iter()
        .filter(|p| p.0.0 == p.1.0 || p.0.1 == p.1.1)
        .collect();
    return Ok(solve_first(lines));
}

fn parse_input() -> Result<Vec<((i32, i32), (i32, i32))>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day5.txt")?;
    let lines = input.lines().collect::<Vec<&str>>();
    Ok(lines.iter()
        .map(|line| {
            let points: Vec<(i32, i32)> = line.split("->")
                .map(|part| {
                    let point: Vec<i32> = part.trim().split(",")
                        .map(|part| part.parse::<i32>().unwrap())
                        .collect();
                    return (point[0], point[1]);
                })
                .collect();
            return (points[0], points[1]);
        }).collect())
}

fn solve_first(lines: Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut counters = HashMap::new();
    for line in lines {
        if line.0.0 == line.1.0 {
            let x = line.0.0;
            let from = min(line.0.1, line.1.1);
            let to = max(line.0.1, line.1.1);
            for y in from..to + 1 {
                let count = counters.entry(format!("{}_{}", x, y)).or_insert_with(|| 0);
                *count += 1;
            }
        } else if line.0.1 == line.1.1 {
            let y = line.0.1;
            let from = min(line.0.0, line.1.0);
            let to = max(line.0.0, line.1.0);
            for x in from..to + 1 {
                let count = counters.entry(format!("{}_{}", x, y)).or_insert_with(|| 0);
                *count += 1;
            }
        }
    }
    counters.iter().filter(|counter| *counter.1 > 1).count() as i32
}

fn solve_day5_2() -> Result<i32, Box<dyn Error>> {
    let lines = parse_input()?.into_iter()
        .filter(|p| p.0.0 == p.1.0
            || p.0.1 == p.1.1
                || (p.0.1 - p.1.1).abs() == (p.0.0 - p.1.0).abs()
        )
        .collect();
    return Ok(solve_second(lines));
}


fn solve_second(lines: Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut counters = HashMap::new();
    for line in lines {
        let offset_x = if line.0.0 == line.1.0 {
            0
        } else if line.0.0 > line.1.0 {
            -1
        } else {
            1
        };

        let offset_y = if line.0.1 == line.1.1 {
            0
        } else if line.0.1 > line.1.1 {
            -1
        } else {
            1
        };
        let mut current_x = line.0.0;
        let mut current_y = line.0.1;
        while current_x != line.1.0 || current_y != line.1.1 {
            let count = counters.entry(format!("{}_{}", current_x, current_y)).or_insert_with(|| 0);
            *count += 1;
            current_x += offset_x;
            current_y += offset_y;
        }
        let count = counters.entry(format!("{}_{}", current_x, current_y)).or_insert_with(|| 0);
        *count += 1;
    }
    counters.iter().filter(|counter| *counter.1 > 1).count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day5_1()?);
        Ok(())
    }


    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day5_2()?);
        Ok(())
    }
}

