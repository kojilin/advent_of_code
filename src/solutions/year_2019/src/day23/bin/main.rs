use std::collections::HashMap;
use std::error::Error;
use std::fs;

use _2019::int_code_computer_23::IntCodeComputer23;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day23/bin/day23.txt")?;
    let codes: Vec<i64> = input.trim().split(",").map(|c| c.parse::<i64>().unwrap()).collect();

    let mut computers = HashMap::new();
    let mut packets = vec![];
    for i in 0..50 {
        let mut computer = IntCodeComputer23::new(&codes);
        let outputs = computer.compute(&mut vec![i as i64, -1]);
        let mut index = 0;
        while index < outputs.len() {
            packets.push((outputs[index], outputs[index + 1], outputs[index + 2]));
            index += 3;
        }
        computers.insert(i as i64, computer);
    }


    let mut cached = (-1, -1);
    loop {
        if !packets.is_empty() {
            let (to, x, y) = packets.remove(0);
            if to == 255 {
                cached = (x, y);
                continue;
            }
            if to < 0 || to >= 50 {
                continue;
            }

            let computer = computers.get_mut(&to).unwrap();
            let outputs = computer.compute(&mut vec![x, y, -1]);
            let mut index = 0;
            while index < outputs.len() {
                packets.push((outputs[index], outputs[index + 1], outputs[index + 2]));
                index += 3;
            }
        } else {
            let computer = computers.get_mut(&(0 as i64)).unwrap();
            let outputs = computer.compute(&mut vec![cached.0, cached.1]);
            if outputs.is_empty() {
                println!("y {}", cached.1);
                break;
            }
            let mut index = 0;
            while index < outputs.len() {
                packets.push((outputs[index], outputs[index + 1], outputs[index + 2]));
                index += 3;
            }
        }
    }
    Ok(())
}

