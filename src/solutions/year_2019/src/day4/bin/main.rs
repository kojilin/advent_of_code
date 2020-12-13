fn main() {
    let input = 171309..=643603;

    let result: Vec<i32> = input.filter(|i| check(i)).collect();
    println!("{}", result.len());
}

fn check(number: &i32) -> bool {
    let chars: Vec<i32> = number.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut has_double = false;
    let mut has_one_pair = false;
    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
            has_double = true;
        } else {
            if count == 2 {
                has_one_pair = true;
            }
            count = 1;
        }
        if chars[i] < chars[i - 1] {
            return false;
        }
    }
    if count == 2 {
        has_one_pair = true;
    }
    has_double && has_one_pair
}
