use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let n = 119_315_717_514_047;
    let mut result_i = 2020;
    let input = fs::read_to_string("src/day22/bin/day22.txt")?;
    let lines = input.lines().rev();
    let mut cache = Vec::new();
    cache.push(result_i);
    for i in 0..101_741_582_076_661 as i64 {
        for line in lines.clone() {
            let line = line.trim();
            if line == "deal into new stack" {
                result_i = reverse_v2(n, result_i);
            } else if line.starts_with("deal with increment ") {
                let num = &line[20..].trim().parse::<i32>().unwrap();
                result_i = increment_v2(n, *num as i64, result_i);
            } else if line.starts_with("cut ") {
                let num = &line[4..].trim().parse::<i32>().unwrap();
                result_i = cut_v2(n, *num as i64, result_i);
            }
        }
        if cache.contains(&result_i) {
            let index = 101_741_582_076_661 % cache.len();
            println!("{} and result is {}", cache.len(), cache[index]);
            break;
        }
        cache.push(result_i);
    }
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let mut cards = Vec::new();
    for i in 0..10007 {
        cards.push(i);
    }

    let input = fs::read_to_string("src/day22/bin/day22.txt")?;
    let lines = input.lines();
    for line in lines {
        let line = line.trim();
        if line == "deal into new stack" {
            cards.reverse();
        } else if line.starts_with("deal with increment ") {
            let num = &line[20..].trim().parse::<i32>().unwrap();
            cards = increment(&cards, *num);
        } else if line.starts_with("cut ") {
            let num = &line[4..].trim().parse::<i32>().unwrap();
            cards = cut(&cards, *num);
        }
    }

    for i in 0..cards.len() {
        if cards[i] == 2019 {
            println!("{:?}", i);
            break;
        }
    }
    Ok(())
}

fn reverse_v2(n: i64, result_i: i64) -> i64 {
    n - result_i - 1
}

// p * return % n = result_i
fn increment_v2(n: i64, p: i64, result_i: i64) -> i64 {
    let mut result = 0 as i64;
    let n2 = n % p;
    loop {
        if (result * n2 + result_i) % p == 0 {
            break;
        }
        result += 1;
    }
    (result * n + result_i) / p
}

fn cut_v2(n: i64, mut cut: i64, result_i: i64) -> i64 {
    if cut < 0 {
        cut = n + cut;
    }
    let index = (result_i + cut) % n;
    index
}

fn increment(cards: &Vec<i64>, num: i32) -> Vec<i64> {
    let mut new_cards = Vec::new();
    new_cards.resize(cards.len(), -1);

    for i in 0..cards.len() {
        new_cards[i * num as usize % cards.len()] = cards[i];
    }
    new_cards
}

fn cut(cards: &Vec<i64>, cut_num: i32) -> Vec<i64> {
    let mut new_cards = Vec::new();
    if cut_num > 0 {
        let cut_num = cut_num as usize;

        for i in cut_num..cards.len() {
            new_cards.push(cards[i]);
        }

        for i in 0..cut_num {
            new_cards.push(cards[i]);
        }
    } else {
        let cut_num = (cards.len() as i32 + cut_num) as usize;
        for i in cut_num..cards.len() {
            new_cards.push(cards[i]);
        }

        for i in 0..cut_num {
            new_cards.push(cards[i]);
        }
    }
    new_cards
}
