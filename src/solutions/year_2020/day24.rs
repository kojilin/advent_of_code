use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn solve_day24() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day24.txt")?;
    Ok(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {:?}", solve_day24()?);
        Ok(())
    }
}
