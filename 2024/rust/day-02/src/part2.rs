#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let container: Vec<Vec<i32>> = input
            .lines()
            .map(| line | {
                line
                    .split_whitespace()
                    .map(| s | s.parse::<i32>().expect("Invalid Number"))
                    .collect()
            })
            .collect();

    let mut safe: i32 = 0;

    for line in container {
        if line.len() < 2 {
            continue ;
        }
        let mut count: bool = true;

        let mut idx: usize = 0;

        while idx < line.iter().len()
        {
            let mut new_pair = line.clone();

            new_pair.remove(idx);

            count = true;
            let up: bool;
            if idx == 1 {
                up = line[0] < line[2];
            }
            else if idx == 0 {
                up = line[1] < line[2];
            }
            else {
                up = line[0] < line[1];
            }
            for pair in new_pair.windows(2)
            {
                if pair[0] == pair[1] {
                    count = false;
                }
                if up != (pair[0] < pair[1]) {
                    count = false;
                    break ;
                }
                if (pair[0] - pair[1]).abs() > 3 {
                    count = false;
                    break ;
                }
            }
            if count
            {
                break ;
            }
            idx += 1;
        }
        if count {
            safe += 1;
        }
    }
    Ok(safe.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
