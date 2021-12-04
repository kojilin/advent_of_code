use std::error::Error;
use std::fs;

fn solve_day3_1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day3.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    Ok(solution(lines))
}

fn solve_day3_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day3.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    Ok(solution2(lines))
}


fn solution(lines: Vec<&str>) -> i64 {
    let input_len = lines.len();
    let digits = lines[0].len();
    let mut counts: Vec<i32> = Vec::with_capacity(digits);
    for _ in 0..digits {
        counts.push(0);
    }
    for line in lines {
        for (index, c) in line.chars().enumerate() {
            match c {
                '1' => counts[index] += 1,
                _ => {}
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for x in counts {
        if x > input_len as i32 / 2 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    return i64::from_str_radix(&gamma, 2).unwrap()
        * i64::from_str_radix(&epsilon, 2).unwrap();
}

fn grouping<'a>(index: usize, lines: &Vec<&'a str>) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut vec1: Vec<&str> = Vec::new();
    let mut vec2: Vec<&str> = Vec::new();
    for line in lines {
        if line.chars().nth(index).unwrap() == '1' {
            vec1.push(line);
        } else {
            vec2.push(line);
        }
    }
    return (vec1, vec2);
}

fn oxygen<'a>(index: usize, lines: &Vec<&'a str>) -> Vec<&'a str> {
    let grouping = grouping(index, lines);
    return if grouping.0.len() > grouping.1.len() {
        grouping.0
    } else if grouping.0.len() < grouping.1.len() {
        grouping.1
    } else {
        grouping.0
    };
}

fn find_oxygen(lines: &Vec<&str>) -> i64 {
    let digits = lines[0].len();
    let mut result = lines.clone();
    for i in 0..digits {
        result = oxygen(i, &result);
        if result.len() == 1 {
            return i64::from_str_radix(result[0], 2).unwrap();
        }
    }
    panic!()
}

fn co2<'a>(index: usize, lines: &Vec<&'a str>) -> Vec<&'a str> {
    let grouping = grouping(index, lines);
    return if grouping.0.len() < grouping.1.len() {
        grouping.0
    } else if grouping.0.len() > grouping.1.len() {
        grouping.1
    } else {
        grouping.1
    };
}

fn find_co2(lines: &Vec<&str>) -> i64 {
    let digits = lines[0].len();
    let mut result = lines.clone();
    for i in 0..digits {
        result = co2(i, &result);
        if result.len() == 1 {
            return i64::from_str_radix(result[0], 2).unwrap();
        }
    }
    panic!()
}

fn solution2(lines: Vec<&str>) -> i64 {
    return find_co2(&lines) * find_oxygen(&lines);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day3_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day3_2()?);
        Ok(())
    }
}
