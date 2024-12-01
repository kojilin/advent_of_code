use std::fs;
use std::error::Error;

fn solve_day12() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day12.txt")?;
    let mut lines = content.lines();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let prefix = split[0];
        let postfix = split[1];
        let request: Vec<usize> = postfix.split(",").map(|s| s.parse().unwrap()).collect();
        println!("{:?}", pattern_of(prefix));
    }
    Ok(0)
}

fn pattern_of(input: &str) -> Vec<usize> {
    let mut result = vec![];
    let mut count = 0;
    for c in input.chars() {
        match c {
            '#' => { count += 1 }
            '.' => {
                if count != 0 {
                    result.push(count);
                };
                count = 0
            }
            '?' => { break; }
            _ => { panic!("wrong input") }
        }
    }
    if count != 0 {
        result.push(count);
    }
    result
}

fn solve_day12_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day12.txt")?;
    let mut lines = content.lines();
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day12()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day12_2()?);
        Ok(())
    }
}
