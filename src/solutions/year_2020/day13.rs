use std::error::Error;
use std::fs;

fn solve_day13() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day13.txt")?;
    Ok(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day13()?);
        Ok(())
    }
}
