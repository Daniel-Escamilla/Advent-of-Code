pub fn collect_valid_pairs(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split("|")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect::<Vec<usize>>()
            .into_iter()
            .rev()
            .collect()
        })
        .collect()
}

pub fn collect_order_pages(input: String) -> Vec<Vec<usize>> {
    input
    .lines()
    .map(|line| {
        line
            .split(",")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect()
    })
    .collect()
}

pub fn checker(numbers_to_check: &Vec<Vec<usize>>, valid_numbers: &Vec<Vec<usize>>) -> bool {

    let mut new_numbers: Vec<Vec<usize>> = vec![];
    for numbers in numbers_to_check {
        for i in 0..numbers.len() {
            let first = numbers[i];
            for numbers  in numbers.iter().skip(i + 1) {
                new_numbers.push(vec![first, *numbers]);
            }
        }
    }
    for numbers in &new_numbers {
        for wrong in valid_numbers {
            if numbers == wrong {
                return false;
            }
        }
    }
    true
}

fn insert_or_replace(result: &mut Vec<Vec<usize>>, new: Vec<usize>) {
    if let Some(pos) = result.iter().position(|elem| elem[1] == new[1]) {
        result[pos] = new;
    } else {
        result.push(new);
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let block1 = _input.split("\n\n").next().unwrap();
    let block2 = _input.split("\n\n").nth(1).unwrap();

    let valid_pairs: Vec<Vec<usize>> = collect_valid_pairs(block1.to_string());

    let mut all_numbers: Vec<Vec<usize>> = collect_order_pages(block2.to_string());

    let mut result: Vec<Vec<usize>> = vec![];

    // let line: usize = 0;

    loop {
        for (line, numbers) in &mut all_numbers.iter_mut().enumerate() {
            let mut status: bool = true;
            let mut position_swap: Vec<Vec<usize>> = vec![];
            let i: usize = 0;
            let mut j: usize = i + 1;
            for (i, num) in numbers.windows(2).enumerate() {
                for pairs in &valid_pairs {
                    for pair in pairs.windows(2) {
                        if num == pair {
                            status = false;
                            position_swap.push(vec![i, j]);
                        }
                    }
                }
                j += 1;
            }
            for pos in position_swap {
                let x = pos[0];
                let y = pos[1];
                numbers.swap(x, y);
            }
            if !status {
                insert_or_replace(&mut result, vec![numbers[numbers.len() / 2], line]);
            }
        }
        if checker(&all_numbers, &valid_pairs) {
            break;
        }
    }
    result.reverse();

    let mut finish: usize = 0 ;

    for r in result {
        finish += r[0];
    }
    Ok(finish.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";



        assert_eq!("123", process(input)?);
        Ok(())

    }
}