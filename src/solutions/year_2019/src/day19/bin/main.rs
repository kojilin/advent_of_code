use std::error::Error;
use std::fs;

use _2019::int_code_computer_19::IntCodeComputer19;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day19/bin/day19.txt")?;
    let mut codes: Vec<i64> = input.trim().split(",").map(|c| { c.parse::<i64>().unwrap() }).collect();
//    for y in 0..100{
//        for x in 0..100{
//            let mut computer19 = IntCodeComputer19::new(&codes);
//            let result = computer19.compute(&mut vec![x, y]);
//            if result[0] == 1{
//                print!("#");
//            }else{
//                print!(".");
//            }
//        }
//        println!();
//    }

    let mut x = 3;
    let mut y = 4;

    loop {
        let mut computer19 = IntCodeComputer19::new(&codes);
        let result = computer19.compute(&mut vec![x, y]);
        if result[0] == 1 {
            if x >= 100 {
                let mut computer19 = IntCodeComputer19::new(&codes);
                let result = computer19.compute(&mut vec![x - 99, y + 99]);
                if result[0] == 1 {
                    //bingo
                    break;
                }
            }
            y += 1;
            x += 2;
        } else {
            x -= 1;
        }
    }
    println!("{}", (x - 99) * 10000 + y);
    Ok(())
}

fn verify(x: i32, y: i32, codes: &Vec<i64>) {
    for j in y..y + 101 {
        for i in x..x + 101 {
            let mut computer19 = IntCodeComputer19::new(&codes);
            let result = computer19.compute(&mut vec![i, j]);
            print!("{}", result[0]);
        }
        println!();
    }
}
