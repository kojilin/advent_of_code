use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;

fn solve_day13_1() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_1(input: Input) -> i64 {
    let mut current = HashSet::from_iter(input.positions.into_iter());
    for &(c, p) in &input.instructions {
        let mut temp = HashSet::new();
        for &(x, y) in &current {
            if c == 'y' {
                if y < p {
                    temp.insert((x, y));
                } else if y > p {
                    let new_y = p - (y - p);
                    temp.insert((x, new_y));
                }
            } else {
                if x < p {
                    temp.insert((x, y));
                } else if x > p {
                    let new_x = p - (x - p);
                    temp.insert((new_x, y));
                }
            }
        }
        current = temp;
        break;
    }
    current.len() as i64
}

fn solve_day13_2() -> Result<HashSet<(i32, i32)>, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}


fn solve_2(input: Input) -> HashSet<(i32, i32)> {
    let mut current = HashSet::from_iter(input.positions.into_iter());
    for &(c, p) in &input.instructions {
        let mut temp = HashSet::new();
        for &(x, y) in &current {
            if c == 'y' {
                if y < p {
                    temp.insert((x, y));
                } else if y > p {
                    let new_y = p - (y - p);
                    temp.insert((x, new_y));
                }
            } else {
                if x < p {
                    temp.insert((x, y));
                } else if x > p {
                    let new_x = p - (x - p);
                    temp.insert((new_x, y));
                }
            }
        }
        current = temp;
    }
    current
}


struct Input {
    positions: Vec<(i32, i32)>,
    instructions: Vec<(char, i32)>,
}

fn parse_input() -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day13.txt")?;
    let mut points = vec![];
    let mut instructions = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("fold") {
            if let Some(index) = line.chars().position(|c| c == '=') {
                let c = line.chars().nth(index - 1).unwrap();
                let instruction: Vec<&str> = line.split('=').collect();
                instructions.push((c, i32::from_str(instruction[1]).unwrap()));
            }
        } else {
            let point: Vec<i32> = line.split(',').map(|str| i32::from_str(str).unwrap())
                .collect();
            let x = point[0];
            let y = point[1];
            points.push((x, y));
        }
    }

    return Ok(Input {
        positions: points,
        instructions,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day13_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        let set = solve_day13_2()?;
        for y in 0..6 {
            for x in 0..40 {
                if set.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        Ok(())
    }
}
