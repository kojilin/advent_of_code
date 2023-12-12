use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::error::Error;
use std::fs;

use crate::solutions::year_2023::day7::CardType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum CardType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq)]
struct Card {
    raw_values: [i32; 5],
    card_type: CardType,
    bet: i64,
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.card_type.cmp(&other.card_type);
        if ordering != Equal {
            ordering
        } else {
            for i in 0..5usize {
                let raw_ordering = self.raw_values[i].cmp(&other.raw_values[i]);
                if raw_ordering != Equal {
                    return raw_ordering;
                }
            }
            panic!("can't compare");
        }
    }
}


impl Card {
    fn create(raw: &str, bet: i64) -> Card {
        let mut nums = [0; 14];
        let mut raw_values = [-1; 5];
        for (index, c) in raw.chars().enumerate() {
            match c {
                'A' => {
                    nums[1] += 1;
                    raw_values[index] = 99;
                }
                '2' => {
                    nums[2] += 1;
                    raw_values[index] = 2;
                }
                '3' => {
                    nums[3] += 1;
                    raw_values[index] = 3;
                }
                '4' => {
                    nums[4] += 1;
                    raw_values[index] = 4;
                }
                '5' => {
                    nums[5] += 1;
                    raw_values[index] = 5;
                }
                '6' => {
                    nums[6] += 1;
                    raw_values[index] = 6;
                }
                '7' => {
                    nums[7] += 1;
                    raw_values[index] = 7;
                }
                '8' => {
                    nums[8] += 1;
                    raw_values[index] = 8;
                }
                '9' => {
                    nums[9] += 1;
                    raw_values[index] = 9;
                }
                'T' => {
                    nums[10] += 1;
                    raw_values[index] = 10;
                }
                'J' => {
                    nums[11] += 1;
                    raw_values[index] = 11;
                }
                'Q' => {
                    nums[12] += 1;
                    raw_values[index] = 12;
                }
                'K' => {
                    nums[13] += 1;
                    raw_values[index] = 13;
                }
                _ => {
                    panic!("???")
                }
            }
        }
        let card_type = if nums.iter().any(|v| *v == 5) {
            FiveOfAKind
        } else if nums.iter().any(|v| *v == 4) {
            FourOfAKind
        } else if nums.iter().any(|v| *v == 3) && nums.iter().any(|v| *v == 2) {
            FullHouse
        } else if nums.iter().any(|v| *v == 3) {
            ThreeOfAKind
        } else if nums.iter().filter(|&v| *v == 2).count() == 2 {
            TwoPair
        } else if nums.iter().filter(|&v| *v == 2).count() == 1 {
            OnePair
        } else {
            HighCard
        };

        Card { raw_values, card_type, bet }
    }
}


fn solve_day7() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day7.txt")?;
    let mut lines = content.lines();

    let mut cards: Vec<Card> = lines.map(|line| {
        let mut split = line.split_whitespace();
        Card::create(split.next().unwrap(), split.next().unwrap().parse::<i64>().unwrap())
    }).collect();
    cards.sort();
    Ok(cards.iter().enumerate().fold(0, |acc, (index, c)| {
        acc + (index as i64 + 1) * c.bet
    }))
}


#[derive(Debug, Eq)]
struct Card2 {
    raw_values: [i32; 5],
    card_type: CardType,
    bet: i64,
}

impl PartialEq<Self> for Card2 {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd<Self> for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.card_type.cmp(&other.card_type);
        if ordering != Equal {
            ordering
        } else {
            for i in 0..5usize {
                let raw_ordering = self.raw_values[i].cmp(&other.raw_values[i]);
                if raw_ordering != Equal {
                    return raw_ordering;
                }
            }
            panic!("can't compare");
        }
    }
}

impl Card2 {
    fn create(raw: &str, bet: i64) -> Card2 {
        let mut nums = [0; 14];
        let mut raw_values = [-1; 5];
        for (index, c) in raw.chars().enumerate() {
            match c {
                'A' => {
                    nums[1] += 1;
                    raw_values[index] = 99;
                }
                '2' => {
                    nums[2] += 1;
                    raw_values[index] = 2;
                }
                '3' => {
                    nums[3] += 1;
                    raw_values[index] = 3;
                }
                '4' => {
                    nums[4] += 1;
                    raw_values[index] = 4;
                }
                '5' => {
                    nums[5] += 1;
                    raw_values[index] = 5;
                }
                '6' => {
                    nums[6] += 1;
                    raw_values[index] = 6;
                }
                '7' => {
                    nums[7] += 1;
                    raw_values[index] = 7;
                }
                '8' => {
                    nums[8] += 1;
                    raw_values[index] = 8;
                }
                '9' => {
                    nums[9] += 1;
                    raw_values[index] = 9;
                }
                'T' => {
                    nums[10] += 1;
                    raw_values[index] = 10;
                }
                'J' => {
                    nums[0] += 1;
                    raw_values[index] = 0;
                }
                'Q' => {
                    nums[12] += 1;
                    raw_values[index] = 12;
                }
                'K' => {
                    nums[13] += 1;
                    raw_values[index] = 13;
                }
                _ => {
                    panic!("???")
                }
            }
        }
        let card_type = if nums.iter().any(|v| *v == 5)
            // 1J, other 4
            || (*nums.iter().skip(1).max().unwrap() == 4 && nums[0] == 1)
            || (*nums.iter().skip(1).max().unwrap() == 3 && nums[0] == 2)
            || (*nums.iter().skip(1).max().unwrap() == 2 && nums[0] == 3)
            || (*nums.iter().skip(1).max().unwrap() == 1 && nums[0] == 4)
            || nums[0] == 4 {
            FiveOfAKind
        } else if nums.iter().any(|v| *v == 4)
            || (*nums.iter().skip(1).max().unwrap() == 3 && nums[0] == 1)
            || (*nums.iter().skip(1).max().unwrap() == 2 && nums[0] == 2)
            || (*nums.iter().skip(1).max().unwrap() == 1 && nums[0] == 3) {
            FourOfAKind
        } else if (nums.iter().any(|v| *v == 3) && nums.iter().any(|v| *v == 2))
            || (nums.iter().skip(1).filter(|&v| *v == 2).count() == 2 && nums[0] == 1)
        {
            FullHouse
        } else if nums.iter().any(|v| *v == 3)
            || (nums.iter().skip(1).filter(|&v| *v == 2).count() == 1 && nums[0] == 1)
            || (nums.iter().skip(1).filter(|&v| *v == 1).count() == 3 && nums[0] == 2) {
            ThreeOfAKind
        } else if nums.iter().filter(|&v| *v == 2).count() == 2 {
            TwoPair
        } else if nums.iter().filter(|&v| *v == 2).count() == 1
            || (nums.iter().skip(1).filter(|&v| *v == 1).count() == 4 && nums[0] == 1) {
            OnePair
        } else {
            HighCard
        };

        Card2 { raw_values, card_type, bet }
    }
}


fn solve_day7_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day7.txt")?;
    let mut lines = content.lines();

    let mut cards: Vec<Card2> = lines.map(|line| {
        let mut split = line.split_whitespace();
        Card2::create(split.next().unwrap(), split.next().unwrap().parse::<i64>().unwrap())
    }).collect();
    cards.sort();
    Ok(cards.iter().enumerate().fold(0, |acc, (index, c)| {
        acc + (index as i64 + 1) * c.bet
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day7()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day7_2()?);
        Ok(())
    }
}