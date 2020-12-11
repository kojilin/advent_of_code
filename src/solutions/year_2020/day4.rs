use std::collections::HashMap;
use std::error::Error;

use regex::Regex;

fn solve_day4(rules: &Vec<Rule>) -> Result<i64, Box<dyn Error>> {
    let input = std::fs::read_to_string("src/solutions/year_2020/day4.txt")?;
    let mut passports: Vec<HashMap<String, String>> = vec![];

    let mut count = 0;
    let mut passport = HashMap::new();
    for line in input.lines() {
        let regex = Regex::new(r"([^ ]*):([^ ]*)").unwrap();
        for cap in regex.captures_iter(line) {
            passport.insert(cap[1].to_owned(), cap[2].to_owned());
        }
        if line.is_empty() {
            if is_valid(&passport, rules) {
                count += 1;
            }
            passports.push(passport);
            passport = HashMap::new();
        }
    }
    if is_valid(&passport, rules) {
        count += 1;
    }
    Ok(count)
}

struct Rule {
    key: String,
    prediction: Box<dyn Fn(&String) -> bool>,
}

fn is_valid(passport: &HashMap<String, String>, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if let Some(value) = passport.get(&rule.key) {
            if !(rule.prediction)(value) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result: {}", solve_day4(&vec![
            Rule {
                key: String::from("byr"),
                prediction: Box::new(|_| { true }),
            },
            Rule {
                key: String::from("iyr"),
                prediction: Box::new(|_| { true }),
            },
            Rule {
                key: String::from("eyr"),
                prediction: Box::new(|_| { true }),
            },
            Rule {
                key: String::from("hgt"),
                prediction: Box::new(|_| { true }),
            },
            Rule {
                key: String::from("hcl"),
                prediction: Box::new(|_| { true }),
            },
            Rule {
                key: String::from("ecl"),
                prediction: Box::new(|_| { true }),
            },
            Rule {
                key: String::from("pid"),
                prediction: Box::new(|_| { true }),
            }
        ])?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        let in_regex = Regex::new(r"^(\d{2})in$").unwrap();
        let cm_regex = Regex::new(r"^(\d{3})cm$").unwrap();
        let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let pid_regex = Regex::new(r"^\d{9}$").unwrap();
        println!("-----real-----");
        println!("result: {}", solve_day4(&vec![
            Rule {
                key: String::from("byr"),
                prediction: Box::new(|value| {
                    if let Ok(year) = value.parse::<i64>() {
                        year >= 1920 && year <= 2002
                    } else {
                        false
                    }
                }),
            },
            Rule {
                key: String::from("iyr"),
                prediction: Box::new(|value| {
                    if let Ok(year) = value.parse::<i64>() {
                        year >= 2010 && year <= 2020
                    } else {
                        false
                    }
                }),
            },
            Rule {
                key: String::from("eyr"),
                prediction: Box::new(|value| {
                    if let Ok(year) = value.parse::<i64>() {
                        year >= 2020 && year <= 2030
                    } else {
                        false
                    }
                }),
            },
            Rule {
                key: String::from("hgt"),
                prediction: Box::new(move |value| {
                    if let Some(capture) = in_regex.captures(value) {
                        let &in_value = &capture[1].parse::<i64>().unwrap();
                        return in_value >= 59 && in_value <= 76;
                    }
                    if let Some(capture) = cm_regex.captures(value) {
                        let &cm_value = &capture[1].parse::<i64>().unwrap();
                        return cm_value >= 150 && cm_value <= 193;
                    }
                    false
                }),
            },
            Rule {
                key: String::from("hcl"),
                prediction: Box::new(move |value| {
                    hcl_regex.is_match(value)
                }),
            },
            Rule {
                key: String::from("ecl"),
                prediction: Box::new(|value| {
                    value.eq("amb") || value.eq("blu")
                        || value.eq("brn") || value.eq("gry")
                        || value.eq("grn") || value.eq("hzl")
                        || value.eq("oth")
                }),
            },
            Rule {
                key: String::from("pid"),
                prediction: Box::new(move |value| {
                    pid_regex.is_match(value)
                }),
            }
        ])?);
        Ok(())
    }
}
