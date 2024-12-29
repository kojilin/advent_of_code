use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::{Mul, Sub};

fn solve_day24() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day24.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut iter1 = lines.iter();
    let iter = iter1.by_ref();
    let mut gates = HashMap::new();
    for &line in iter.take_while(|line| !line.is_empty()) {
        let raw: Vec<&str> = line.split(": ").collect();
        let gate_name = raw[0];
        let input = raw[1].parse::<i32>().unwrap();
        gates.insert(gate_name, input);
    }

    let mut froms = HashMap::new();
    for &line in iter {
        let raw: Vec<&str> = line.split(" ").collect();
        let op1 = raw[0];
        let op = raw[1];
        let op2 = raw[2];
        let output = raw[4];
        froms.insert(output, (op1, op, op2));
    }

    for (key, _) in &froms {
        solve(key, &mut gates, &froms);
    }

    let mut result: Vec<&str> = gates
        .iter()
        .filter(|(x, _)| x.starts_with("z"))
        .map(|(&x1, _)| x1)
        .collect();
    result.sort();
    result.reverse();
    let mut number_str = String::new();
    for str in result {
        number_str += &gates.get(str).unwrap().to_string();
    }
    Ok(i64::from_str_radix(&number_str, 2).unwrap())
}

fn solve<'a>(
    key: &'a str,
    gates: &mut HashMap<&'a str, i32>,
    froms: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) {
    if gates.contains_key(key) {
        return;
    }
    let operations = froms.get(key).unwrap();
    if !gates.contains_key(operations.0) {
        solve(operations.0, gates, froms);
    }
    let &left = gates.get(operations.0).unwrap();

    if !gates.contains_key(operations.2) {
        solve(operations.2, gates, froms);
    }
    let &right = gates.get(operations.2).unwrap();

    let r = match operations.1 {
        "XOR" => left ^ right,
        "AND" => left & right,
        "OR" => left | right,
        _ => {
            panic!()
        }
    };
    gates.insert(key, r);
}

fn solve_day24_2() -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day24.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut iter1 = lines.iter();
    let iter = iter1.by_ref();
    let mut gates = HashMap::new();
    for &line in iter.take_while(|line| !line.is_empty()) {
        let raw: Vec<&str> = line.split(": ").collect();
        let gate_name = raw[0];
        let input = raw[1].parse::<i32>().unwrap();
        gates.insert(gate_name, input);
    }

    let mut froms = HashMap::new();
    for &line in iter {
        let raw: Vec<&str> = line.split(" ").collect();
        let op1 = raw[0];
        let op = raw[1];
        let op2 = raw[2];
        let output = raw[4];
        froms.insert(output, (op1, op, op2));
    }
    let mut result = try_fix(&mut froms, &gates, 0);
    result.sort();
    Ok(result.join(","))
}

fn try_fix<'a>(
    froms: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    gates: &HashMap<&'a str, i32>,
    stack: i32,
) -> Vec<String> {
    sort_inputs(froms);

    let mut names = None;

    'outer: for &name in froms.keys() {
        if !name.starts_with("z") {
            continue;
        }
        let actual = build_terms(name, froms, gates);
        let expected = expected_term(name[1..].parse::<i32>().unwrap(), 45);
        if let Some(diff_result) = diff(&actual, &expected) {
            println!("{:?} <> {:?}", diff_result.0, diff_result.1);
            for &name in froms.keys() {
                let input = build_terms(name, froms, gates);
                if let None = diff(diff_result.0, &input) {
                    // found target
                    println!(
                        "swap {} and {} {:?} <> {:?}",
                        diff_result.1, input.name, diff_result.0, input
                    );

                    names = Some((diff_result.1.to_string(), input.name));
                    break 'outer;
                }
            }
        }
    }

    let mut result = Vec::new();
    if let Some((n1, n2)) = names {
        // println!("c1: {} > {:?}", n1, froms.get(n1.as_str()));
        // println!("c2: {} > {:?}", n2, froms.get(n2.as_str()));
        if let (Some(&input_1), Some(&input_2)) = (froms.get(n1.as_str()), froms.get(n2.as_str())) {
            // Perform the swap
            *froms.get_mut(n1.as_str()).unwrap() = input_2;
            *froms.get_mut(n2.as_str()).unwrap() = input_1;
        }
        result.push(n1);
        result.push(n2);
        // println!("c3: {} > {:?}", n1, froms.get(n1.as_str()));
        // println!("c4: {} > {:?}", n2, froms.get(n2.as_str()));
        result.append(&mut try_fix(froms, gates, stack + 1));
    }
    result
}

