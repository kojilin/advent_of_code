use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs;

fn solve_day15_1() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day15_2() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2021/day15.txt")?;
    let mut map = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let risk = c.to_digit(10).unwrap();
            row.push(risk);
        }
        map.push(row);
    }
    Ok(map)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
    cost: u32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_1(map: Vec<Vec<u32>>) -> i32 {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut visited = vec![vec![false; width as usize]; height as usize];
    let mut heap = BinaryHeap::new();
    heap.push(Point { x: 0, y: 0, cost: 0 });
    let direction = [0, 1, 0, -1, 0];
    while !heap.is_empty() {
        let point = heap.pop().unwrap();
        if point.x == width - 1 && point.y == height - 1 {
            return point.cost as i32;
        }
        if visited[point.y as usize][point.x as usize] {
            continue;
        }
        visited[point.y as usize][point.x as usize] = true;
        for i in 0..4 {
            let new_y = point.y + direction[i];
            let new_x = point.x + direction[i + 1];
            if new_y >= 0 && new_y < height && new_x >= 0 && new_x < width
                && !visited[new_y as usize][new_x as usize] {
                heap.push(Point {
                    y: new_y,
                    x: new_x,
                    cost: point.cost + map[new_y as usize][new_x as usize],
                });
            }
        }
    }
    panic!()
}

fn solve_2(map: Vec<Vec<u32>>) -> i32 {
    let height = (map.len() * 5) as i32;
    let width = (map[0].len() * 5) as i32;
    let mut visited = vec![vec![false; width as usize]; height as usize];
    let mut heap = BinaryHeap::new();
    heap.push(Point { x: 0, y: 0, cost: 0 });
    let direction = [0, 1, 0, -1, 0];
    while !heap.is_empty() {
        let point = heap.pop().unwrap();
        if point.x == width - 1 && point.y == height - 1 {
            return point.cost as i32;
        }
        if visited[point.y as usize][point.x as usize] {
            continue;
        }
        visited[point.y as usize][point.x as usize] = true;
        for i in 0..4 {
            let new_y = point.y + direction[i];
            let new_x = point.x + direction[i + 1];
            if new_y >= 0 && new_y < height && new_x >= 0 && new_x < width
                && !visited[new_y as usize][new_x as usize] {
                heap.push(Point {
                    y: new_y,
                    x: new_x,
                    cost: point.cost + find_cost(new_y as usize, new_x as usize, &map),
                });
            }
        }
    }
    panic!()
}

fn find_cost(y: usize, x: usize, map: &Vec<Vec<u32>>) -> u32 {
    let height = map.len();
    let width = map[0].len();
    let div_y = y / height;
    let div_x = x / width;
    let mod_y = y % height;
    let mod_x = x % width;
    let cost = map[mod_y][mod_x] + div_y as u32 + div_x as u32;
    if cost % 9 == 0 {
        return 9;
    }
    return cost % 9;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day15_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day15_2()?);
        Ok(())
    }
}

