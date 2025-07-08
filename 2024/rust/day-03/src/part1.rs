use regex::Regex;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let cpas: Vec<&str> = re
        .find_iter(_input)
        .map(|m| m.as_str())
        .collect();

    let re1 = Regex::new(r"[0-9]{1,3}").unwrap();

    let mut numbers: Vec<i32> = vec![];

    for line in cpas
    {
        let s_numbers: Vec<&str> = re1
            .find_iter(line)
           .map(|n| n.as_str())
           .collect();
        for s_number in s_numbers {
            numbers.push(s_number.parse::<i32>().unwrap());
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
