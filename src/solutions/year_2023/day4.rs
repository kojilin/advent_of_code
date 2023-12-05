use std::collections::HashSet;
use std::error::Error;
use std::fs;
use regex::Regex;

fn solve_day4() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day4.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    let re = Regex::new(r"\b\d+\b").unwrap();
    let prefix = Regex::new(r"Card\s+\d+:").unwrap();
    let mut total = 0;
    for line in lines {
        let line = prefix.replace(line, "");
        let mut count = 0;
        let mut score = 0;
        let mut set = HashSet::new();
        for captures in re.captures_iter(&line) {
            let value = captures[0].parse::<i32>()?;

            if count < 10 {
                set.insert(value);
            } else if set.contains(&value) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
            count += 1;
        }
        total += score
    }
    Ok(total)
}


fn solve_day4_2() -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day4.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    let re = Regex::new(r"\b\d+\b").unwrap();
    let prefix = Regex::new(r"Card\s+\d+:").unwrap();
    let mut cards = vec![1; lines.len() + 1];
    cards[0] = 0;
    let mut index = 1usize;
    for line in lines {
        let line = prefix.replace(line, "");
        let mut count = 0;
        let mut copies = 0;
        let mut set = HashSet::new();
        for captures in re.captures_iter(&line) {
            let value = captures[0].parse::<i32>()?;
            if count < 10 {
                set.insert(value);
            } else if set.contains(&value) {
                copies += 1;
            }
            count += 1;
        }
        for i in (index + 1)..=(index + copies) {
            cards[i] += cards[index]
        }
        index += 1;
    }

    Ok(cards.iter().to_owned().sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day4()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day4_2()?);
        Ok(())
    }
}