use regex::Regex;
use std::cmp::min;
use std::error::Error;
use std::fs;

fn solve_day13() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day13.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut index = 0;
    let mut a = (-1, -1);
    let mut b = (-1, -1);
    let mut target = (-1, -1);
    let mut result = 0;
    while index < lines.len() {
        if let Some(captures) = Regex::new(r"X\+(\d+),\s*Y\+(\d+)")
            .unwrap()
            .captures(lines[index])
        {
            let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

            a = (x, y);
        }
        index += 1;

        if let Some(captures) = Regex::new(r"X\+(\d+),\s*Y\+(\d+)")
            .unwrap()
            .captures(lines[index])
        {
            let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

            b = (x, y);
        }
        index += 1;

        if let Some(captures) = Regex::new(r"X=(\d+),\s*Y=(\d+)")
            .unwrap()
            .captures(lines[index])
        {
            let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

            target = (x, y);
        }
        index += 1;
        // may empty line
        index += 1;

        // solve here
        let mut first = 0;
        let mut possible_result = i64::MAX;
        // println!("check for {:?} & {:?} = {:?}", a, b, target);
        while first * a.0 <= target.0 && first * a.1 <= target.1 {
            if (target.0 - first * a.0) % b.0 == 0
                && (target.1 - first * a.1) % b.1 == 0
                && (target.0 - first * a.0) / b.0 == (target.1 - first * a.1) / b.1
            {
                possible_result = min(possible_result, first * 3 + (target.0 - first * a.0) / b.0);
            }
            first += 1;
        }
        if possible_result != i64::MAX {
            result += possible_result;
        }
    }
    Ok(result)
}

fn solve_day13_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day13.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut index = 0;
    let mut a = (-1, -1);
    let mut b = (-1, -1);
    let mut target = (-1, -1);
    let mut result = 0;
    while index < lines.len() {
        if let Some(captures) = Regex::new(r"X\+(\d+),\s*Y\+(\d+)")
            .unwrap()
            .captures(lines[index])
        {
            let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

            a = (x, y);
        }
        index += 1;

        if let Some(captures) = Regex::new(r"X\+(\d+),\s*Y\+(\d+)")
            .unwrap()
            .captures(lines[index])
        {
            let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

            b = (x, y);
        }
        index += 1;

        if let Some(captures) = Regex::new(r"X=(\d+),\s*Y=(\d+)")
            .unwrap()
            .captures(lines[index])
        {
            let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

            target = (10000000000000 + x, 10000000000000 + y);
            // target = (x, y);
        }
        index += 1;
        // may empty line
        index += 1;
        let right = (target.0 * b.1 - target.1 * b.0);
        let x = b.1 * a.0 - a.1 * b.0;
        if right % x != 0 {
            continue;
        }

        let first = right / x;
        let remain = target.0 - first * a.0;
        if remain % b.0 != 0 {
            continue;
        }
        let second = remain / b.0;
        result += first * 3 + second;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_1() {
        println!("result: {:?}", solve_day13());
    }

    #[test]
    fn test_day13_2() {
        println!("result: {:?}", solve_day13_2());
    }
}
