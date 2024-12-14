use regex::Regex;
use std::error::Error;
use std::fs;

fn solve_day14() -> Result<i64, Box<dyn Error>> {
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    let mut map = [[0; WIDTH as usize]; HEIGHT as usize];

    let input = fs::read_to_string("src/solutions/year_2024/day14.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"-?\d+").unwrap();
    for line in lines {
        let numbers: Vec<i64> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        let pos = (numbers[0], numbers[1]);
        let v = (numbers[2], numbers[3]);

        let tx = v.0 * 100 + pos.0;
        let ty = v.1 * 100 + pos.1;

        let final_x = ((tx % WIDTH) + WIDTH) % WIDTH;
        let final_y = ((ty % HEIGHT) + HEIGHT) % HEIGHT;
        map[final_y as usize][final_x as usize] += 1;
    }

    let mut result: [i64; 4] = [0; 4];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                continue;
            }

            if i < (HEIGHT / 2) as usize && j < (WIDTH / 2) as usize {
                result[0] += map[i][j];
            } else if i < (HEIGHT / 2) as usize && j > (WIDTH / 2) as usize {
                result[1] += map[i][j];
            } else if i > (HEIGHT / 2) as usize && j < (WIDTH / 2) as usize {
                result[2] += map[i][j];
            } else if i > (HEIGHT / 2) as usize && j > (WIDTH / 2) as usize {
                result[3] += map[i][j];
            }
        }
    }

    Ok(result[0] * result[1] * result[2] * result[3])
}

fn solve_day14_2() -> Result<i64, Box<dyn Error>> {
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;

    let input = fs::read_to_string("src/solutions/year_2024/day14.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"-?\d+").unwrap();

    let mut points = vec![];

    for line in lines {
        let numbers: Vec<i64> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        points.push((numbers[0], numbers[1], numbers[2], numbers[3]));
    }

    let mut count = 0;
    for _ in 0..100000 {
        let mut map = [[0; WIDTH as usize]; HEIGHT as usize];
        for point in &points {
            let tx = point.2 * count + point.0;
            let ty = point.3 * count + point.1;

            let final_x = ((tx % WIDTH) + WIDTH) % WIDTH;
            let final_y = ((ty % HEIGHT) + HEIGHT) % HEIGHT;
            map[final_y as usize][final_x as usize] = 1;
        }

        let mut found = false;
        for i in 0..map.len() {
            let line: String = map[i].iter().map(|x| x.to_string()).collect();
            if line.contains("1111111111111111111") {
                // found?
                found = true;
                break;
            }
        }

        if found {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == 0 {
                        print!(".");
                    } else {
                        print!("{}", map[i][j]);
                    }
                }
                println!();
            }
            break;
        }
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_1() {
        println!("result: {:?}", solve_day14());
    }

    #[test]
    fn test_day14_2() {
        println!("result: {:?}", solve_day14_2());
    }
}
