use std::cmp::max;
use std::error::Error;
use std::fs;

use _2019::int_code_computer::IntCodeComputer;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day7/bin/day7.txt")?;
    let original_codes: Vec<i32> = input.split(',')
        .map(|s: &str| s.trim().parse::<i32>().unwrap())
        .collect();

    let mut combinations = vec![vec![5], vec![6], vec![7], vec![8], vec![9]];
    while !combinations.is_empty() {
        if combinations[0].len() == 5 {
            break;
        }
        let target = combinations.remove(0);
        for i in 5..10 {
            if !target.contains(&i) {
                let mut new_vector = target.clone();
                new_vector.push(i);
                combinations.push(new_vector);
            }
        }
    }
    let mut result = -1;
    for combination in combinations {
        let mut output = 0;
        let mut computers = vec![
            IntCodeComputer::new(&original_codes),
            IntCodeComputer::new(&original_codes),
            IntCodeComputer::new(&original_codes),
            IntCodeComputer::new(&original_codes),
            IntCodeComputer::new(&original_codes)];
        // initialize phases
        for (i, phase) in combination.iter().enumerate() {
            computers[i].compute(*phase);
        }

        let mut halt = false;
        while !halt {
            for i in 0..5 {
                output = computers[i].compute(output);
                halt |= computers[i].is_halt();
            }
        }
        result = max(result, output);
    }
    println!("{}", result);
    Ok(())
}
