use std::collections::HashMap;
use std::error::Error;
use std::fs;

use regex::Regex;

struct Image {
    id: usize,
    data: Vec<Vec<char>>,
}

impl Image {
    fn add_row(&mut self, row: Vec<char>) {
        self.data.push(row);
    }

    fn count_row(&self, row_index: usize) -> i64 {
        let top_1 = i64::from_str_radix(&self.data[row_index].clone().into_iter().collect::<String>(), 2).unwrap();
        let top_2 = i64::from_str_radix(&self.data[row_index].clone().into_iter().rev().collect::<String>(), 2).unwrap();
        if top_1 > top_2 {
            top_1
        } else {
            top_2
        }
    }

    fn count_col(&self, col_index: usize) -> i64 {
        let mut col = vec![];
        for index in 0..self.data.len() {
            col.push(self.data[index][col_index]);
        }
        let left_1 = i64::from_str_radix(&col.clone().into_iter().collect::<String>(), 2).unwrap();
        let left_2 = i64::from_str_radix(&col.clone().into_iter().rev().collect::<String>(), 2).unwrap();
        if left_1 > left_2 {
            left_1
        } else {
            left_2
        }
    }

    fn all_number(&self) -> Vec<i64> {
        let mut result = vec![];
        result.push(self.count_row(0));
        result.push(self.count_row(self.data.len() - 1));
        result.push(self.count_col(0));
        result.push(self.count_col(self.data[0].len() - 1));
        result
    }
}

fn solve_day20() -> Result<usize, Box<dyn Error>> {
    let title_regex = Regex::new(r"Tile (.*):$")?;
    let input = fs::read_to_string("src/solutions/year_2020/day20.txt")?;
    let mut images = vec![];
    let mut image = None;
    for line in input.lines() {
        // input is 10x10
        if title_regex.is_match(line) {
            let capture = title_regex.captures(line).unwrap();
            let tile_id = capture[1].parse::<usize>()?;
            image = Some(Image {
                id: tile_id,
                data: vec![],
            });
        } else if line.is_empty() {
            images.push(image.unwrap());
            image = None;
        } else {
            image.as_mut().unwrap().add_row(line.chars().map(|c| if c == '#' { '1' } else { '0' }).collect());
        }
    }

    let mut map = HashMap::new();
    for image in &images {
        let numbers = image.all_number();
        for number in numbers {
            let count = map.entry(number).or_insert(0);
            *count += 1;
        }
    }
    let mut result = 1;
    for image in &images {
        let numbers = image.all_number();
        // find not enough block
        let count = numbers.iter().filter(|&n| *map.get(n).unwrap() == 1).count();
        if count >= 2 {
            result *= image.id;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day20()?);
        Ok(())
    }
}
