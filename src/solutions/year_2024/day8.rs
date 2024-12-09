use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn solve_day8() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day8.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let mut antenas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                antenas
                    .entry(map[i][j])
                    .or_insert_with(Vec::new)
                    .push((i as i64, j as i64));
            }
        }
    }

    let result: HashSet<(i64, i64)> = antenas
        .iter()
        .flat_map(|(&c, positions)| {
            let mut targets = Vec::new();
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let diff1 = (
                        positions[i].0 - positions[j].0,
                        positions[i].1 - positions[j].1,
                    );

                    if positions[i].0 + diff1.0 >= 0
                        && positions[i].0 + diff1.0 < map[i].len() as i64
                        && positions[i].1 + diff1.1 >= 0
                        && positions[i].1 + diff1.1 < map.len() as i64
                    {
                        targets.push((positions[i].0 + diff1.0, positions[i].1 + diff1.1))
                    }

                    let diff2 = (
                        positions[j].0 - positions[i].0,
                        positions[j].1 - positions[i].1,
                    );

                    if positions[j].0 + diff2.0 >= 0
                        && positions[j].0 + diff2.0 < map[j].len() as i64
                        && positions[j].1 + diff2.1 >= 0
                        && positions[j].1 + diff2.1 < map.len() as i64
                    {
                        targets.push((positions[j].0 + diff2.0, positions[j].1 + diff2.1))
                    }
                }
            }
            targets
        })
        .collect();

    Ok(result.len() as i64)
}

fn solve_day8_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day8.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let mut antenas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                antenas
                    .entry(map[i][j])
                    .or_insert_with(Vec::new)
                    .push((i as i64, j as i64));
            }
        }
    }

    let mut result: HashSet<(i64, i64)> = antenas
        .iter()
        .flat_map(|(&c, positions)| {
            let mut targets = Vec::new();
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let diff1 = (
                        positions[i].0 - positions[j].0,
                        positions[i].1 - positions[j].1,
                    );

                    for count in 1..i64::MAX {
                        if positions[i].0 + diff1.0 * count >= 0
                            && positions[i].0 + diff1.0 * count < map[i].len() as i64
                            && positions[i].1 + diff1.1 * count >= 0
                            && positions[i].1 + diff1.1 * count < map.len() as i64
                        {
                            targets.push((
                                positions[i].0 + diff1.0 * count,
                                positions[i].1 + diff1.1 * count,
                            ))
                        } else {
                            break;
                        }
                    }
                    let diff2 = (
                        positions[j].0 - positions[i].0,
                        positions[j].1 - positions[i].1,
                    );
                    for count in 1..i64::MAX {
                        if positions[j].0 + diff2.0 * count >= 0
                            && positions[j].0 + diff2.0 * count < map[j].len() as i64
                            && positions[j].1 + diff2.1 * count >= 0
                            && positions[j].1 + diff2.1 * count < map.len() as i64
                        {
                            targets.push((
                                positions[j].0 + diff2.0 * count,
                                positions[j].1 + diff2.1 * count,
                            ))
                        } else {
                            break;
                        }
                    }
                }
            }
            targets
        })
        .collect();

    let flattened: HashSet<(i64, i64)> = antenas
        .into_iter()
        .flat_map(|(_, vec)| vec.into_iter())
        .collect();
    result.extend(flattened);

    // println!("{:?}", result);
    Ok(result.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_1() {
        println!("result: {:?}", solve_day8());
    }

    #[test]
    fn test_day8_2() {
        println!("result: {:?}", solve_day8_2());
    }
}
