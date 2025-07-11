
pub fn collect_valid_pairs(input: String) -> Vec<Vec<usize>> {
    input
    .lines()
    .map(|line| {
        line.split("|")
        .filter_map(|num| num.parse::<usize>().ok())
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
        .collect::<Vec<usize>>()
        .into_iter()
        .rev()
        .collect()
    })
    .collect()
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let block1 = _input.split("\n\n").next().unwrap();
    let block2 = _input.split("\n\n").nth(1).unwrap();

    let valid_pairs: Vec<Vec<usize>> = collect_valid_pairs(block1.to_string());

    let mut all_numbers: Vec<Vec<usize>> = collect_order_pages(block2.to_string());

    let mut result: usize = 0;

    for numbers in &mut all_numbers {
        let mut status: bool = true;
        for i in 0..numbers.len() {
            let first = numbers[i];
            for numbers  in numbers.iter().skip(i + 1)
            {
                for pair in &valid_pairs {
                    if *pair == vec![first, *numbers] {
                        status = false;
                        break ;
                    }
                }
                if !status {
                    break ;
                }
            }
            if !status {
                break ;
            }
        }
        if status {
            result += numbers[numbers.len() / 2];
        }
    }

    // println!("{result}");

    Ok(result.to_string())
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
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
