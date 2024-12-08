use std::error::Error;
use std::fs;

fn solve_day7() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day7.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for line in lines {
        let raw: Vec<&str> = line.split(": ").collect();
        let target = raw[0].parse::<i64>()?;
        let nums: Vec<i64> = raw[1]
            .split(" ")
            .map(|v| v.parse::<i64>())
            .filter_map(|result| result.ok())
            .collect();

        if dfs(&nums, 1, nums[0], target) {
            result += target;
        }
    }
    Ok(result)
}

fn dfs(nums: &Vec<i64>, next_index: usize, current_value: i64, target: i64) -> bool {
    if next_index == nums.len() {
        if target == current_value {
            return true;
        }
        return false;
    }

    if dfs(
        nums,
        next_index + 1,
        current_value + nums[next_index],
        target,
    ) {
        return true;
    }

    return dfs(
        nums,
        next_index + 1,
        current_value * nums[next_index],
        target,
    );
}

fn solve_day7_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day7.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for line in lines {
        let raw: Vec<&str> = line.split(": ").collect();
        let target = raw[0].parse::<i64>()?;
        let nums: Vec<i64> = raw[1]
            .split(" ")
            .map(|v| v.parse::<i64>())
            .filter_map(|result| result.ok())
            .collect();

        if dfs_2(&nums, 1, nums[0], target) {
            result += target;
        }
    }
    Ok(result)
}

fn dfs_2(nums: &Vec<i64>, next_index: usize, current_value: i64, target: i64) -> bool {
    if next_index == nums.len() {
        if target == current_value {
            return true;
        }
        return false;
    }

    if dfs_2(
        nums,
        next_index + 1,
        current_value + nums[next_index],
        target,
    ) {
        return true;
    }

    if dfs_2(
        nums,
        next_index + 1,
        (current_value.to_string() + &nums[next_index].to_string())
            .parse::<i64>()
            .unwrap(),
        target,
    ) {
        return true;
    }

    return dfs_2(
        nums,
        next_index + 1,
        current_value * nums[next_index],
        target,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_1() {
        println!("result: {:?}", solve_day7());
    }

    #[test]
    fn test_day7_2() {
        println!("result: {:?}", solve_day7_2());
    }
}
