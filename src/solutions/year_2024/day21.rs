use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_first_robot(line: &str, robot_count: i64) -> i64 {
    let mut current = 'A';
    let mut pad = HashMap::new();
    // 2nd is (y,x)
    pad.insert('7', (0, 0));
    pad.insert('8', (0, 1));
    pad.insert('9', (0, 2));
    pad.insert('4', (1, 0));
    pad.insert('5', (1, 1));
    pad.insert('6', (1, 2));
    pad.insert('1', (2, 0));
    pad.insert('2', (2, 1));
    pad.insert('3', (2, 2));
    pad.insert('0', (3, 1));
    pad.insert('A', (3, 2));

    let mut dp = HashMap::new();
    let mut result = 0;
    for c in line.chars() {
        let mut next_input = String::new();
        let from = pad.get(&current).unwrap();
        let to = pad.get(&c).unwrap();
        if from.1 == 0 && to.0 == 3 {
            // must do horizontal first
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            next_input += "A";
            result += solve_other_robots(c, &next_input, robot_count, &mut dp);
        } else if from.0 == 3 && to.1 == 0 {
            // must do vertical fist
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            next_input += "A";
            result += solve_other_robots(c, &next_input, robot_count, &mut dp);
        } else {
            //need to try both
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            next_input += "A";
            let mut tmp_result = solve_other_robots(c, &next_input, robot_count, &mut dp);
            next_input.clear();
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            next_input += "A";
            let second_result = solve_other_robots(c, &next_input, robot_count, &mut dp);
            if second_result < tmp_result {
                tmp_result = second_result;
            }
            result += tmp_result;
        }
        current = c;
    }

    result
}
fn solve_other_robots(
    debug_current: char,
    input: &String,
    indirect_count: i64,
    dp: &mut HashMap<(String, i64), i64>,
) -> i64 {
    let key = (input.clone(), indirect_count);
    if dp.contains_key(&key) {
        // println!("hit");
        return *dp.get(&key).unwrap();
    }

    // println!("{}/{} > {:?}", debug_current, indirect_count, input);
    if indirect_count == 0 {
        return input.len() as i64;
    }
    let mut pad = HashMap::new();
    // 2nd is (y,x)
    pad.insert('^', (0, 1));
    pad.insert('A', (0, 2));
    pad.insert('<', (1, 0));
    pad.insert('v', (1, 1));
    pad.insert('>', (1, 2));

    let mut current = 'A';
    let mut result = 0;
    for c in input.chars() {
        let mut next_input = String::new();
        let from = pad.get(&current).unwrap();
        let to = pad.get(&c).unwrap();
        if from.1 == 0 {
            // must do horizontal first
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            next_input += "A";
            result += solve_other_robots(c, &next_input, indirect_count - 1, dp);
        } else if to.1 == 0 {
            // must do vertical first
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            next_input += "A";
            result += solve_other_robots(c, &next_input, indirect_count - 1, dp);
        } else {
            // choose one of it
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            next_input += "A";
            let tmp = solve_other_robots(c, &next_input, indirect_count - 1, dp);
            next_input.clear();

            let diff_x: i64 = to.1 - from.1;
            if diff_x > 0 {
                next_input.push_str(&">".repeat(diff_x as usize));
            } else {
                next_input.push_str(&"<".repeat(diff_x.abs() as usize));
            }
            let diff_y: i64 = to.0 - from.0;
            if diff_y > 0 {
                next_input.push_str(&"v".repeat(diff_y as usize));
            } else {
                next_input.push_str(&"^".repeat(diff_y.abs() as usize));
            }
            next_input += "A";
            let second_tmp = solve_other_robots(c, &next_input, indirect_count - 1, dp);
            if second_tmp < tmp {
                result += second_tmp;
            } else {
                result += tmp;
            }
        }

        current = c;
    }
    dp.insert(key, result.clone());
    result
}

fn solve_day21() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day21.txt")?;
    let mut answer = 0;
    for line in input.lines() {
        let result = solve_first_robot(line, 2);
        answer += line[0..line.len() - 1].parse::<i64>().unwrap() * result as i64;
        println!(
            "{}*{}",
            line[0..line.len() - 1].parse::<i64>().unwrap(),
            result
        )
    }
    Ok(answer)
}

fn solve_day21_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day21.txt")?;
    let mut answer = 0;
    for line in input.lines() {
        let result = solve_first_robot(line, 25);
        answer += line[0..line.len() - 1].parse::<i64>().unwrap() * result as i64;
        println!(
            "{}*{}",
            line[0..line.len() - 1].parse::<i64>().unwrap(),
            result
        )
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day21()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day21_2()?);
        Ok(())
    }
}
