use std::error::Error;
use std::fs;

use crate::solutions::year_2020::day18::Operator::{Multiply, Plus};

fn solve_day18() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day18.txt")?;
    let mut sum = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        sum += calculate(&mut 0, &chars);
    }
    Ok(sum)
}

enum Operator {
    Plus,
    Multiply,
}

fn calculate(cursor: &mut usize, chars: &Vec<char>) -> i64 {
    let mut sum = 0;
    let mut operator = Plus;
    while *cursor < chars.len() {
        match chars[*cursor] {
            '(' => {
                *cursor += 1;
                match operator {
                    Plus => sum += calculate(cursor, chars),
                    Multiply => sum *= calculate(cursor, chars),
                }
            }
            ')' => {
                *cursor += 1;
                break;
            }
            '+' => {
                operator = Plus;
                *cursor += 1;
            }
            '*' => {
                operator = Multiply;
                *cursor += 1;
            }
            c if c.is_digit(10) => {
                let number = read_digit(cursor, chars);
                match operator {
                    Plus => sum += number,
                    Multiply => sum *= number,
                }
            }
            others => panic!("Wrong input: {}.", others)
        }
        skip_space(cursor, chars);
    }
    sum
}

fn skip_space(cursor: &mut usize, chars: &Vec<char>) {
    while *cursor < chars.len() && chars[*cursor] == ' ' {
        *cursor += 1;
    }
}

fn read_digit(cursor: &mut usize, chars: &Vec<char>) -> i64 {
    let from = *cursor;
    while *cursor < chars.len() && chars[*cursor].is_digit(10) {
        *cursor += 1;
    }
    chars[from..*cursor].iter().collect::<String>().parse::<i64>().unwrap()
}

fn solve_day18_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day18.txt")?;
    let mut sum = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        sum += calculate_2(&mut 0, &chars);
    }
    Ok(sum)
}

fn calculate_2(cursor: &mut usize, chars: &Vec<char>) -> i64 {
    skip_space(cursor, chars);
    let mut sum = 0;
    while *cursor < chars.len() {
        match chars[*cursor] {
            '(' => {
                *cursor += 1;
                let value = calculate_2(cursor, chars);
                sum += value;
            }
            ')' => {
                *cursor += 1;
                break;
            }
            '+' => {
                *cursor += 1;
            }
            '*' => {
                *cursor += 1;
                let value = calculate_2(cursor, chars);
                if sum == 0 {
                    sum = value;
                } else {
                    sum *= value;
                }
                break;
            }
            c if c.is_digit(10) => {
                sum += read_digit(cursor, chars);
            }
            others => panic!("Wrong input: '{}'.", others)
        }
        skip_space(cursor, chars);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day18()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day18_2()?);
        Ok(())
    }
}
