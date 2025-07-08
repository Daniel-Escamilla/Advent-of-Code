use regex::Regex;

#[tracing::instrument]

pub fn line_into_vector(line: &str) -> Vec<&str>
{
    let re1: Regex = Regex::new(r"[0-9]{1,3}").unwrap();
    let s_numbers: Vec<&str> = re1
        .find_iter(line)
        .map(|n| n.as_str())
        .collect();
    s_numbers
}

pub fn process(_input: &str) -> miette::Result<String> {

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don\'t\(\)").unwrap();

    let cpas: Vec<&str> = re
        .find_iter(_input)
        .map(|m| m.as_str())
        .collect();

    let mut numbers: Vec<i32> = vec![];
    let mut enable: bool = true;

    for line in cpas
    {
        if line == "don't()" {
            enable = false;
        } else if line == "do()" {
            enable = true;
        } else {
            if !enable {
                continue ;
            }
            let s_numbers = line_into_vector(line);
            for s_number in s_numbers {
                numbers.push(s_number.parse::<i32>().unwrap());
            }
        }
    }
    let mut result: i32 = 0;

    for number in numbers.chunks(2) {
        result += number[0] * number[1];
    }


    Ok(result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
