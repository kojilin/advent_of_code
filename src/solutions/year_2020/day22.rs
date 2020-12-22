use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn solve_day22() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day22.txt")?;
    let mut first = vec![];
    let mut second = vec![];
    let mut current = &mut first;
    for line in input.lines() {
        if line.is_empty() || line.contains("Player 1:") {
            continue;
        }
        if line.contains("Player 2:") {
            current = &mut second;
            continue;
        }
        current.push(line.parse::<i64>()?);
    }
    while !first.is_empty() && !second.is_empty() {
        let a = first.remove(0);
        let b = second.remove(0);
        if a > b {
            first.push(a);
            first.push(b);
        } else {
            second.push(b);
            second.push(a);
        }
    }
    let result;
    if first.is_empty() {
        result = second;
    } else {
        result = first;
    }

    Ok(result.iter()
        .rev()
        .enumerate()
        .map(|(index, &number)| (index + 1) as i64 * number)
        .sum())
}

fn solve_day22_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day22.txt")?;
    let mut first = vec![];
    let mut second = vec![];
    let mut current = &mut first;
    for line in input.lines() {
        if line.is_empty() || line.contains("Player 1:") {
            continue;
        }
        if line.contains("Player 2:") {
            current = &mut second;
            continue;
        }
        current.push(line.parse::<i64>()?);
    }
    Ok(play_round(first.clone(), second.clone(), 1).1)
}

fn score(result: &Vec<i64>) -> i64 {
    result.iter()
        .rev()
        .enumerate()
        .map(|(index, &number)| (index + 1) as i64 * number)
        .sum()
}

// true -> p1 win, false -> p2 win, second is score
fn play_round(mut p1: Vec<i64>, mut p2: Vec<i64>, game: usize) -> (bool, i64) {
    let mut p1_memory = HashSet::new();
    let mut p2_memory = HashSet::new();

    loop {
        let snapshot_p1 = p1.iter().map(|p| p.to_string()).collect::<Vec<String>>().join("|");
        let snapshot_p2 = p2.iter().map(|p| p.to_string()).collect::<Vec<String>>().join("|");
        if p1_memory.contains(&snapshot_p1) || p2_memory.contains(&snapshot_p2) {
            return (true, score(&p1));
        }
        if p1.is_empty() {
            return (false, score(&p2));
        }
        if p2.is_empty() {
            return (true, score(&p1));
        }
        let a = p1.remove(0);
        let b = p2.remove(0);

        if p1.len() >= a as usize && p2.len() >= b as usize {
            if play_round(p1[0..a as usize].to_vec(), p2[0..b as usize].to_vec(), game + 1).0 {
                p1.push(a);
                p1.push(b);
            } else {
                p2.push(b);
                p2.push(a);
            }
        } else {
            if a > b {
                p1.push(a);
                p1.push(b);
            } else {
                p2.push(b);
                p2.push(a);
            }
        }
        p1_memory.insert(snapshot_p1);
        p2_memory.insert(snapshot_p2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day22()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day22_2()?);
        Ok(())
    }
}
