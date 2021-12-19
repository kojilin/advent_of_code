use std::error::Error;
use std::fs;

fn solve_day10_1() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day10_2() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day10.txt")?;
    let mut result = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for c in line.chars() {
            line_vec.push(c);
        }
        result.push(line_vec);
    }
    return Ok(result);
}

fn solve_1(inputs: Vec<Vec<char>>) -> i64 {
    let mut score = 0;
    for line in inputs {
        let mut stack = vec![];
        for c in line {
            match c {
                '(' => {
                    stack.push('(')
                }
                ')' => {
                    if stack.is_empty() || *stack.last().unwrap() != '(' {
                        score += 3;
                        break;
                    }
                    stack.pop();
                }
                '[' => {
                    stack.push('[');
                }
                ']' => {
                    if stack.is_empty() || *stack.last().unwrap() != '[' {
                        score += 57;
                        break;
                    }
                    stack.pop();
                }
                '{' => {
                    stack.push('{');
                }
                '}' => {
                    if stack.is_empty() || *stack.last().unwrap() != '{' {
                        score += 1197;
                        break;
                    }
                    stack.pop();
                }
                '<' => {
                    stack.push('<');
                }
                '>' => {
                    if stack.is_empty() || *stack.last().unwrap() != '<' {
                        score += 25137;
                        break;
                    }
                    stack.pop();
                }
                _ => panic!()
            }
        };
    }
    return score;
}

fn solve_2(inputs: Vec<Vec<char>>) -> i64 {
    let mut scores = vec![];
    for line in inputs {
        let mut stack = vec![];
        let mut incomplete = true;
        for c in line {
            match c {
                '(' => {
                    stack.push('(')
                }
                ')' => {
                    if stack.is_empty() || *stack.last().unwrap() != '(' {
                        incomplete = false;
                        break;
                    }
                    stack.pop();
                }
                '[' => {
                    stack.push('[');
                }
                ']' => {
                    if stack.is_empty() || *stack.last().unwrap() != '[' {
                        incomplete = false;
                        break;
                    }
                    stack.pop();
                }
                '{' => {
                    stack.push('{');
                }
                '}' => {
                    if stack.is_empty() || *stack.last().unwrap() != '{' {
                        incomplete = false;
                        break;
                    }
                    stack.pop();
                }
                '<' => {
                    stack.push('<');
                }
                '>' => {
                    if stack.is_empty() || *stack.last().unwrap() != '<' {
                        incomplete = false;
                        break;
                    }
                    stack.pop();
                }
                _ => panic!()
            }
        };
        // scores
        if incomplete {
            let mut score = 0i64;
            for &c in stack.iter().rev() {
                score *= 5;
                if c == '(' {
                    score += 1;
                } else if c == '[' {
                    score += 2;
                } else if c == '{' {
                    score += 3;
                } else if c == '<' {
                    score += 4;
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    return *scores.get(scores.len() / 2).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day10_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day10_2()?);
        Ok(())
    }
}
