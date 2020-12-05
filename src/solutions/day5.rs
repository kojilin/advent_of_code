use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;

fn solve_day5() -> Result<i64, Box<dyn Error>> {
    let input = std::fs::read_to_string("src/solutions/day5.txt")?;
    let mut result = 0;
    for line in input.lines() {
        result = max(result, solution1(line));
    }
    Ok(result)
}

fn solve_day5_2() -> Result<i64, Box<dyn Error>> {
    let input = std::fs::read_to_string("src/solutions/day5.txt")?;
    let mut seats = HashSet::new();
    for i in 1..1027 {
        seats.insert(i);
    }
    for line in input.lines() {
        seats.remove(&solution1(line));
    }
    Ok(*seats.iter().next().unwrap())
}

fn solution1(seat: &str) -> i64 {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut col_min = 0;
    let mut col_max = 7;

    let chars: Vec<char> = seat.chars().collect();
    for i in 0..7usize {
        if chars[i] == 'F' {
            row_max = (row_min + row_max) / 2;
        } else {
            row_min = (row_min + row_max) / 2 + 1;
        }
    }
    for i in 7..10usize {
        if chars[i] == 'L' {
            col_max = (col_min + col_max) / 2;
        } else {
            col_min = (col_min + col_max) / 2 + 1;
        }
    }
    row_min * 8 + col_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solution1("FBFBBFFRLR"), 357);
        println!("-----real-----");
        println!("Result: {:?}", solve_day5());
    }

    // max seat 1027, min seat 0
    #[test]
    fn test2() {
        println!("-----real-----");
        println!("Result: {:?}", solve_day5_2());
    }
}
