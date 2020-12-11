use std::cmp::min;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day16/bin/day16.txt")?;
    let raw_numbers: Vec<i32> = input.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut offset = 0;
    for num in raw_numbers[0..7].iter() {
        offset *= 10;
        offset += *num;
    }

    let mut numbers: Vec<i64> = Vec::new();

    for _ in 0..10000 {
        for j in raw_numbers.iter() {
            numbers.push(j.clone() as i64);
        }
    }

    for count in 1..101 {
        let mut temp: Vec<i64> = Vec::new();
        numbers.insert(0, 0);
        for i in 1..numbers.len() {
            numbers[i] = numbers[i - 1] + numbers[i];
        }

        // for 1
        let mut step = 1;
        for _ in 1..numbers.len() {
            let mut result = 0;
            //for 1
            let mut base = step;
            while base < numbers.len() {
                let from = base - 1;
                let end = min(base + step - 1, numbers.len() - 1);
                result += numbers[end] - numbers[from];
                base += 4 * step;
            }

            // for -1
            let mut base = 3 * step;
            while base < numbers.len() {
                let from = base - 1;
                let end = min(base + step - 1, numbers.len() - 1);
                result -= numbers[end] - numbers[from];
                base += 4 * step;
            }
            temp.push(result.abs() % 10);
            step += 1;
        }
        numbers.clear();
        for i in 0..temp.len() {
            numbers.push(temp[i] as i64);
        }
        println!("{:?} done.", count);
    }
    let mut result = String::new();
    for i in offset..offset + 8 {
        result.push_str(&numbers[i as usize].to_string());
    }
    println!("{}", result);
    Ok(())
}

