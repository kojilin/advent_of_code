use std::error::Error;
use std::fs;

fn solve_day13() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day13.txt")?;
    let mut lines = input.lines();
    let arrive_at = lines.next().unwrap().parse::<i64>()?;
    let intervals: Vec<i64> = lines.next().unwrap()
        .split(",")
        .filter(|s| !s.eq(&"x"))
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut result = std::i64::MAX;
    let mut current_wait = std::i64::MAX;
    for interval in intervals {
        let wait = interval - arrive_at % interval;
        if current_wait > wait {
            result = wait * interval;
            current_wait = wait;
        }
    }
    Ok(result)
}


fn solve_day13_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day13.txt")?;
    let mut lines = input.lines();
    lines.next();
    let mut count = 0;
    let mut pairs = vec![];
    for c in lines.next().unwrap().split(",") {
        if !c.eq("x") {
            pairs.push((c.parse::<i64>()?, count));
        }
        count += 1;
    }

    println!("{:?}", pairs);

    let first = pairs[0].0;
    let mut result = first;
    let mut a = 'a';
    let mut count = 0;
    for &(number, index) in pairs.iter().skip(1) {
        // put this to wolframalpha -> get a's possible result -> use smallest number.
        print!("{}{}-{}x={}, ", number, (a as u8 + count) as char, first, index);
        count += 1;
        if (a as u8 + count) as char == 'e' {
            // wolframalpha will convert e. 19e-37x=56 -> 19x10^-37x=56
            count += 1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day13()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        solve_day13_2()?;
        Ok(())
    }
}
