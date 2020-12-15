use std::collections::HashMap;
use std::error::Error;

fn solve_day15(input: Vec<usize>, turn: usize) -> Result<usize, Box<dyn Error>> {
    let mut last_index = HashMap::with_capacity(turn);
    let mut current = 0;
    for i in 0..turn {
        if i < input.len() {
            if i > 0 {
                last_index.insert(current, i - 1);
            }
            current = input[i];
        } else {
            if last_index.contains_key(&current) {
                let &prev_index = last_index.get(&current).unwrap();
                last_index.insert(current, i - 1);
                current = i - 1 - prev_index;
            } else {
                last_index.insert(current, i - 1);
                current = 0;
            }
        }
    }
    Ok(current)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Sample1: {}", solve_day15(vec![0, 3, 6], 10)?);
        println!("Result1: {}", solve_day15(vec![2, 0, 1, 7, 4, 14, 18], 2020)?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Sample2: {}", solve_day15(vec![0, 3, 6], 30_000_000)?);
        println!("Result2: {}", solve_day15(vec![2, 0, 1, 7, 4, 14, 18], 30_000_000)?);
        Ok(())
    }
}
