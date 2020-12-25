use std::cmp::max;

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

fn solve_day23_2(input: i64) -> Vec<u32> {
    let max_value = 1_000_000u32;
    let mut cups = vec![1; max_value as usize];
    let mut first_cups: Vec<u32> =
        input.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
    for i in 1..=cups.len() {
        if i <= first_cups.len() {
            cups[i - 1] = first_cups[i - 1];
        } else {
            cups[i - 1] = i as u32;
        }
    }

    let mut current = 0;
    for step in 0..10_000_000 {
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
        // println!("current: {} at {}, picked: {:?}", current_cup, tmp, picked);
        // current is wrong now.
        let mut target = current_cup - 1;
        if target == 0 {
            target = max_value;
        }
        let mut insert_index = 0;
        loop {
            if let Some(index) = cups.iter().position(|v| v.eq(&target)) {
                insert_index = index;
                break;
            }
            target -= 1;
            if target == 0 {
                target = max_value;
            }
        }
        for (offset, &p) in picked.iter().enumerate() {
            cups.insert(insert_index + 1 + offset, p);
        }
        // just have 3 offset change
        let mut possible_current = 0;
        let mut found = false;
        for index in current..current + 4 {
            if cups[index % max_value as usize] == current_cup {
                possible_current = index;
                found = true;
            }
        }
        for index in max(0, current as i64 - 4) as usize..current {
            if cups[(index + max_value as usize) % max_value as usize] == current_cup {
                possible_current = index;
                found = true;
            }
        }
        if !found {
            panic!("{} at {}, find {}", current, step, current_cup);
        }
        current = possible_current;
        current += 1;
        current %= cups.len();
    }
    let found = cups.iter().position(|v| v.eq(&1)).unwrap();

    vec![cups[(found + 1) % 1_000_000], cups[(found + 2) % 1_000_000]]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("Result1: {:?}", solve_day23(368195742));
    }

    #[test]
    fn test2() {
        println!("Result2: {:?}", solve_day23_2(368195742));
    }
}