fn diff<'a>(actual: &'a Node, expected: &'a Node) -> Option<(&'a Node, &'a str)> {
    if actual.op != expected.op {
        return Some((expected, &actual.name));
    }
    if expected.op == "ID" {
        // Wrong input gate of x or y
        if expected.name != actual.name {
            return Some((expected, &actual.name));
        }
    } else {
        if let Some(e_left) = &expected.left {
            if let Some(a_left) = &actual.left {
                if let Some(result) = diff(a_left, e_left) {
                    return Some(result);
                }
            }
        }
        if let Some(e_right) = &expected.right {
            if let Some(a_right) = &actual.right {
                if let Some(result) = diff(a_right, e_right) {
                    return Some(result);
                }
            }
        }
    }
    None
}

fn build_terms<'a>(
    name: &'a str,
    froms: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    gates: &HashMap<&'a str, i32>,
) -> Node {
    if gates.contains_key(name) {
        return Node::new(name.to_string(), "ID".to_string(), None, None);
    }
    if froms.contains_key(name) {
        let (left, op, right) = froms.get(name).unwrap();
        return Node::new(
            name.to_string(),
            op.to_string(),
            Some(Box::new(build_terms(left, froms, gates))),
            Some(Box::new(build_terms(right, froms, gates))),
        );
    }
    panic!()
}

fn score<'a>(name: &'a str, froms: &HashMap<&'a str, (&'a str, &'a str, &'a str)>) -> i32 {
    if name.starts_with('y') {
        return name[1..name.len()].parse().unwrap();
    }
    // let x, y ordered
    if name.starts_with('x') {
        return i32::MAX;
    }

    let input = froms.get(name).unwrap();
    min(score(input.0, froms), score(input.2, froms))
}

fn sort_inputs<'a>(froms: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>) {
    let keys: Vec<&'a str> = froms.keys().cloned().collect();

    for key in keys {
        if let Some(&(left, middle, right)) = froms.get(key) {
            if score(left, &froms) <= score(right, &froms) {
                // Swap left and right
                froms.insert(key, (right, middle, left));
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    op: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(
        name: String,
        op: String,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    ) -> Self {
        Self {
            name,
            op,
            left,
            right,
        }
    }
}

// No need name for middle gate, we don't compare name directly
fn expected_term(bit: i32, max_bit: i32) -> Node {
    if bit == 0 {
        Node::new(
            "".to_string(),
            "XOR".to_string(),
            Some(Box::new(Node::new(
                format!("x{:02}", bit),
                "ID".to_string(),
                None,
                None,
            ))),
            Some(Box::new(Node::new(
                format!("y{:02}", bit),
                "ID".to_string(),
                None,
                None,
            ))),
        )
    } else if bit < max_bit {
        Node::new(
            "".to_string(),
            "XOR".to_string(),
            Some(Box::new(Node::new(
                // not used
                "".to_string(),
                "XOR".to_string(),
                Some(Box::new(Node::new(
                    format!("x{:02}", bit),
                    "ID".to_string(),
                    None,
                    None,
                ))),
                Some(Box::new(Node::new(
                    format!("y{:02}", bit),
                    "ID".to_string(),
                    None,
                    None,
                ))),
            ))),
            Some(Box::new(expected_term_from_lower_bit(bit - 1))),
        )
    } else {
        expected_term_from_lower_bit(bit - 1)
    }
}

// No need name for middle gate, we don't compare name directly
fn expected_term_from_lower_bit(bit: i32) -> Node {
    if bit == 0 {
        Node::new(
            "".to_string(),
            "AND".to_string(),
            Some(Box::new(Node::new(
                format!("x{:02}", bit),
                "ID".to_string(),
                None,
                None,
            ))),
            Some(Box::new(Node::new(
                format!("y{:02}", bit),
                "ID".to_string(),
                None,
                None,
            ))),
        )
    } else {
        Node::new(
            "".to_string(),
            "OR".to_string(),
            Some(Box::new(Node::new(
                "".to_string(),
                "AND".to_string(),
                Some(Box::new(Node::new(
                    format!("x{:02}", bit),
                    "ID".to_string(),
                    None,
                    None,
                ))),
                Some(Box::new(Node::new(
                    format!("y{:02}", bit),
                    "ID".to_string(),
                    None,
                    None,
                ))),
            ))),
            Some(Box::new(Node::new(
                "".to_string(),
                "AND".to_string(),
                Some(Box::new(Node::new(
                    "".to_string(),
                    "XOR".to_string(),
                    Some(Box::new(Node::new(
                        format!("x{:02}", bit),
                        "ID".to_string(),
                        None,
                        None,
                    ))),
                    Some(Box::new(Node::new(
                        format!("y{:02}", bit),
                        "ID".to_string(),
                        None,
                        None,
                    ))),
                ))),
                Some(Box::new(expected_term_from_lower_bit(bit - 1))),
            ))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day24()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day24_2()?);
        Ok(())
    }
}
