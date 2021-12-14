use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

fn solve_day8_1() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day8_2() -> Result<usize, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<Vec<(Vec<String>, Vec<String>)>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day8.txt")?;
    let mut result = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split("|").collect();
        let first = line[0];
        let first: Vec<String> = first.split(" ")
            .filter(|s| !s.is_empty()).map(|s| s.to_owned()).collect();
        let second = line[1];
        let second: Vec<String> = second.split(" ")
            .filter(|s| !s.is_empty()).map(|s| s.to_owned()).collect();
        result.push((first, second));
    }
    return Ok(result);
}


fn solve_1(input: Vec<(Vec<String>, Vec<String>)>) -> i32 {
    return input.iter().flat_map(|(_, second)| second)
        .filter(|str| str.len() == 2 || str.len() == 4 || str.len() == 3 || str.len() == 7)
        .count() as i32;
}

fn solve_2(inputs: Vec<(Vec<String>, Vec<String>)>) -> usize {
    // numbers pattern
    let number_vec = vec![
        "1110111".to_owned(), "0010010".to_owned(), "1011101".to_owned(), "1011011".to_owned(),
        "0111010".to_owned(), "1101011".to_owned(), "1101111".to_owned(), "1010010".to_owned(),
        "1111111".to_owned(), "1111011".to_owned()];
    let numbers: HashSet<String> = HashSet::from_iter(number_vec.clone());

    let mut current = vec![0usize; 7];
    let variations = permutation(0, &mut current, 0);
    let mut answer = 0;
    for input in inputs {
        let samples = &input.0;
        // let variation = vec![2, 5, 6, 0, 1, 3, 4];
        for variation in &variations {
            let mut found = true;
            for sample in samples {
                let chars: HashSet<char> = sample.chars().collect();
                let mut tmp = vec!["0"; 7];
                for i in 0..7 {
                    if chars.contains(&(('a' as u8 + i) as char)) {
                        tmp[variation[i as usize]] = "1";
                    }
                }
                let tmp = tmp.join("");
                if numbers.contains(&tmp) {
                    // println!("{:?}", variation);
                } else {
                    found = false;
                }
            }
            if found {
                let answers = &input.1;
                let mut number = 0;
                for answer in answers {
                    let mut tmp = vec!["0"; 7];
                    for c in answer.chars() {
                        let index = variation[(c as u8 - 'a' as u8) as usize];
                        tmp[index] = "1";
                    }
                    let index = number_vec.iter().position(|r| r.eq(&tmp.join(""))).unwrap();
                    number = number * 10 + index;
                }
                answer += number;
                break;
            }
        }
    }

    return answer;
}

fn permutation(current_index: usize, current: &mut Vec<usize>, used: i32) -> Vec<Vec<usize>> {
    if current_index >= 7 {
        return vec![current.clone()];
    }

    let mut result = vec![];
    for i in 0..7 {
        let value = 1 << i;
        if (value & used) == 0 {
            current[current_index] = i;
            let mut vec1 = permutation(current_index + 1, current, value | used);
            result.append(&mut vec1)
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day8_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day8_2()?);
        Ok(())
    }
}
