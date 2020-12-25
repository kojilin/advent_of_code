use std::error::Error;

fn day25(key1: i64, key2: i64) -> Result<i64, Box<dyn Error>> {
    let num1 = find_value(key1);
    let mut result = 1;
    for _ in 0..num1 {
        result *= key2;
        result %= 20201227;
    }
    Ok(result)
}

fn find_value(key: i64) -> i64 {
    let mut num = 1i64;
    let mut count = 1;
    loop {
        num *= 7;
        num %= 20201227;
        if num == key {
            return count as i64;
        }
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", day25(16616892, 14505727)?);
        Ok(())
    }
}
