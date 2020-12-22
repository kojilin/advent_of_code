use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn solve_day21() -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day21.txt")?;
    // allergen -> possible ingredients
    let mut map = HashMap::new();
    let mut all: Vec<&str> = vec![];

    for line in input.lines() {
        let line: Vec<&str> = line.split(" (contains ").collect();
        let ingredients: HashSet<&str> = line[0].split_whitespace().collect();
        all.extend(ingredients.iter());

        let allergens: Vec<&str> = line[1][0..line[1].len() - 1].split(", ").collect();
        for allergen in allergens {
            let possible = map.entry(allergen).or_insert_with(|| HashSet::new());
            if possible.is_empty() {
                for &ingredient in &ingredients {
                    possible.insert(ingredient);
                }
            } else {
                let intersection = possible.intersection(&ingredients);
                let mut remains = HashSet::new();
                for &remain in intersection {
                    remains.insert(remain);
                }
                map.insert(allergen, remains);
            }
        }
    }
    let mut dangerous_foods = HashSet::new();
    let mut possibles: Vec<HashSet<&str>> = map.drain().map(|(_, v)| v).collect();
    loop {
        possibles.sort_by(|a, b| a.len().cmp(&b.len()));
        if possibles.is_empty() {
            break;
        }
        let first = possibles.remove(0);

        if first.len() != 1 {
            // oops?
            panic!();
        }
        let &target = first.iter().next().unwrap();

        dangerous_foods.insert(target);

        for possible in possibles.iter_mut() {
            possible.remove(target);
        }
    }
    Ok(all.iter().filter(|&name| !dangerous_foods.contains(name)).count())
}

fn solve_day21_2() -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day21.txt")?;
    // allergen -> possible ingredients
    let mut map = HashMap::new();
    let mut all: Vec<&str> = vec![];

    for line in input.lines() {
        let line: Vec<&str> = line.split(" (contains ").collect();
        let ingredients: HashSet<&str> = line[0].split_whitespace().collect();
        all.extend(ingredients.iter());

        let allergens: Vec<&str> = line[1][0..line[1].len() - 1].split(", ").collect();
        for allergen in allergens {
            let possible = map.entry(allergen).or_insert_with(|| HashSet::new());
            if possible.is_empty() {
                for &ingredient in &ingredients {
                    possible.insert(ingredient);
                }
            } else {
                let intersection = possible.intersection(&ingredients);
                let mut remains = HashSet::new();
                for &remain in intersection {
                    remains.insert(remain);
                }
                map.insert(allergen, remains);
            }
        }
    }
    let mut result: Vec<(&str, &str)> = vec![];
    let mut dangerous_foods = HashSet::new();
    let mut possibles: Vec<(&str, HashSet<&str>)> = map.drain().collect();
    loop {
        possibles.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        if possibles.is_empty() {
            break;
        }
        let first = possibles.remove(0);
        if first.1.len() != 1 {
            // oops?
            panic!();
        }

        let &target = first.1.iter().next().unwrap();
        result.push((first.0, target));
        dangerous_foods.insert(target);
        for possible in possibles.iter_mut() {
            possible.1.remove(target);
        }
    }
    result.sort_by(|a, b| a.0.cmp(&b.0));
    let result = result.into_iter()
        .map(|(_allergen, ingredient)| ingredient)
        .collect::<Vec<&str>>()
        .join(",");
    Ok(result.clone())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result1: {}", solve_day21()?);
        Ok(())
    }


    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result2: {}", solve_day21_2()?);
        Ok(())
    }
}
