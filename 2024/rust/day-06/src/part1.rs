pub fn suma(a: [i32; 2], b: [i32; 2]) -> [i32; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

pub fn compare(a: [i32; 2]) -> Option<[i32; 2]> {

    if a[0] == -1 || a[1] == -1 {
        return None;
    }
    Some(a)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {

    
    let mut chars: Vec<Vec<char>> = input
        .lines()
        .map(|line|  {
            line
            .chars()
            .collect()
        })
        .collect();

    let mut origin: [i32; 2] = chars
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
            .enumerate()
            .find_map(|(col, c)| 
            if *c == '^' {
                Some([row as i32, col as i32])
            } else {
                None
            })
        })
        .expect("Sin caracter ^");

    let dir: Vec<[i32; 2]>  =  vec![
        [-1, 0],
        [0, 1],
        [1, 0],
        [0, -1],
    ];

    let mut pos: usize = 0;
    let mut count: usize = 1;
    while compare(suma(origin, dir[pos])).is_some() {
        let tmp = suma(origin, dir[pos]);
        println!("{:?}", &tmp);
        if tmp[0] as usize == chars.len() || tmp[1] as usize == chars.len() {
            break ;
        }
        let x: usize = tmp[0] as usize;
        let y: usize = tmp[1] as usize;
        if chars[x][y] == '#' {
            pos = (pos + 1) % 4;
        } else {
            if chars[x][y] != 'X' && chars[x][y] != '^'{
                count += 1;
            }
            chars[x][y] = 'X';
            origin = tmp;
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".........
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......##..
";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
