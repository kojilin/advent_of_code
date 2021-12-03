use std::error::Error;
use std::fs;

fn solve_day2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day2.txt")?;
    Ok(solution(input.lines().collect()))
}

fn solve_day2_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day2.txt")?;
    Ok(solution2(input.lines().collect()))
}

fn solution(commands: Vec<&str>) -> i64 {
    let mut y = 0i64;
    let mut x = 0i64;
    for (command, value) in commands.iter().map(|s|
        {
            let line: Vec<&str> = s.split_whitespace().collect();
            return (line[0], line[1].parse::<i64>().unwrap());
        }
    ) {
        match command {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            _ => panic!()
        }
    }
    return x * y;
}

fn solution2(commands: Vec<&str>) -> i64 {
    let mut aim = 0;
    let mut y = 0i64;
    let mut x = 0i64;
    for (command, value) in commands.iter().map(|s|
        {
            let line: Vec<&str> = s.split_whitespace().collect();
            return (line[0], line[1].parse::<i64>().unwrap());
        }
    ) {
        match command {
            "forward" => {
                x += value;
                y += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!()
        }
    }
    return x * y;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day2()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day2_2()?);
        Ok(())
    }
}
