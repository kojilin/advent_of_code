use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use regex::Regex;

#[derive(Clone)]
struct Image {
    id: usize,
    data: Vec<Vec<char>>,
}

impl Image {
    fn add_row(&mut self, row: Vec<char>) {
        self.data.push(row);
    }

    // col change left<->right
    fn flip_vertically(&self) -> Image {
        Image {
            id: self.id,
            data: flip_vertically(&self.data),
        }
    }

    // row change top<->down
    fn flip_horizontally(&self) -> Image {
        Image {
            id: self.id,
            data: flip_horizontally(&self.data),
        }
    }

    fn rotate_clockwise(&self) -> Image {
        Image {
            id: self.id,
            data: rotate_clockwise(&self.data),
        }
    }

    // target_border top:0, right:1, bottom:2, left:3
    fn compute_behavior(&self, target_number: i64, target_border: usize) -> Image {
        let raw_numbers = self.list_border_numbers();
        let flipped_numbers = self.list_flipped_border_numbers();
        if let Some(index) = raw_numbers.iter().position(|&n| n == target_number) {
            if index == target_border {
                self.clone()
            } else if index == (target_border + 3) % 4 {
                if target_border == 0 {
                    self.rotate_clockwise().flip_vertically()
                } else if target_border == 1 {
                    self.rotate_clockwise()
                } else if target_border == 2 {
                    self.rotate_clockwise().flip_vertically()
                } else {
                    self.rotate_clockwise()
                }
            } else if index == (target_border + 2) % 4 {
                if target_border == 0 {
                    self.flip_horizontally()
                } else if target_border == 1 {
                    self.flip_vertically()
                } else if target_border == 2 {
                    self.flip_horizontally()
                } else {
                    self.flip_vertically()
                }
            } else {
                if target_border == 0 {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise()
                } else if target_border == 1 {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise()
                        .flip_horizontally()
                } else if target_border == 2 {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise()
                } else {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise()
                        .flip_horizontally()
                }
            }
        } else if let Some(index) = flipped_numbers.iter().position(|&n| n == target_number) {
            if index == target_border {
                if target_border == 0 {
                    self.flip_vertically()
                } else if target_border == 1 {
                    self.flip_horizontally()
                } else if target_border == 2 {
                    self.flip_vertically()
                } else {
                    self.flip_horizontally()
                }
            } else if index == (target_border + 3) % 4 {
                if target_border == 0 {
                    self.rotate_clockwise()
                } else if target_border == 1 {
                    self.rotate_clockwise().flip_horizontally()
                } else if target_border == 2 {
                    self.rotate_clockwise()
                } else {
                    self.rotate_clockwise().flip_horizontally()
                }
            } else if index == (target_border + 2) % 4 {
                if target_border == 0 {
                    self.flip_horizontally().flip_vertically()
                } else if target_border == 1 {
                    self.flip_vertically().flip_horizontally()
                } else if target_border == 2 {
                    self.flip_horizontally().flip_vertically()
                } else {
                    self.flip_vertically().flip_horizontally()
                }
            } else {
                if target_border == 0 {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise().flip_vertically()
                } else if target_border == 1 {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise()
                } else if target_border == 2 {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise().flip_vertically()
                } else {
                    self.rotate_clockwise().rotate_clockwise().rotate_clockwise()
                }
            }
        } else {
            unreachable!()
        }
    }

    fn calculate_unified_row_pattern(&self, row_index: usize) -> i64 {
        max(self.calculate_row_pattern(row_index, false), self.calculate_row_pattern(row_index, true))
    }

    fn calculate_row_pattern(&self, row_index: usize, reverse: bool) -> i64 {
        if reverse {
            i64::from_str_radix(&self.data[row_index].clone().into_iter().rev().collect::<String>(), 2).unwrap()
        } else {
            i64::from_str_radix(&self.data[row_index].clone().into_iter().collect::<String>(), 2).unwrap()
        }
    }

