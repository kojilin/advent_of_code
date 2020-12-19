use std::collections::HashMap;
use std::error::Error;
use std::fs;

use regex::Regex;

fn solve_day19() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day19.txt")?;
    let mut end_rules = HashMap::new();
    let mut rules = HashMap::new();
    let mut inputs = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            let line: Vec<&str> = line.split(":").collect();
            let rule_id = line[0].parse::<i64>()?;
            //rule, only a&b
            if line[1].contains("a") {
                // end rule
                end_rules.insert(rule_id, "a");
            }
            if line[1].contains("b") {
                // end rule
                end_rules.insert(rule_id, "b");
            } else if line[1].contains("|") {
                // pipe
                let content: Vec<&str> = line[1].split("|").collect();
                let first: Vec<&str> = content[0].split_whitespace().map(&str::trim).collect();
                let second: Vec<&str> = content[1].split_whitespace().map(&str::trim).collect();
                rules.insert(rule_id, vec![first, second]);
            } else {
                let rule: Vec<&str> = line[1].split_whitespace().map(&str::trim).collect();
                rules.insert(rule_id, vec![rule]);
            }
        } else {
            //input
            inputs.push(line);
        }
    }
    let mut dp = HashMap::new();

    let mut pattern: String = make_rule_pattern(0, &rules, &end_rules, &mut dp);
    pattern.insert(0, '^');
    pattern.push('$');
    let regex = Regex::new(&pattern)?;
    let mut sum = 0;
    for input in inputs {
        if regex.is_match(input) {
            sum += 1;
        }
    }
    Ok(sum)
}

fn make_rule_pattern(id: i64, rules: &HashMap<i64, Vec<Vec<&str>>>, end_rules: &HashMap<i64, &str>, dp: &mut HashMap<i64, String>) -> String {
    if dp.contains_key(&id) {
        return dp.get(&id).unwrap().clone();
    }
    if end_rules.contains_key(&id) {
        let pattern = end_rules.get(&id).unwrap().to_owned().to_owned();
        dp.insert(id, pattern.clone());
        return pattern;
    }
    let each_rule = rules.get(&id).unwrap();
    if each_rule.len() == 2 {
        let mut result = String::new();
        result.push('(');
        for (i, rule) in each_rule.iter().enumerate() {
            if i != 0 {
                result.push('|');
            }
            result.push_str(&build_sub_rule(&rules, &end_rules, dp, rule));
        }
        result.push(')');
        dp.insert(id, result.clone());
        return result;
    } else {
        let rule = &each_rule[0];
        let result = build_sub_rule(&rules, &end_rules, dp, rule);
        dp.insert(id, result.clone());
        return result;
    }
}

fn build_sub_rule(rules: &&HashMap<i64, Vec<Vec<&str>>>, end_rules: &&HashMap<i64, &str>, dp: &mut HashMap<i64, String>, rule: &Vec<&str>) -> String {
    let mut result = String::new();
    for &sub_id in rule {
        result.push_str(&make_rule_pattern(sub_id.parse::<i64>().unwrap(), rules, end_rules, dp));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day19()?);
        Ok(())
    }
}
