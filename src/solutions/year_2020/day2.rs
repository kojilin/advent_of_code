use std::error::Error;
use std::fs;
use std::ops::Deref;

#[derive(Debug)]
struct Input {
    min: i64,
    max: i64,
    c: char,
    password: String,
}

impl Input {
    fn is_valid_by_count(&self) -> bool {
        let mut count = 0;
        for v in self.password.chars() {
            if v == self.c {
                count += 1;
            }
            if count > self.max {
                return false;
            }
        }
        count >= self.min
    }

    fn is_valid_by_position(&self) -> bool {
        let mut count = 0;
        let chars: Vec<char> = self.password.chars().collect();
        let first = chars[self.min as usize - 1];
        let second = chars[self.max as usize - 1];
        if first == self.c {
            count += 1;
        }
        if second == self.c {
            count += 1;
        }
        count == 1
    }
}

fn solve_day2(valid_function: fn(&Input) -> bool) -> Result<usize, Box<dyn Error>> {
    let file_content = fs::read_to_string("src/solutions/year_2020/day2.txt")?;
    let inputs: Vec<Input> = file_content.lines().map(|s| {
        let line: Vec<&str> = s.split_whitespace().collect();
        let range: Vec<i64> = line[0]
            .split('-').map(|s| s.parse::<i64>().unwrap())
            .collect();
        Input {
            min: range[0],
            max: range[1],
            c: line[1].replace(":", "").chars().next().unwrap(),
            password: line[2].to_owned(),
        }
    }).collect();
    Ok(solution(&inputs, valid_function))
}

fn solution(inputs: &Vec<Input>, is_valid: fn(&Input) -> bool) -> usize {
    inputs.iter().filter(|x| is_valid(x.deref())).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(solution(&vec![
            Input { min: 1, max: 3, c: 'a', password: String::from("abcde") },
            Input { min: 1, max: 3, c: 'b', password: String::from("cdefg") },
            Input { min: 2, max: 9, c: 'c', password: String::from("ccccccccc") }
        ], Input::is_valid_by_count), 2);
        println!("-----real-----");
        println!("result:{}", solve_day2(Input::is_valid_by_count)?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(solution(&vec![
            Input { min: 1, max: 3, c: 'a', password: String::from("abcde") },
            Input { min: 1, max: 3, c: 'b', password: String::from("cdefg") },
            Input { min: 2, max: 9, c: 'c', password: String::from("ccccccccc") }
        ], Input::is_valid_by_position), 1);
        println!("-----real-----");
        println!("result:{}", solve_day2(Input::is_valid_by_position)?);
        Ok(())
    }
}