    fn calculate_unified_col_pattern(&self, col_index: usize) -> i64 {
        max(self.calculate_col_pattern(col_inndex, false),
            self.calculate_col_pattern(col_inndex, true))
    }

    fn calculate_col_pattern(&self, col_index: usize, reverse: bool) -> i64 {
        let mut col = vec![];
        for index in 0..self.data.len() {
            col.push(self.data[index][col_index]);
        }
        if reverse {
            i64::from_str_radix(&col.clone().into_iter().rev().collect::<String>(), 2).unwrap()
        } else {
            i64::from_str_radix(&col.clone().into_iter().collect::<String>(), 2).unwrap()
        }
    }

    // Up, right, bottom, left with largest for easy matching.
    fn list_unified_border_numbers(&self) -> Vec<i64> {
        let mut result = vec![];
        result.push(self.calculate_unified_row_pattern(0));
        result.push(self.calculate_unified_col_pattern(self.data[0].len() - 1));
        result.push(self.calculate_unified_row_pattern(self.data.len() - 1));
        result.push(self.calculate_unified_col_pattern(0));
        result
    }

    // Up, right, bottom, left without flip/rotate.
    fn list_border_numbers(&self) -> Vec<i64> {
        let mut result = vec![];
        result.push(self.calculate_row_pattern(0, false));
        result.push(self.calculate_col_pattern(self.data[0].len() - 1, false));
        result.push(self.calculate_row_pattern(self.data.len() - 1, false));
        result.push(self.calculate_col_pattern(0, false));
        result
    }

    // Up, right, bottom, left.
    fn list_flipped_border_numbers(&self) -> Vec<i64> {
        let mut result = vec![];
        result.push(self.calculate_row_pattern(0, true));
        result.push(self.calculate_col_pattern(self.data[0].len() - 1, true));
        result.push(self.calculate_row_pattern(self.data.len() - 1, true));
        result.push(self.calculate_col_pattern(0, true));
        result
    }
}

fn solve_day20() -> Result<(usize, usize), Box<dyn Error>> {
    let title_regex = Regex::new(r"Tile (.*):$")?;
    let input = fs::read_to_string("src/solutions/year_2020/day20.txt")?;
    let mut images = vec![];
    let mut image = None;
    // Change #->1, .->0. Will convert it to number.
    // The 2 same number means it's possible to be nearby image.
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
    images.push(image.unwrap());
    // border data -> Vec<Image>
    let mut inverse_mapping = HashMap::new();
    let mut map = HashMap::new();
    for image in &images {
        let numbers = image.list_unified_border_numbers();
        for number in numbers {
            let count = map.entry(number).or_insert(0);
            *count += 1;
            let mapping_images = inverse_mapping.entry(number).or_insert_with(|| vec![]);
            mapping_images.push(image);
        }
    }
    let mut result = 1;
    let mut corners = vec![];

    // Check each border pattern has how many border.
    // The question is designed that there is no border pattern has borders larger than 2.
    for image in &images {
        let numbers = image.list_unified_border_numbers();
        // find not enough block
        let count = numbers.iter().filter(|&n| *map.get(n).unwrap() == 1).count();
        if count >= 2 {
            result *= image.id;
            corners.push(image);
        }
    }
    println!("corners: {:?}", corners.iter().map(|img| img.id).collect::<Vec<usize>>());
    // due to each border has 1-1 mapping, it should be easy to find the combinations.
    let first_image = corners[0];
    for (index, number) in first_image.list_unified_border_numbers().iter().enumerate() {
        if map.get(&number).unwrap() == &1 {
            // this is 0,3. So just used it.
            println!("{}", index);
        }
    }
    // first image is suitable for left/top after checking the above log.
    let mut big_image = vec![];
    big_image.push(vec![first_image.clone()]);
    // 12x12 big picture
    for i in 0..12 {
        for j in 0..12 {
            if i == 0 && j == 0 {
                continue;
            }
            if j == 0 {
                let prev = &big_image[i - 1][j];
                let bottom = prev.list_unified_border_numbers()[2];
                let actual_bottom = prev.list_border_numbers()[2];
                let &next = inverse_mapping.get(&bottom).unwrap().iter()
                    .filter(|image| image.id != prev.id)
                    .next()
                    .unwrap();
                if j == 0 {
                    big_image.push(vec![]);
                }
                let rotated = next.compute_behavior(actual_bottom, 0);
                big_image[i].push(rotated);
            } else {
                let prev = &big_image[i][j - 1];
                let right = prev.list_unified_border_numbers()[1];
                let actual_right = prev.list_border_numbers()[1];
                let &next = inverse_mapping.get(&right).unwrap().iter()
                    .filter(|image| image.id != prev.id)
                    .next()
                    .unwrap();
                big_image[i].push(next.compute_behavior(actual_right, 3));
            }
        }
    }

    // combine the map(remove the border)
    let mut final_map = vec![vec!['-'; 96]; 96];
    let mut sea_count = 0;
    for i in 0..big_image.len() {
        for j in 0..big_image[0].len() {
            let image = &big_image[i][j];
            for k in 0..8 {
                for l in 0..8 {
                    final_map[i * 8 + k][j * 8 + l] = image.data[1 + k][1 + l];
                    if image.data[1 + k][1 + l] == '1' {
                        sea_count += 1;
                    }
                }
            }
        }
    }

    let mut monster_count = 0;
    let count: usize = count_monster(&final_map);
    if count != 0 {
        monster_count = count;
    }
    println!("{}", monster_count);
    Ok((result, sea_count - 15 * monster_count))
}

