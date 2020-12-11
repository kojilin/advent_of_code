use std::error::Error;
use std::fs;

use _2019::int_code_computer::IntCodeComputer;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day5/bin/day5.txt")?;
    let codes: Vec<i32> = input.split(',')
        .map(|s: &str| s.trim().parse::<i32>().unwrap())
        .collect();

    let mut computer = IntCodeComputer::new(&codes);
    println!("{}", computer.compute(5));
    Ok(())
}

