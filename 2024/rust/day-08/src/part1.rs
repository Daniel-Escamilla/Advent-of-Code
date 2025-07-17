#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {

    let mut values: Vec<((usize, usize), char)> = vec![];
    let mut marked: Vec<(usize, usize)> = vec![];
    let size = input.lines().count();

    println!("{size}");

    let _chars: Vec<_> = input
    .lines()
    .enumerate()
    .map(|(y, line)| {
        line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_alphanumeric())
        .map(|(x, c)| {

            if values.iter().any(|((_, _), char)| *char == c) {
                for ((ey, ex), _) in 
                    values
                    .iter()
                    .filter(|((_, _), existence)| *existence == c) {


                    let fy = y as i32 - *ey as i32;
                    let fx = x as i32 - *ex as i32;

                    // Antinodos a ambos lados del par de antenas
                    let new_up_row = x as i32 + fx;
                    let new_up_col = y as i32 + fy;
                    let new_down_row = *ex as i32 - fx;
                    let new_down_col = *ey as i32 - fy;

                    if (0..size as i32).contains(&(new_up_col))
                    && (0..size as i32).contains(&(new_up_row))
                    && !marked.contains(&(new_up_col as usize, new_up_row as usize)) {
                        marked.push((new_up_col as usize, new_up_row as usize));
                    }
                    if (0..size as i32).contains(&new_down_col)
                    && (0..size as i32).contains(&new_down_row)
                    && !marked.contains(&(new_down_col as usize, new_down_row as usize)) {
                        marked.push((new_down_col as usize, new_down_row as usize));
                    }
                }
            } 
            values.push(((y, x), c))
        })
        .collect::<Vec<_>>()
    }).collect();

    let result = marked.len();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
