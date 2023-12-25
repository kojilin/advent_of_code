use std::error::Error;
use std::fs;

fn solve_day11() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day11.txt")?;
    let mut lines = content.lines();

    //scanned row maps
    let mut tmp_map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if line.contains('#') {
            tmp_map.push(line.to_owned().chars().collect::<Vec<char>>());
        } else {
            tmp_map.push(line.to_owned().chars().collect::<Vec<char>>());
            tmp_map.push(line.to_owned().chars().collect::<Vec<char>>());
        }
    }

    // col scan and expand
    let cols = tmp_map[0].len();
    let rows = tmp_map.len();
    let mut expanded_map: Vec<Vec<char>> = vec![Vec::new(); rows];

    for col_index in 0..cols {
        let mut has_sharp = false;
        for row_index in 0..rows {
            expanded_map[row_index].push(tmp_map[row_index][col_index]);
            if tmp_map[row_index][col_index] == '#' {
                has_sharp = true;
            }
        }

        if !has_sharp {
            for i in 0..rows {
                let c = *expanded_map[i].last().unwrap();
                // clone last cols
                expanded_map[i].push(c);
            }
        }
    }

    let rows = expanded_map.len();

    let mut pairs = Vec::new();
    for i in 0..rows {
        for j in 0..expanded_map[i].len() {
            if expanded_map[i][j] == '#' {
                pairs.push((i as i64, j as i64));
            }
        }
    }

    let mut result = 0;
    for i in 0..pairs.len() {
        for j in i..pairs.len() {
            result += (pairs[i].0 - pairs[j].0).abs() + (pairs[i].1 - pairs[j].1).abs();
        }
    }
    Ok(result)
}

fn solve_day11_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day11.txt")?;
    let mut lines = content.lines();
    let mut input_map: Vec<Vec<char>> = Vec::new();
    let mut expand_rows = Vec::new();

    for (row_index, line) in lines.enumerate() {
        input_map.push(line.chars().collect());
        if !line.contains('#') {
            expand_rows.push(row_index);
        }
    }

    let mut expand_cols = Vec::new();
    for col in 0..input_map[0].len() {
        let mut has_star = false;
        for row in 0..input_map.len() {
            if input_map[row][col] == '#' {
                has_star = true;
                break;
            }
        }
        if !has_star {
            expand_cols.push(col);
        }
    }
    let mut stars = Vec::new();
    for row in 0..input_map.len() {
        for col in 0..input_map[row].len() {
            if input_map[row][col] == '#' {
                stars.push((row as i64, col as i64));
            }
        }
    }

    let mut result = 0;
    for i in 0..stars.len() {
        let from = stars[i];
        for j in i + 1..stars.len() {
            let to = stars[j];
            let diff_row = (from.0 - to.0).abs();
            let r1 = expand_rows.binary_search(&(from.0 as usize)).err().unwrap() as i64;
            let r2 = expand_rows.binary_search(&(to.0 as usize)).err().unwrap() as i64;
            let mut diff_count = 0;
            if r1 == r2 {
                // no gap
                diff_count += diff_row;
            } else {
                let expand_count = ((r2 - r1).abs() );
                diff_count += expand_count * 1000000;
                diff_count += diff_row - expand_count;
            }
            // println!("{:?} <> {:?} row diff>>>>{}", from, to, diff_count);
            result += diff_count;

            diff_count = 0;
            let diff_col = (from.1 - to.1).abs();
            let c1 = expand_cols.binary_search(&(from.1 as usize)).err().unwrap() as i64;
            let c2 = expand_cols.binary_search(&(to.1 as usize)).err().unwrap() as i64;
            // println!("{:?}, {:?}", expand_cols, from);
            // println!("{}, {}", c1, c2);
            if c1 == c2 {
                // no gap
                diff_count += diff_col;
            } else {
                let expand_count = ((c2 - c1).abs() );
                diff_count += expand_count * 1000000;
                diff_count += diff_col - expand_count;
            }
            // println!("{:?} <> {:?} col diff>>>>{}", from, to, diff_count);
            result += diff_count;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day11()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day11_2()?);
        Ok(())
    }
}
