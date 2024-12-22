use regex::Regex;
use std::error::Error;
use std::fs;
use std::ops::BitXor;

struct Register {
    a: i64,
    b: i64,
    c: i64,
}

impl Register {
    pub fn new(a: i64, b: i64, c: i64) -> Self {
        Self { a, b, c }
    }
}

fn solve_day17() -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day17.txt")?;
    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut a_value = re
        .find_iter(lines[0])
        .next()
        .unwrap()
        .as_str()
        .parse::<i64>()?;
    let mut b_value = re
        .find_iter(lines[1])
        .next()
        .unwrap()
        .as_str()
        .parse::<i64>()?;
    let mut c_value = re
        .find_iter(lines[2])
        .next()
        .unwrap()
        .as_str()
        .parse::<i64>()?;
    let program: Vec<i64> = lines[4]
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut register = Register::new(a_value, b_value, c_value);

    let mut index = 0;
    let mut result = Vec::new();
    while index < program.len() {
        let op_code = program[index];
        let operand = program[index + 1];
        match op_code {
            0 => {
                adv(operand, &mut register);
            }
            1 => {
                bxl(operand, &mut register);
            }
            2 => {
                bst(operand, &mut register);
            }
            3 => {
                if register.a != 0 {
                    index = operand as usize;
                    continue;
                }
            }
            4 => {
                bxc(&mut register);
            }
            5 => {
                out(operand, &register, &mut result);
            }
            6 => {
                bdv(operand, &mut register);
            }
            7 => {
                cdv(operand, &mut register);
            }
            _ => {}
        }
        index += 2;
    }

    Ok(result
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

fn out(operand: i64, register: &Register, result: &mut Vec<i64>) {
    let resolved_operand = combo(operand, register);
    result.push(resolved_operand % 8);
}

fn bst(operand: i64, register: &mut Register) {
    let resolved_operand = combo(operand, register);
    register.b = resolved_operand % 8;
}

fn bxc(register: &mut Register) {
    register.b = register.b.bitxor(register.c);
}

fn bxl(operand: i64, register: &mut Register) {
    register.b = register.b.bitxor(operand);
}

fn adv(operand: i64, register: &mut Register) {
    let resolved_operand = combo(operand, register);
    register.a = register.a / 2_i32.pow(resolved_operand as u32) as i64;
}

fn bdv(operand: i64, register: &mut Register) {
    let resolved_operand = combo(operand, register);
    register.b = register.a / 2_i32.pow(resolved_operand as u32) as i64;
}
fn cdv(operand: i64, register: &mut Register) {
    let resolved_operand = combo(operand, register);
    register.c = register.a / 2_i32.pow(resolved_operand as u32) as i64;
}

fn combo(operand: i64, register: &Register) -> i64 {
    if operand >= 0 && operand <= 3 {
        return operand;
    } else if operand == 4 {
        return register.a;
    } else if operand == 5 {
        return register.b;
    } else if operand == 6 {
        return register.c;
    } else {
        panic!();
    }
}

fn solve_day17_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day17.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let program: Vec<i64> = lines[4]
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    let mut reveres = program.clone();
    reveres.reverse();

    let mut possibles = vec![0, 1, 2, 3, 4, 5, 6, 7];
    for (index, &output) in reveres.iter().enumerate() {
        let nexts = solve(possibles, &program, output);
        if index == reveres.len() - 1 {
            return Ok(nexts[0]);
        }
        possibles = nexts
            .into_iter()
            .flat_map(|item| (0..=7).map(move |offset| item * 8 + offset))
            .collect();
    }
    panic!()
}

fn solve(possible_input: Vec<i64>, program: &Vec<i64>, target_value: i64) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for a in possible_input {
        let mut register = Register::new(a, 0, 0);
        let mut index = 0;
        while index < program.len() {
            let op_code = program[index];
            let operand = program[index + 1];
            match op_code {
                0 => {
                    adv(operand, &mut register);
                }
                1 => {
                    bxl(operand, &mut register);
                }
                2 => {
                    bst(operand, &mut register);
                }
                3 => {
                    break;
                }
                4 => {
                    bxc(&mut register);
                }
                5 => {
                    let resolved_operand = combo(operand, &register);
                    if resolved_operand % 8 == target_value {
                        result.push(a);
                    }
                }
                6 => {
                    bdv(operand, &mut register);
                }
                7 => {
                    cdv(operand, &mut register);
                }
                _ => {}
            }
            index = index + 2;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_1() {
        println!("result: {:?}", solve_day17());
    }

    #[test]
    fn test_day17_2() {
        println!("result: {:?}", solve_day17_2());
    }
}
