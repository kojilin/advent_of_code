use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day14_1() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day14_2() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<(String, HashMap<String, String>), Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day14.txt")?;
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap();
    let mut map = HashMap::new();
    lines.next();

    for s in lines {
        let split: Vec<&str> = s.split(" -> ").collect();
        map.insert(split[0].to_owned(), split[1].to_owned());
    }

    Ok((polymer_template.to_owned(), map))
}

fn solve_1(input: (String, HashMap<String, String>)) -> i32 {
    let mut from = input.0;
    let dict = &input.1;
    for _ in 0..10 {
        let mut temp = String::new();
        let chars: Vec<char> = from.chars().collect();
        for i in 0..from.len() - 1 {
            let c1 = chars[i];
            let c2 = chars[i + 1];
            let insert = dict.get(&format!("{}{}", c1, c2)).unwrap();
            temp.push(c1);
            temp.push_str(insert);
        }
        temp.push(*chars.last().unwrap());
        from = temp;
    }
    let chars: Vec<char> = from.chars().collect();
    let mut map = HashMap::new();
    for c in chars {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    let mut values: Vec<i32> = map.values().cloned().collect();
    values.sort();
    values.last().unwrap() - values[0]
}


fn solve_2(input: (String, HashMap<String, String>)) -> i64 {
    let from = input.0;
    let dict = &input.1;
    let mut dp = HashMap::new();
    let chars: Vec<char> = from.chars().collect();
    let mut counts = HashMap::new();
    for &c in &chars {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    for i in 0..from.len() - 1 {
        let c1 = chars[i];
        let c2 = chars[i + 1];
        let result = do_solve(c1, c2, 40, &mut dp, &dict);
        counts = merge(counts, result)
    }
    let mut values: Vec<i64> = counts.values().cloned().collect();
    values.sort();
    values.last().unwrap() - values[0]
}

fn do_solve(c1: char, c2: char, round: i32, dp: &mut HashMap<String, HashMap<char, i64>>, dict: &HashMap<String, String>) -> HashMap<char, i64> {
    if round == 0 {
        return HashMap::new();
    }
    let key = format!("{}{}_{}", c1, c2, round);
    if let Some(value) = dp.get(&key) {
        return value.clone();
    }

    let target = dict.get(&format!("{}{}", c1, c2)).unwrap();
    let middle = target.chars().next().unwrap();
    let left = do_solve(c1, middle, round - 1, dp, dict);
    let right = do_solve(middle, c2, round - 1, dp, dict);
    let mut result = merge(left, right);
    let count = result.entry(middle).or_insert(0);
    *count += 1;
    dp.insert(key, result.clone());
    result
}

fn merge(left: HashMap<char, i64>, right: HashMap<char, i64>) -> HashMap<char, i64> {
    let mut counts = HashMap::new();
    for i in 0..26 {
        let c = ('A' as u8 + i as u8) as char;
        if let Some(lv) = left.get(&c) {
            let count = counts.entry(c).or_insert(0);
            *count += lv;
        }
        if let Some(lv) = right.get(&c) {
            let count = counts.entry(c).or_insert(0);
            *count += lv;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day14_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day14_2()?);
        Ok(())
    }
}
