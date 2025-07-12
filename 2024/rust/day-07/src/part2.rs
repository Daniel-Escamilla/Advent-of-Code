
pub fn concatenation(current_value: usize, number: usize) -> Option<usize> {

    let power_of_10 = 10_usize.pow(number.to_string().len() as u32);

    if let Some(val) = current_value.checked_mul(power_of_10) {
        if let Some(result) = val.checked_add(number) {
            return Some(result);
        }
    }
    None
}

pub fn can_reach_target(numbers: &[usize], target: usize) -> bool {
    fn backtrack(nums: &[usize], index: usize, current_value: usize, target: usize) -> bool {
        if index == nums.len() {
            return current_value == target;
        }
        if backtrack(nums, index +1 , current_value + nums[index], target) {
            return true;
        }
        if let Some(val) = current_value.checked_mul(nums[index]) {
            if backtrack(nums, index + 1, val, target) {
                return true;
            }
        }
        if let Some(val) = concatenation(current_value, nums[index]) {
            if backtrack(nums, index + 1, val, target) {
                return true;
            }
        }
        false
    }
    if numbers.is_empty() {
        return false;
    }
    backtrack(numbers, 1, numbers[0], target)
}

pub fn process(input: &str) -> miette::Result<String> {

    let results: Vec<usize> = input.lines().filter_map(|num| num.split(":").next()?.trim().parse::<usize>().ok()).collect();
    let numbers: Vec<Vec<usize>> = input
            .lines()
            .filter_map(|line| {
                line.split(":")
                .nth(1)
                .map(|number| {
                    number.split_whitespace()
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
                })
            })
            .collect();
    let mut total: usize = 0;

    for (results, numbers) in results.iter().zip(numbers.iter()) {
        if can_reach_target(numbers, *results) {
            total += *results;
        }
    }
    


    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
