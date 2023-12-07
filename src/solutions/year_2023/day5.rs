use std::cmp::min;
use std::error::Error;
use std::fs;

fn solve_day5() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day5.txt")?;
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Vec<[i64; 3]>> = Vec::new();
    let mut is_map_block = false;
    for line in content.lines() {
        if line.contains("seeds:") {
            seeds = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect()
        } else if line.is_empty() {
            if !is_map_block {
                is_map_block = true;
            }
        } else if line.contains("-to-") {
            is_map_block = true;
            maps.push(Vec::new());
        } else {
            // the map numbers
            let range: Vec<i64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            maps.last_mut().unwrap().push([range[0], range[1], range[2]]);
        }
    }

    let mut answer = i64::MAX;
    for seed in seeds {
        let mut now = seed;
        for map in &maps {
            for x in map {
                if now >= x[1] && now < x[1] + x[2] {
                    now = now - x[1] + x[0];
                    break;
                }
            }
        }
        answer = min(now, answer);
    }
    Ok(answer)
}

fn solve_day5_2() -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string("src/solutions/year_2023/day5.txt")?;
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Vec<[i64; 3]>> = Vec::new();
    let mut is_map_block = false;
    for line in content.lines() {
        if line.contains("seeds:") {
            seeds = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect()
        } else if line.is_empty() {
            if !is_map_block {
                is_map_block = true;
            }
        } else if line.contains("-to-") {
            is_map_block = true;
            maps.push(Vec::new());
        } else {
            // the map numbers
            let range: Vec<i64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            maps.last_mut().unwrap().push([range[0], range[1], range[2]]);
        }
    }

    let mut answer = i64::MAX;
    for i in 0..seeds.len() / 2 {
        let from = seeds[i * 2];
        let to = from + seeds[i * 2 + 1];

        for seed in from..to {
            let mut now = seed;
            for map in &maps {
                for x in map {
                    if now >= x[1] && now < x[1] + x[2] {
                        now = now - x[1] + x[0];
                        break;
                    }
                }
            }
            answer = min(now, answer);
        }
    }
    Ok(answer)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day5()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day5_2()?);
        Ok(())
    }
}