// col change left<->right
fn flip_vertically(from: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut data = vec![];
    for r_i in 0..from.len() {
        let mut row = vec![];
        for c_i in 0..from[r_i].len() {
            row.push(from[r_i][c_i]);
        }
        row.reverse();
        data.push(row);
    }
    data
}

// row change top<->down
fn flip_horizontally(from: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut data = from.clone();
    data.reverse();
    data
}

fn rotate_clockwise(from: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = from.len();
    let width = from[0].len();
    let mut data = vec![vec!['-'; height]; width];
    for r_i in 0..height {
        for c_i in 0..width {
            data[c_i][height - 1 - r_i] = from[r_i][c_i];
        }
    }
    data
}

fn count_monster(map: &Vec<Vec<char>>) -> usize {
    let mut points_matrix = vec![
        vec!['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0'],
        vec!['1', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0', '1', '1', '1'],
        vec!['0', '1', '0', '0', '1', '0', '0', '1', '0', '0', '1', '0', '0', '1', '0', '0', '1', '0', '0', '0'],
    ];
    let mut result = 0;
    for _rotate_count in 0..4 {
        points_matrix = rotate_clockwise(&points_matrix);
        // Looks like there is only one direction will be correct, so just do sum.
        result += count_pattern(map, &flip_vertically(&points_matrix));
        result += count_pattern(map, &flip_horizontally(&points_matrix));
        result += count_pattern(map, &points_matrix);
        //we found it.
        if result != 0 {
            break;
        }
    }
    result
}

fn count_pattern(map: &Vec<Vec<char>>, rotated: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut found = true;
            for offset_y in 0..rotated.len() {
                for offset_x in 0..rotated[offset_y].len() {
                    let new_y = y + offset_y;
                    let new_x = x + offset_x;
                    if new_y >= map.len() || new_x >= map[y].len() {
                        found = false;
                        break;
                    }
                    if rotated[offset_y][offset_x] == '1' && map[new_y][new_x] == '0' {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                result += 1;
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let result = solve_day20()?;
        println!("Result1: {}, Result2: {}", result.0, result.1);
        Ok(())
    }
}
