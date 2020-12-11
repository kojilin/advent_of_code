use std::error::Error;
use std::fs;

use _2019::int_code_computer_21::IntCodeComputer21;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day21/bin/day21.txt")?;
    let codes: Vec<i64> = input.trim().split(",").map(|c| c.parse::<i64>().unwrap()).collect();
    let mut computer21 = IntCodeComputer21::new(&codes);
    println!("{}", '.' as i32);
    // check 1,2,3 is walkable, then check 4, 8 is not a hole, jump.
    // NOT A T, OR A T is resetting.
    let commands = "OR A T\nAND B T\nAND C T\nNOT T J\nNOT A T\nOR A T\nAND D T\nAND H T\nAND T J\nNOT A T\nOR T J\nRUN\n";
    let mut i64_command: Vec<i32> = commands.chars().map(|c| c as i32).collect();
    let result = computer21.compute(&mut i64_command);
    for x in result.iter() {
        print!("{}", *x as u8 as char);
    }
    println!("{:?}", result.last());
    Ok(())
}
