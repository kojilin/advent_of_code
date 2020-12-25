use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn solve_day24() -> Result<(i64, i64), Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day24.txt")?;
    let mut blacks = HashSet::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        // x, y
        let mut point = (0, 0);
        let mut index = 0;
        while index < chars.len() {
            match chars[index] {
                'e' => {
                    point.0 += 1;
                    index += 1;
                }
                's' => {
                    match chars[index + 1] {
                        'e' => {
                            point.0 += 1;
                            point.1 += 1;
                        }
                        'w' => {
                            point.1 += 1;
                        }
                        _ => panic!(),
                    }
                    index += 2;
                }
                'w' => {
                    point.0 -= 1;
                    index += 1;
                }
                'n' => {
                    match chars[index + 1] {
                        'e' => {
                            point.1 -= 1;
                        }
                        'w' => {
                            point.0 -= 1;
                            point.1 -= 1;
                        }
                        _ => panic!(),
                    }
                    index += 2;
                }
                _ => panic!()
            }
        }
        if blacks.contains(&point) {
            blacks.remove(&point);
        } else {
            blacks.insert(point);
        }
    }
    let directions = vec![(1, 0), (1, 1), (0, 1), (0, -1), (-1, -1), (-1, 0)];
    let first_result = blacks.len();
    for _ in 0..100 {
        let mut map = HashMap::new();
        let mut next_blacks = HashSet::new();
        for &(bx, by) in &blacks {
            let mut nearby_count = 0;
            for &(x, y) in &directions {
                let target = (bx + x, by + y);
                if blacks.contains(&target) {
                    nearby_count += 1;
                } else {
                    let count = map.entry(target).or_insert(0);
                    *count += 1;
                }
            }
            if nearby_count == 1 || nearby_count == 2 {
                next_blacks.insert((bx, by));
            }
        }
        for (point, count) in map {
            if count == 2 {
                next_blacks.insert(point);
            }
        }
        blacks = next_blacks;
    }

    Ok((first_result as i64, blacks.len() as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {:?}", solve_day24()?);
        Ok(())
    }
}
