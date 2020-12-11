use std::error::Error;
use std::fs;

use _2019::int_code_computer_arcade::IntCodeComputerArcade;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day13/bin/day13.txt")?;
    let input_codes: Vec<i64> = input.trim().split(",").map(|c| { c.parse::<i64>().unwrap() }).collect();
    let result = IntCodeComputerArcade::new(&input_codes).compute();
    for i in 0..result.len() {
        if i % 3 == 2 && result[i - 2] == -1 && result[i - 1] == 0 {
            println!("{}", result[i]);
        }
    }
    Ok(())
}


