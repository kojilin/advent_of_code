use std::collections::HashMap;
use std::error::Error;
use std::fs;

use _2019::int_code_computer_17::IntCodeComputer17;
use _2019::int_code_computer_17_2::IntCodeComputer172;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day17/bin/day17.txt")?;
    let mut codes: Vec<i64> = input.trim().split(",").map(|c| { c.parse::<i64>().unwrap() }).collect();
    let mut computer17 = IntCodeComputer17::new(&codes);
    let vec = computer17.compute();
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut count: HashMap<(i32, i32), i32> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut result = 0;
    let around = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];

    for v in vec {
        match v {
            35 => {
                map.insert((x, y), '#');
                for dir in around.iter() {
                    let c = count.entry((x + dir.0, y + dir.1)).or_insert(0);
                    *c += 1;
                    if *c == 5 {
                        result += (x + dir.0) * (y + dir.1);
                    }
                }
                x += 1;
                print!("#");
            }
            46 => {
                map.insert((x, y), '.');
                x += 1;
                print!(".");
            }
            10 => {
                x = 0;
                y += 1;
                println!();
            }
            _ => {
                map.insert((x, y), v as u8 as char);
                print!("{}", v as u8 as char);
            }
        }
    }
    println!("{}", result);

    let input: Vec<char> = vec!['A',',','B',',','A',',','C',',','A',',','B',',','C',',','B',',','C',',','B', '\n', 'R',',','1','0',',','R',',','1','0',',','R',',','6',',','R',',','4', '\n', 'R',',','1','0',',','R',',','1','0',',','L',',','4', '\n', 'R',',','4',',','L',',','4',',','L',',','1','0',',','L',',','1','0', '\n', 'n','\n'];
    let mut input_number: Vec<i32> = input.iter().map(|c| { c.clone() as i32 }).collect();
    // part 2
    codes[0] = 2;
    println!("{:?}", input_number);

    let mut computer172 = IntCodeComputer172::new(&codes);
    let result = computer172.compute(&mut input_number);
    println!("{:?}", result);

    Ok(())
}
// A,B,A,C,A,B,C,B,C,B
// R,10,R,10,R,6,R,4
// R,10,R,10,L,4
// R,4,L,4,L,10,L,10
