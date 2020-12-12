use std::error::Error;
use std::fs;

fn solve_day12() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day12.txt")?;
    let mut x = 0i64;
    let mut y = 0i64;
    // (1, 0), (0, -1), (-1, 0), (0, 1)
    let mut current_direction = 0;
    for line in input.lines() {
        let action = &line[0..1];
        let value = &line[1..].parse::<i64>()?;
        match action {
            "N" => {
                y += value;
            }
            "S" => {
                y -= value;
            }
            "E" => {
                x += value;
            }
            "W" => {
                x -= value;
            }
            "L" => {
                match value {
                    90 => {
                        current_direction -= 1;
                    }
                    180 => {
                        current_direction -= 2;
                    }
                    270 => {
                        current_direction -= 3;
                    }
                    360 => {
                        current_direction -= 4;
                    }
                    _ => {
                        panic!("wrong degree");
                    }
                }
                current_direction = (current_direction + 4) % 4;
            }
            "R" => {
                match value {
                    90 => {
                        current_direction += 1;
                    }
                    180 => {
                        current_direction += 2;
                    }
                    270 => {
                        current_direction += 3;
                    }
                    360 => {
                        current_direction += 4;
                    }
                    _ => {
                        panic!("wrong degree.");
                    }
                }
                current_direction %= 4;
            }
            "F" => {
                match current_direction {
                    0 => {
                        x += value;
                    }
                    1 => {
                        y -= value;
                    }
                    2 => {
                        x -= value;
                    }
                    3 => {
                        y += value;
                    }
                    _ => {
                        panic!("wrong direction.");
                    }
                }
            }
            _ => panic!("Wrong input.")
        }
    }
    Ok(x.abs() + y.abs())
}


fn solve_day12_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day12.txt")?;
    let mut x = 0i64;
    let mut y = 0i64;
    let mut way_x = 10i64;
    let mut way_y = 1i64;
    for line in input.lines() {
        let action = &line[0..1];
        let value = &line[1..].parse::<i64>()?;
        match action {
            "N" => {
                way_y += value;
            }
            "S" => {
                way_y -= value;
            }
            "E" => {
                way_x += value;
            }
            "W" => {
                way_x -= value;
            }
            "L" => {
                let count = value / 90;
                for _ in 0..count {
                    let tmp = -way_y;
                    way_y = way_x;
                    way_x = tmp;
                }
            }
            "R" => {
                let count = value / 90;
                for _ in 0..count {
                    let tmp = way_y;
                    way_y = -way_x;
                    way_x = tmp;
                }
            }
            "F" => {
                x += way_x * value;
                y += way_y * value;
            }
            _ => panic!("Wrong input.")
        }
    }
    Ok(x.abs() + y.abs())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day12()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day12_2()?);
        Ok(())
    }
}


