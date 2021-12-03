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
        let insert_index;
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

fn solve_day23_2() -> usize {
    let first = vec![3, 6, 8, 1, 9, 5, 7, 4, 2];
    //let first = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    // Preparing the next array.
    let mut next = vec![0; 1_000_001];
    for (index, &value) in first.iter().enumerate() {
        if index < first.len() - 1 {
            next[value] = first[index + 1];
        }
    }
    next[first[first.len() - 1]] = 10;

    for i in 10..1_000_000 {
        next[i] = i + 1;
    }
    next[1_000_000] = first[0];

    let mut current = 3;
    for _ in 0..10_000_000 {
        let mut picked = Vec::new();
        let mut next_value = current;
        for _ in 0..3 {
            next_value = next[next_value];
            picked.push(next_value);
        }
        let fourth = next[next_value];
        next[current] = fourth;

        let mut destination = current - 1;
        if destination == 0 {
            destination = 1_000_000;
        }
        while picked.contains(&destination) {
            destination -= 1;
            if destination == 0 {
                destination = 1_000_000;
            }
        }
        let tmp = next[destination];
        next[picked[2]] = tmp;
        next[destination] = picked[0];

        current = next[current];
    }

    next[1] * next[next[1]]
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
        println!("{}", solve_day23_2());
    }
}
