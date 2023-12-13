use std::collections::HashMap;
use std::error::Error;
use std::fs;


fn solve_day8() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day8.txt")?;
    let mut lines = content.lines();
    let instruction: Vec<char> = lines.next().unwrap().chars().collect();
    let mut iter = lines.skip(1);
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in iter {
        let key = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        map.insert(key, (left, right));
    }

    let mut count = 0;
    let mut current = "AAA";
    let mut index = 0usize;
    while current != "ZZZ" {
        let tos = map.get(current).unwrap();
        let l_or_r = instruction[index % instruction.len()];
        current = if l_or_r == 'R' {
            &tos.1
        } else {
            &tos.0
        };
        index += 1;
        count += 1;
    }

    Ok(count)
}

fn solve_day8_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day8.txt")?;
    let mut lines = content.lines();
    let instruction: Vec<char> = lines.next().unwrap().chars().collect();
    let mut iter = lines.skip(1);
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut froms: Vec<String> = Vec::new();
    for line in iter {
        let key = line[0..3].to_string();
        if key.ends_with("A") {
            froms.push(key.clone());
        }
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        map.insert(key, (left, right));
    }


    let mut result = 1i64;
    for i in 0..froms.len() {
        let mut next = &froms[i];
        let mut index = 0usize;
        let mut steps = 0;
        let mut all_steps: Vec<i32> = Vec::new();
        loop {
            let tos = map.get(next).unwrap();
            let l_or_r = instruction[index % instruction.len()];
            next = if l_or_r == 'R' {
                &tos.1
            } else {
                &tos.0
            };

            steps += 1;
            if next.ends_with("Z") {
                // lucky that the question is not a + bn but just bn;
                if !all_steps.is_empty() && all_steps.last().unwrap().eq(&steps) {
                    if result == 1 {
                        result = result * (*all_steps.last().unwrap() as i64);
                    } else {
                        let gcd = gcd(result, *all_steps.last().unwrap() as i64);
                        result = gcd * (result / gcd) * (*all_steps.last().unwrap() as i64 / gcd);
                    }

                    break;
                }
                all_steps.push(steps);
                steps = 0;
            }
            index += 1;
        }
    }
    Ok(result)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b % a == 0 {
        return a;
    }
    if a > b {
        return gcd(b, a);
    }
    return gcd(b % a, a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result 1:{}", solve_day8()?);
        Ok(())
    }

    #[test]
    fn test_day8_2() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result 2:{}", solve_day8_2()?);
        Ok(())
    }
}
