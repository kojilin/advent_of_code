use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;

fn solve_day5() -> Result<i64, Box<dyn Error>> {
    let input = std::fs::read_to_string("src/solutions/year_2020/day5.txt")?;
    let mut result = 0;
    for line in input.lines() {
        result = max(result, solution1(line));
    }
    Ok(result)
}

fn solve_day5_2() -> Result<i64, Box<dyn Error>> {
    let input = std::fs::read_to_string("src/solutions/year_2020/day5.txt")?;
    let mut seats = HashSet::new();
    for i in 0..1028 {
        seats.insert(i);
    }
    for line in input.lines() {
        seats.remove(&solution1(line));
    }

    for &candidate in seats.iter() {
        if candidate == 0 || candidate == 1027 {
            continue;
        }
        if seats.contains(&(candidate + 1)) || seats.contains(&(candidate - 1)) {
            continue;
        }
        return Ok(candidate);
    }
    panic!("Can't find the seat")
}

fn solution1(seat: &str) -> i64 {
    unsafe {
        let row = find_value(0, 127, seat.get_unchecked(0..7), 'F');
        let col = find_value(0, 7, seat.get_unchecked(7..10), 'L');
        row * 8 + col
    }
}

fn find_value(min: i64, max: i64, input: &str, small_half: char) -> i64 {
    let mut min = min;
    let mut max = max;
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if chars[i] == small_half {
            max = (min + max) / 2;
        } else {
            min = (min + max) / 2 + 1;
        }
    }
    min
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solution1("FBFBBFFRLR"), 357);
        assert_eq!(solution1("BFFFBBFRRR"), 567);
        assert_eq!(solution1("FFFBBBFRRR"), 119);
        assert_eq!(solution1("BBFFBBFRLL"), 820);
        println!("-----real-----");
        println!("Result: {:?}", solve_day5());
    }

    #[test]
    fn test2() {
        println!("-----real-----");
        println!("Result: {:?}", solve_day5_2());
    }
}
