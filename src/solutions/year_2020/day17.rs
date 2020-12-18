use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn solve_day17() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day17.txt")?;
    let mut area = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            if c == '#' {
                row.push(true);
            } else {
                row.push(false);
            }
        }
        area.push(row);
    }
    let mut areas = vec![area];
    for _cycle in 0..6 {
        // _render(&areas);
        let height = areas[0].len();
        let width = areas[0][0].len();
        let mut new_snap_shot = vec![];
        let new_layer_index = areas.len();
        let top = areas.last().unwrap();
        if !is_empty(top) {
            let mut new_layer = vec![];
            //one extra layer
            for cy in -1..=height as i64 {
                let mut row = vec![];
                for cx in -1..=width as i64 {
                    // all neighbor
                    let count = count_neighbor(&areas, new_layer_index as i64, cy as i64, cx as i64);
                    if 3 == count {
                        row.push(true);
                    } else {
                        row.push(false);
                    }
                }
                new_layer.push(row)
            }
            new_snap_shot.push(new_layer);
        }
        for cz in (0..new_layer_index as i64).rev() {
            let mut new_layer = vec![];
            for cy in -1..=height as i64 {
                let mut row = vec![];
                for cx in -1..=width as i64 {
                    let count = count_neighbor(&areas, cz as i64, cy as i64, cx as i64);

                    if cy == -1 || cy == height as i64
                        || cx == -1 || cx == width as i64
                        || !(areas[cz as usize][cy as usize][cx as usize]) {
                        if count == 3 {
                            row.push(true);
                        } else {
                            row.push(false);
                        }
                    } else {
                        if count == 2 || count == 3 {
                            row.push(true);
                        } else {
                            row.push(false);
                        }
                    }
                }
                new_layer.push(row);
            }
            new_snap_shot.push(new_layer);
        }
        areas = new_snap_shot;
        areas.reverse();
    }
    let mut result = 0;
    for i in 0..areas.len() {
        if i != 0 {
            result += count_active(&areas[i]) * 2;
        } else {
            result += count_active(&areas[i]);
        }
    }
    Ok(result)
}

fn _render(areas: &Vec<Vec<Vec<bool>>>) {
    for (index, z) in areas.iter().enumerate().rev() {
        println!("z={}", index);
        for y in z {
            for x in y {
                if *x {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("---cycle---");
}

fn count_neighbor(areas: &Vec<Vec<Vec<bool>>>, center_z: i64, center_y: i64, center_x: i64) -> usize {
    let mut filtered_z = neighbors(areas.len() as i64, center_z);
    if center_z == 0 && areas.len() > 1 {
        // for z == 0, we need to check it's -1 layer.
        filtered_z.push(1);
    }
    let filtered_y = neighbors(areas[0].len() as i64, center_y);
    let filtered_x = neighbors(areas[0][0].len() as i64, center_x);
    filtered_z.iter()
        .flat_map(|&z| filtered_y.iter().map(move |&y| (z, y)))
        .flat_map(|(z, y)| filtered_x.iter().map(move |&x| (z, y, x)))
        .filter(|&(z, y, x)| !(x == center_x && y == center_y && z == center_z)
            && areas[z as usize][y as usize][x as usize])
        .count()
}

fn neighbors(upper: i64, center: i64) -> Vec<i64> {
    [center].iter()
        .flat_map(|&c| [-1, 0, 1].iter().map(move |&offset| offset + c))
        .filter(|&new_c| new_c >= 0 && new_c < upper)
        .collect()
}

fn is_empty(area: &Vec<Vec<bool>>) -> bool {
    count_active(area) == 0
}

fn count_active(area: &Vec<Vec<bool>>) -> usize {
    area.iter()
        .flat_map(|row| row.iter())
        .filter(|active| **active)
        .count()
}

fn solve_day17_2() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day17.txt")?;
    let mut area = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                area.insert((0 as i64, 0 as i64, y as i64, x as i64), true);
            }
        }
    }

    for _cycle in 0..6 {
        let mut counts = HashMap::new();
        for entry in &area {
            let pos = entry.0;
            let &active = area.get(&pos).unwrap_or(&false);
            if !active {
                continue;
            }
            for w in -1..=1 {
                for z in -1..=1 {
                    for y in -1..=1 {
                        for x in -1..=1 {
                            if w == 0 && z == 0 && y == 0 && x == 0 {
                                continue;
                            }
                            let count = counts.entry((pos.0 + w, pos.1 + z, pos.2 + y, pos.3 + x)).or_insert(0);
                            *count += 1;
                        }
                    }
                }
            }
        }

        let mut new_area = HashMap::new();
        for (&pos, &count) in &counts {
            if *area.get(&pos).unwrap_or(&false) {
                if count == 2 || count == 3 {
                    new_area.insert(pos, true);
                }
            } else {
                if count == 3 {
                    new_area.insert(pos, true);
                }
            }
        }
        area = new_area;
    }
    Ok(area.keys().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day17()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day17_2()?);
        Ok(())
    }
}
