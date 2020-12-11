use std::error::Error;
use std::fs;
use std::io::{self, Read};

use _2019::int_code_computer_25::IntCodeComputer25;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day25/bin/day25.txt")?;
    let mut codes: Vec<i64> = input.trim().split(",").map(|c| { c.parse::<i64>().unwrap() }).collect();
    let mut computer = IntCodeComputer25::new(&codes);
    let mut command: Vec<i64> = vec![];
    loop {
        let vec = computer.compute(&mut command);
        for x in vec {
            print!("{}", x as u8 as char);
        }
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        for c in buffer.chars() {
            command.push(c as i64);
        }
    }
    Ok(())
}
// what we need is ?
//- prime number
//- asterisk
//- mutex
//- mug

