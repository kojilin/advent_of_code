use std::collections::HashMap;
use std::error::Error;
use std::fs;

use regex::{Captures, Regex};

fn solve_day14() -> Result<i64, Box<dyn Error>> {
    let mask_regex = Regex::new(r"^mask = (.*)$")?;
    let mem_regex = Regex::new(r"^mem\[(\d+)] = (\d+)$")?;
    let input = fs::read_to_string("src/solutions/year_2020/day14.txt")?;
    let mut memory = HashMap::new();
    let mut mask = vec!['X'; 36];
    for line in input.lines() {
        if line.starts_with("mask") {
            let captures: Captures = mask_regex.captures_iter(line).next().unwrap();
            mask = captures[1].chars().collect();
        } else {
            let captures: Captures = mem_regex.captures_iter(line).next().unwrap();
            let address = captures[1].parse::<i64>()?;
            let value = captures[2].parse::<i64>()?;
            let current_value = memory.entry(address).or_insert(0);
            *current_value = mask_value(&mask, value);
        }
    }
    Ok(memory.values().sum())
}

fn mask_value(mask: &Vec<char>, value: i64) -> i64 {
    let value_binary: Vec<char> = format!("{:036b}", value).chars().collect();
    let mut result = vec![];
    for i in 0..36usize {
        result.push(if mask[i] == 'X' { value_binary[i] } else { mask[i] });
    }
    let result: String = result.into_iter().collect();
    i64::from_str_radix(&result, 2).unwrap()
}

fn solve_day14_2() -> Result<i64, Box<dyn Error>> {
    let mask_regex = Regex::new(r"^mask = (.*)$")?;
    let mem_regex = Regex::new(r"^mem\[(\d+)] = (\d+)$")?;
    let input = fs::read_to_string("src/solutions/year_2020/day14.txt")?;
    let mut memory = HashMap::new();
    let mut mask = vec!['X'; 36];
    for line in input.lines() {
        if line.starts_with("mask") {
            let captures: Captures = mask_regex.captures_iter(line).next().unwrap();
            mask = captures[1].chars().collect();
        } else {
            let captures: Captures = mem_regex.captures_iter(line).next().unwrap();
            let address = captures[1].parse::<i64>()?;
            let value = captures[2].parse::<i64>()?;
            for address in mask_value_2(&mask, address) {
                let current_value = memory.entry(address).or_insert(0);
                *current_value = value;
            }
        }
    }
    Ok(memory.values().sum())
}

// return list of address
fn mask_value_2(mask: &Vec<char>, address: i64) -> Vec<i64> {
    let mut address_binary: Vec<char> = format!("{:036b}", address).chars().collect();
    let mut indexes = vec![];
    for i in 0..36usize {
        address_binary[i] =
            if mask[i] == 'X' {
                indexes.push(i);
                'X'
            } else if mask[i] == '1' {
                mask[i]
            } else {
                address_binary[i]
            };
    }

    let mut all_addresses = vec![address_binary];
    for index in indexes {
        let mut tmp = vec![];
        for address in all_addresses {
            let mut v1 = address.clone();
            v1[index] = '1';
            tmp.push(v1);
            let mut v0 = address.clone();
            v0[index] = '0';
            tmp.push(v0);
        }
        all_addresses = tmp;
    }
    all_addresses.into_iter()
        .map(|cs| cs.into_iter().collect::<String>())
        .map(|str| i64::from_str_radix(&str, 2).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day14()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day14_2()?);
        Ok(())
    }
}
