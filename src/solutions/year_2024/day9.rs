use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day9() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day9.txt")?;
    let mut block = true;
    let mut input_blocks: Vec<String> = Vec::new();
    let mut index = 0;
    for c in input.chars() {
        let count = c.to_digit(10).unwrap();
        if block {
            for _ in 0..count {
                input_blocks.push(index.to_string());
            }
            index += 1;
        } else {
            for _ in 0..count {
                input_blocks.push(".".to_owned());
            }
        }
        block = !block;
    }

    let mut result: Vec<&str> = Vec::new();
    let mut left_index = 0;
    let mut right_index = input_blocks.len() - 1;

    while left_index <= right_index {
        if input_blocks[left_index] != "." {
            result.push(&input_blocks[left_index]);
            left_index += 1;
        } else {
            if input_blocks[right_index] != "." {
                result.push(&input_blocks[right_index]);
                left_index += 1;
            }
            right_index -= 1;
        }
    }

    let mut checksum = 0;
    for i in 0..result.len() {
        checksum += result[i].parse::<i64>().unwrap() * i as i64;
    }
    Ok(checksum)
}

fn solve_day9_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day9.txt")?;
    let mut block = true;
    // index & how many items

    let mut empty_blocks: HashMap<usize, Vec<usize>> = HashMap::new();
    // (index, count, label)
    let mut used_blocks: Vec<(usize, usize, usize)> = Vec::new();
    let mut input_blocks: Vec<String> = Vec::new();
    let mut label = 0;
    let mut current_index = 0;
    for (block_index, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;
        if block {
            used_blocks.push((current_index, count, label));
            for _ in 0..count {
                input_blocks.push(label.to_string());
                current_index += 1;
            }
            label += 1;
        } else {
            empty_blocks
                .entry(count)
                .or_insert(Vec::new())
                .push(current_index);

            for _ in 0..count {
                input_blocks.push(".".to_owned());
                current_index += 1;
            }
        }
        block = !block;
    }

    for &(index, count, label) in used_blocks.iter().rev() {
        // from right to left
        match find_smallest_index(count, &mut empty_blocks, index) {
            None => {}
            Some((target_count, target_index)) => {
                for i in target_index..target_index + count {
                    input_blocks[i] = label.to_string();
                }

                for i in index..index + count {
                    input_blocks[i] = ".".to_owned();
                }
            }
        }
    }

    // println!("{:?}", input_blocks);
    let mut checksum = 0;
    for i in 0..input_blocks.len() {
        if input_blocks[i] != "." {
            checksum += input_blocks[i].parse::<i64>().unwrap() * i as i64;
        }
    }
    Ok(checksum)
}

// return target (count, index) to put
fn find_smallest_index(
    count: usize,
    empty_blocks: &mut HashMap<usize, Vec<usize>>,
    last_index: usize,
) -> Option<(usize, usize)> {
    // (count, index)
    let mut smallest_pair: Option<(usize, usize)> = Option::None;
    for i in count..=9 {
        match empty_blocks.get(&i) {
            None => {}
            Some(possible_indexes) => {
                if possible_indexes.is_empty() {
                    continue;
                }
                let index = possible_indexes[0];
                if index > last_index {
                    continue;
                }
                if smallest_pair == None || index < smallest_pair.unwrap().1 {
                    smallest_pair = Some((i, index));
                }
            }
        }
    }
    // println!("{:?} for {} i {:?}", smallest_pair, count, empty_blocks);
    if smallest_pair == None {
        return None;
    }
    empty_blocks.get_mut(&smallest_pair?.0)?.remove(0);
    //may return some empty block
    let remain = smallest_pair?.0 - count;
    if remain > 0 {
        let new_index = smallest_pair?.1 + count;
        let new_indexes = empty_blocks.entry(remain).or_insert(Vec::new());
        match new_indexes.binary_search(&new_index) {
            Ok(_) => {
                // should not happen
            }
            Err(v) => {
                new_indexes.insert(v, new_index);
            }
        }
    }
    smallest_pair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_1() {
        println!("result: {:?}", solve_day9());
    }

    #[test]
    fn test_day9_2() {
        println!("result: {:?}", solve_day9_2());
    }
}
