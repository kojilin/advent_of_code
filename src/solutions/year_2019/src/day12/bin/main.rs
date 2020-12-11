use std::error::Error;
use std::fs;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day12/bin/day12.txt")?;
    let mut points: Vec<Vec<i32>> = input.lines().map(|line| {
        let re = Regex::new(r"^<x=(-?\d*), y=(-?\d*), z=(-?\d*)>$").unwrap();
        let caps = re.captures(line).unwrap();
        let x = caps.get(1).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();
        let y = caps.get(2).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();
        let z = caps.get(3).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();

        vec![x, y, z]
    }).collect();

    let mut speed = vec![vec![0; 3]; points.len()];
    let snapshot =
        vec![points.get(0).unwrap().clone(),
             points.get(1).unwrap().clone(),
             points.get(2).unwrap().clone(),
             points.get(3).unwrap().clone()];

    let mut back_or_not = [-1, -1, -1];
    let mut step: i64 = 0;
    while back_or_not[0] == -1 || back_or_not[1] == -1 || back_or_not[2] == -1 {
        step += 1;
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let p1 = &points[i];
                let p2 = &points[j];
                for k in 0..3 {
                    if p1[k] < p2[k] {
                        speed[i][k] += 1;
                        speed[j][k] -= 1;
                    } else if p1[k] > p2[k] {
                        speed[i][k] -= 1;
                        speed[j][k] += 1;
                    }
                }
            }
        }

        for i in 0..points.len() {
            points[i][0] += speed[i][0];
            points[i][1] += speed[i][1];
            points[i][2] += speed[i][2];
        }

        for i in 0..3 {
            if points[0][i] == snapshot[0][i]
                && points[1][i] == snapshot[1][i]
                && points[2][i] == snapshot[2][i]
                && points[3][i] == snapshot[3][i]
                && speed[0][i] == 0
                && speed[1][i] == 0
                && speed[2][i] == 0
                && speed[3][i] == 0
                && back_or_not[i] == -1 {
                back_or_not[i] = step;
            }
        }
    }
    let mut gcd1 = gcd(back_or_not[0], back_or_not[1]);
    let mut result = (back_or_not[0] / gcd1) * (back_or_not[1] / gcd1) * gcd1;
    gcd1 = gcd(result, back_or_not[2]);
    result = (result / gcd1) * (back_or_not[2] / gcd1) * gcd1;
    println!("{}", result);
    Ok(())
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    if a < b {
        return gcd(b, a);
    }
    return gcd(b, a % b);
}

