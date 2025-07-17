#[tracing::instrument]

pub fn process(input: &str) -> miette::Result<String> {

    let high_index: u32 = input
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum();

    
    let mut reverse: Vec<_> = (0..input.len())
            .rev()
            .zip(input.chars().rev())
            .scan(high_index, | base_index, (compressed_index, c)| {
                let num_indices = c.to_digit(10).unwrap();
                *base_index -= num_indices;
                Some(
                    (*base_index..(*base_index + num_indices))
                        .rev()
                        .filter_map(move |i| {
                            (compressed_index % 2 == 0)
                                .then_some((
                                    i,
                                    compressed_index / 2,
                                    num_indices,
                                ))
                        }),
                )
            },
        )
        .flatten()
        .collect();


    // INTENTO FALLIDO -> VOLVER A INTENTARLO

    let mut base_index = 0;
    let mut sum = 0;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indices = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in base_index..(base_index + num_indices) {
            if compressed_index % 2 == 0 {
                sum += uncompressed_index * file_id;
            } else {
                for index in 0..reverse.len() {
                    let (_, file_id, number) = reverse[index];
                    if number as usize <= num_indices {
                        sum += uncompressed_index * file_id;
                        reverse[index] = (u32::MAX, usize::MAX, u32::MAX);
                    }
                    break ;
                }
            }
        }
        base_index += num_indices; 
    }


    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2858";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
