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
    let mut bi = 0;
    let mut delta = 1;
    let mut time = 0;
    while bi < pairs.len() {
        let interval = pairs[bi].0;
        let count = pairs[bi].1;

        if (time + count) % interval == 0 {
            delta *= interval;
            bi += 1;
            continue;
        }
        time += delta;
    }
    Ok(time)
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
        println!("Result2: {}", solve_day13_2()?);
        Ok(())
    }
}
