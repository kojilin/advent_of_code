use std::collections::HashMap;

fn solve_day23(input: i64) -> Vec<u32> {
    let mut cups: Vec<u32> = input.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut current = 0;
    for _ in 0..100 {
        let current_cup = cups[current];
        let mut picked = vec![];
        for _ in 0..3 {
            // go to header, just remove first 1;
            if current + 1 >= cups.len() {
                picked.push(cups.remove(0));
            } else {
                picked.push(cups.remove(current + 1));
            }
        }
        // current is wrong now.
        let mut target = current_cup - 1;
        if target == 0 {
            target = 9;
        }
        let mut insert_index = 0;
        loop {
            if let Some(index) = cups.iter().position(|v| v.eq(&target)) {
                insert_index = index;
                break;
            }
            target -= 1;
            if target == 0 {
                target = 9;
            }
        }
        for (offset, &p) in picked.iter().enumerate() {
            cups.insert(insert_index + 1 + offset, p);
        }
        current = cups.iter().position(|v| v.eq(&current_cup)).unwrap();
        current += 1;
        current %= cups.len();
    }
    cups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("Result1: {:?}", solve_day23(368195742));
    }
}
