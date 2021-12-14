use std::cmp::min;
use std::error::Error;
use std::fs;

fn solve_day7_1() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day7_2() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day7.txt")?;
    return Ok(input.lines().nth(0).unwrap()
        .split(",")
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.parse::<i32>().unwrap()
        }).collect());
}


fn solve_1(crabs: Vec<i32>) -> i32 {
    let mut total = crabs.iter().sum::<i32>();
    let &max = crabs.iter().max().unwrap();
    let max = max as usize;

    let mut array = vec![0; max + 1];
    for &v in &crabs {
        array[v as usize] += 1;
    }
    let mut add = vec![0; max + 1];
    for index in 0..max + 1 {
        if index == 0 {
            add[index] = array[index];
        } else {
            add[index] = array[index] + add[index - 1];
        }
    }
    let mut answer = total;
    for i in 1..max + 1 {
        total = total + add[i - 1] - array[i] - (add[max] - add[i]);
        answer = min(answer, total);
    }
    return answer;
}

fn solve_2(crabs: Vec<i32>) -> i64 {
    let &max = crabs.iter().max().unwrap();
    let max = max as usize;

    let mut pre_calculate = vec![0; max + 1];
    for i in 1..pre_calculate.len() {
        pre_calculate[i] = pre_calculate[i - 1] + i;
    }

    let mut array = vec![0; max + 1];
    for &v in &crabs {
        array[v as usize] += 1;
    }

    let mut total = vec![0i64; max + 1];
    for i in 0..max + 1 {
        if array[i] == 0 {
            continue;
        }
        let count = array[i];
        for j in 0..max + 1 {
            if i > j {
                total[j] += pre_calculate[i - j] as i64 * count;
            } else {
                total[j] += pre_calculate[j - i] as i64 * count;
            }
        }
    }
    return *total.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day7_1()?);
        Ok(())
    }


    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day7_2()?);
        Ok(())
    }
}
