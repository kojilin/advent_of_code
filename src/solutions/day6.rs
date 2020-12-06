use std::collections::HashSet;
use std::error::Error;

fn solve_day6() -> Result<usize, Box<dyn Error>> {
    let input = std::fs::read_to_string("src/solutions/day6.txt")?;
    let mut answer = HashSet::new();
    let mut result = 0;
    let mut new_group = true;
    for line in input.lines() {
        if !line.is_empty() {
            let cs: Vec<char> = line.chars().collect();
            if new_group {
                for &c in &cs {
                    answer.insert(c);
                }
                new_group = false;
            } else {
                answer = answer.iter()
                    .filter(|&remain| cs.contains(remain))
                    .map(|c| c.to_owned())
                    .collect();
            }
        } else {
            result += answer.len();
            answer = HashSet::new();
            new_group = true;
        }
    }
    result += answer.len();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        let result = solve_day6()?;
        println!("-----real-----");
        println!("Result: {:?}", result);
        Ok(())
    }
}
