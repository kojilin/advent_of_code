use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day8/bin/day8.txt")?;
    let pixels: Vec<i32> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let mut i = 0;
    let mut min = std::i32::MAX;
    let mut result = 0;
    let mut colors = [2; 150];

    while i < pixels.len() {
        for j in 0..150 {
            match pixels[i + j] {
                0 => {
                    if colors[(i + j) % 150] == 2 {
                        colors[(i + j) % 150] = 0;
                    }
                }
                1 => {
                    if colors[(i + j) % 150] == 2 {
                        colors[(i + j) % 150] = 1;
                    }
                }
                2 => {}
                _ => {}
            }
        }
        i += 150;
    }

    for i in 0..6 {
        for j in 0..25 {
            print!("{}", colors[i * 25 + j]);
        }
        print!("\n");
    }
    Ok(())
}
