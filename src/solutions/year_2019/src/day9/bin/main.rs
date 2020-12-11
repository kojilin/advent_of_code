use std::error::Error;
use std::fs;

use _2019::int_code_computer2::IntCodeComputer2;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day9/bin/day9.txt")?;
    let codes: Vec<i64> = input.split(',')
        .map(|s: &str| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut computer = IntCodeComputer2::new(&codes);
    computer.compute(2);
    Ok(())
}

