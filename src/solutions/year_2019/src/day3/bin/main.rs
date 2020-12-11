use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use crate::Direction::{D, L, R, U};

#[derive(Debug)]
enum Direction {
    U,
    D,
    R,
    L,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    count: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day3/bin/day3.txt")?;
    let mut lines = input.lines();
    let moves1 = parseLine(lines.next().unwrap());

    let mut firstLinePoints: HashMap<(i32, i32), i32> = HashMap::new();
    let mut o = (0, 0);
    let mut steps = 0;
    for m in moves1 {
        let diff = match m.direction {
            U => (0, 1),
            D => (0, -1),
            R => (1, 0),
            L => (-1, 0),
        };
        for _ in 1..=m.count {
            steps += 1;
            o.0 = o.0 + diff.0;
            o.1 = o.1 + diff.1;
            if !firstLinePoints.contains_key(&o) {
                firstLinePoints.insert(o, steps);
            }
        }
    }
    let moves2 = parseLine(lines.next().unwrap());
    let mut o = (0, 0);
    let mut min = std::i32::MAX;
    let mut steps = 0;
    for m in moves2 {
        let diff = match m.direction {
            U => (0, 1),
            D => (0, -1),
            R => (1, 0),
            L => (-1, 0),
        };
        for _ in 1..=m.count {
            steps += 1;
            o.0 = o.0 + diff.0;
            o.1 = o.1 + diff.1;
            if firstLinePoints.contains_key(&o) {
                min = min.min(steps + *firstLinePoints.get(&o).unwrap());
            }
        }
    }
    println!("{}", min);
    Ok(())
}

fn parseLine(line: &str) -> Vec<Move> {
    let vec: Vec<&str> = line.split(",").collect();
    vec.iter().map(|x| {
        let x1 = x.chars().nth(0).unwrap();
        Move {
            direction: match x1 {
                'U' => U,
                'D' => D,
                'R' => R,
                'L' => L,
                _ => panic!("")
            },
            count: x.chars().skip(1).collect::<String>().parse::<i32>().unwrap(),
        }
    }).collect()
}
