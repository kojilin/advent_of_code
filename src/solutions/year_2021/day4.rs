use std::collections::HashMap;
use std::error::Error;
use std::fs;

struct Board {
    // value to position
    positions: HashMap<i32, usize>,
    counter: [i32; 10],
    sum: i32,
}

impl Board {
    pub fn new(slot: Vec<Vec<i32>>) -> Self {
        let mut positions: HashMap<i32, usize> = HashMap::new();
        let mut sum = 0;
        for (y, row) in slot.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                positions.insert(value, y * 5 + x);
                sum += value;
            }
        }
        Board { positions, counter: [0; 10], sum }
    }

    fn add_number(&mut self, number: i32) -> Option<i32> {
        if let Some(&index) = self.positions.get(&number) {
            self.counter[index / 5] += 1;
            self.counter[index % 5 + 5] += 1;
            self.sum -= number;
            if self.counter[index / 5] == 5 || self.counter[index % 5 + 5] == 5 {
                return Some(self.score(number));
            }
        }
        None
    }

    fn score(&self, number: i32) -> i32 {
        return self.sum * number;
    }
}

fn parse_input() -> Result<(Vec<i32>, Vec<Board>), Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day4.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let instructions: Vec<i32> =
        lines[0].split(",")
            .map(|str| str.parse::<i32>().unwrap())
            .collect();
    let mut iter = lines.iter().skip(1);
    let mut boards: Vec<Board> = Vec::new();
    while let Some(_line) = iter.next() {
        // whitespace with new board
        let mut board: Vec<Vec<i32>> = Vec::new();
        for _ in 0..5 {
            if let Some(line) = iter.next() {
                board.push(line.split_whitespace()
                    .map(|str| str.parse::<i32>().unwrap()).collect());
            }
        }
        boards.push(Board::new(board));
    }
    Ok((instructions, boards))
}

fn solve_day4_1() -> Result<i32, Box<dyn Error>> {
    let (instructions, boards) = parse_input()?;
    return Ok(solve_first(instructions, boards));
}

fn solve_first(instructions: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for value in instructions {
        for board in &mut boards {
            if let Some(score) = board.add_number(value) {
                return score;
            }
        }
    }
    panic!()
}

fn solve_day4_2() -> Result<i32, Box<dyn Error>> {
    let (instructions, boards) = parse_input()?;
    return Ok(solve_second(instructions, boards));
}

fn solve_second(instructions: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for value in instructions {
        if boards.len() == 1 {
            boards[0].add_number(value);
            return boards[0].score(value);
        }
        let mut idx = 0 as usize;
        while idx < boards.len() {
            if let Some(_score) = boards[idx].add_number(value) {
                boards.remove(idx);
                continue;
            }
            idx += 1;
        }
    }
    panic!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day4_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day4_2()?);
        Ok(())
    }
}
