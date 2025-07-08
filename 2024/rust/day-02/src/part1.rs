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
        let up: bool = line[0] < line[1];
        let mut count: bool = true;
        for pair in line.windows(2) {
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
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
