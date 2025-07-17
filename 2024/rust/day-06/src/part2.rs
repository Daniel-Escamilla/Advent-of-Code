pub fn suma(a: [i32; 2], b: [i32; 2]) -> [i32; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

pub fn check_neg(a: [i32; 2]) -> Option<[i32; 2]> {

    if a[0] == -1 || a[1] == -1 {
        return None;
    }
    Some(a)
}

pub fn compare(a: [i32; 2], b: [i32; 2]) -> Option<[i32; 2]> {
    if a[0] == b[0] && a[1] == b[1] {
        return Some(a);
    }
    None
}

fn it_is_inside(next: [i32; 2], chars: &[Vec<(char, usize, usize)>]) -> bool {
    next[0] >= 0 && next[1] >= 0 &&
    (next[0] as usize) < chars.len() &&
    (next[1] as usize) < chars[0].len()
} 

pub fn posible_loop(chars: &[Vec<(char, usize, usize)>], origin: &[i32; 2], dir: &[[i32; 2]], pos: &usize) -> bool
{
    let mut matrix: Vec<Vec<(char, usize, usize)>> = chars.to_vec();
    let mut start = *origin;
    let compass = dir;
    let mut direction = *pos;

    let mut corners: Vec<[[usize; 2]; 2]> = vec![];

    loop {
        let next = suma(start, compass[direction]);
        if !it_is_inside(next, &matrix) {
            return false;
        }
        let x: usize = next[0] as usize;
        let y: usize = next[1] as usize;
        if matrix[x][y].0 == '#' {
            let target: [[usize; 2]; 2] = [
                [start[0] as usize, start[1] as usize],
                [next[0] as usize, next[1] as usize],
            ];
            if !corners.is_empty() {
                let exist_loop = corners.iter().any(|corner: &[[usize; 2]; 2]| *corner == target);
                if exist_loop {
                    return false;
                }
            }
            corners.push(target);
            direction = (direction + 1) % 4;
        } else if next == *origin {
            break ;
        } else {
            matrix[x][y].0 = 'X';
            start = next;
        }
    }
    true
}

pub fn process(input: &str) -> miette::Result<String>  {
    let mut chars: Vec<Vec<(char, usize, usize)>> = input
    .lines()
    .map(|line| {
        line
            .chars()
            .map(|ch| (ch, 5usize, 0usize)) 
            .collect()
    })
    .collect();

    let mut origin: [i32; 2] = chars
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
            .enumerate()
            .find_map(|(col, &(c, _extra, _extra2))| 
            if c == '^' {
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
    let mut count: usize = 0;
    chars[origin[0] as usize][origin[1] as usize].2 = 1;

    loop {
        let next = suma(origin, dir[pos]);
        if !it_is_inside(next, &chars) {
            break ;
        }
        let x: usize = next[0] as usize;
        let y: usize = next[1] as usize;
        if chars[x][y].0 == '#' {
            pos = (pos + 1) % 4;
        }
        else if chars[x][y].2 == 1 {
            origin = next;
            continue ;
        }
        else {
            
            let x_o: usize = origin[0] as usize;
            let y_o: usize = origin[1] as usize;
            chars[x][y].0 = '#';
            chars[x_o][y_o].0 = 'I';
            
            chars[x][y].2 = 1;
            if posible_loop(&chars, &origin, &dir, &pos) {
                count += 1;
            }
            chars[x_o][y_o].0 = '.';
            chars[x][y].0 = '.';
            origin = next;
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..........
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
