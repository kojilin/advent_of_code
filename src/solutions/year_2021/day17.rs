use std::cmp::max;
use std::error::Error;

fn solve_day17_1() -> i32 {
    return solve_1(14, 50, -267, -225);
}

fn solve_1(_x1: i32, _x2: i32, y1: i32, _y2: i32) -> i32 {
    let speed = -y1 - 1;
    return (speed + 1) * speed / 2;
}

fn solve_day17_2() -> i32 {
    return solve_2(14, 50, -267, -225);
}

fn solve_2(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let mut pair = 0;
    for y in y1..-y1 {
        for x in 0..=x2 {
            let mut y_pos = 0;
            let mut x_pos = 0;
            let mut second = 0;
            let mut found = false;
            loop {
                y_pos += y - second;
                x_pos += max(0, x - second);

                if y_pos < y1 {
                    break;
                }
                if x_pos > x2 {
                    break;
                }

                if x_pos >= x1 && x_pos <= x2 && y_pos >= y1 && y_pos <= y2 {
                    found = true;
                    break;
                }

                second += 1;
            }
            if found {
                pair += 1;
            }
        }
    }
    return pair;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day17_1());
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day17_2());
        Ok(())
    }
}

