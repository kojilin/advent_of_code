use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day9_1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/day9.txt")?;
    //2 number must be different
    let mut previous = HashMap::new();
    let mut queue = vec![];
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        let current_value = line.parse::<i64>()?;
        if i < 25 {
            let count = previous.entry(current_value).or_insert(0usize);
            *count += 1;
            queue.push(current_value);
            continue;
        }

        let mut found = false;
        for (&v1, _count) in &previous {
            let v2 = current_value - v1;
            if previous.contains_key(&v2) {
                found = true;
                break;
            }
        }
        if !found {
            return Ok(current_value);
        }
        let head = queue.remove(0);
        let count = previous.get_mut(&head).unwrap();
        if *count > 1 {
            *count -= 1;
        } else {
            previous.remove(&head);
        }
        previous.insert(current_value, 1);
        queue.push(current_value);
    }
    panic!("No answer.")
}

fn solve_day9_2(target: i64) -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/day9.txt")?;
    let lines = input.lines();
    // using 1 to n
    let mut numbers = vec![];
    numbers.push(0);
    for line in lines {
        let value = line.parse::<i64>()?;
        numbers.push(value);
    }

    let mut i = 0;
    let mut j = 0;
    let mut temp = numbers[0];
    while temp != target {
        if temp < target {
            j += 1;
            temp += numbers[j];
        } else {
            i += 1;
            temp -= numbers[i];
        }
    }
    let range = &mut numbers[i..j + 1];
    range.sort();
    return Ok(range[0] + range[range.len() - 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        // 177777905
        println!("Result 1: {}", solve_day9_1()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result 2: {}", solve_day9_2(solve_day9_1()?)?);
        Ok(())
    }
}
