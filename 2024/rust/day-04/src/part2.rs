// use itertools::{rev, Itertools};
// use nom::InputIter;

use std::collections::HashMap;
// use nom::lib::std::collections::HashMap;









pub fn contain_string(hay: &str, reverse: &str, lines: &Vec<String>, pos1: &[Vec<MemPos>], pos2: &[Vec<MemPos>]) -> usize {
    let mut location_normal: Vec<(Option<usize>, usize)> = vec![];
    let mut location_reverse: Vec<(Option<usize>, usize)> = vec![];

    let mut i: usize = 0;
    for line in lines
    {
        let mut found: bool = false;
        for (l,_) in line.match_indices(hay) {
            found = true;
            location_normal.push((Some(l), i));
        }
        if !found {
            location_normal.push((None, i));
        }
        i += 1;
    }
    i = 0;
    for line in lines
    {
        let mut found: bool = false;
        for (l, _) in line.match_indices(reverse) {
            found = true;
            location_reverse.push((Some(l), i));
        }
        if !found {
            location_reverse.push((None, i));
        }
        i += 1;
    }

    let mut status: Vec<MemPos> = vec![];

    for (loc, num) in location_normal {
        if let Some(index) = loc {
            if num < pos1.len() {
                status.push(MemPos{x: pos1[num][index+1].x, y: pos1[num][index+1].y});
            } else {
                let n = num - pos1.len();
                status.push(MemPos{x: pos2[n][index+1].y, y: pos2[n][index+1].x});
            }
        }
    }

    for (loc, num) in location_reverse {
        if let Some(index) = loc {
            if num < pos1.len() {
                status.push(MemPos{x: pos1[num][index+1].x, y: pos1[num][index+1].y});
            } else {
                let n = num - pos1.len();
                status.push(MemPos{x: pos2[n][index+1].y, y: pos2[n][index+1].x});
            }
        }
    }

    let mut counts: HashMap<MemPos, usize> = HashMap::new();

    for pos in &status {
        *counts.entry(pos.clone()).or_insert(0) += 1;
    }

    counts.iter().filter(|(_, &count)| count > 1).count()


    // number
}

#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct MemPos {
    x: usize,
    y: usize,
}

pub fn positions_to_diagonal(lenght: usize) -> Vec<Vec<MemPos>>
{
    let mut positions: Vec<Vec<MemPos>> = vec![];

    for i in 0..lenght - 1 {
        let mut x: usize = 0;
        let mut y: usize = i;
        let mut pos: Vec<MemPos> = vec![];
        loop
        {
            pos.push(MemPos { x, y });
            if y == 0 {
                break;
            }
            y -= 1;
            x += 1;
        }
        positions.push(pos);
    }
    for i in 0..lenght {
        let mut x: usize = lenght - 1;
        let mut y: usize = i;
        let mut pos: Vec<MemPos> = vec![];
        loop
        {
            pos.push(MemPos { x, y });
            if y == lenght - 1 {
                break;
            }
            y += 1;
            x -= 1;
        }
        positions.push(pos);
    }
    positions
}

pub fn positions_to_other_diagonal(lenght: usize) -> Vec<Vec<MemPos>>
{
    let mut positions: Vec<Vec<MemPos>> = vec![];

    let mut i: usize = lenght - 1;

    while i != 0 {
        let mut x: usize = 0;
        let mut y: usize = i;
        let mut pos: Vec<MemPos> = vec![];
        loop
        {
            pos.push(MemPos { x, y });
            if y == lenght - 1 {
                break;
            }
            y += 1;
            x += 1;
        }
        i -= 1;
        positions.push(pos);
    }
    for i in 0..lenght {
        let mut x: usize = i;
        let mut y: usize = 0;
        let mut pos: Vec<MemPos> = vec![];
        loop
        {
            pos.push(MemPos { x, y });
            if x == lenght - 1 {
                break;
            }
            y += 1;
            x += 1;
        }
        positions.push(pos);
    }
    positions
}

pub fn diagonal(grid: &[Vec<char>], pos1: &[Vec<MemPos>], pos2: &[Vec<MemPos>]) -> Vec<String>
{
    let mut lines: Vec<String> = vec![];


    for pos in pos1 {
        let line: String = pos.iter().map(|&MemPos{x, y}| grid[y][x]).collect();
        lines.push(line);
    }
    for pos in pos2 {
        let line: String = pos.iter().map(|&MemPos{x, y}| grid[x][y]).collect();
        lines.push(line);
    }
    lines
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let hay: String = "MAS".to_string();
    let reverse: String = hay.chars().rev().collect::<String>();

    let grid: Vec<Vec<char>> = _input.lines().map(|line|line.chars().collect()).collect();

    let pos1 = positions_to_diagonal(grid.len());
    let pos2 = positions_to_other_diagonal(grid.len());

    let lines = diagonal(&grid, &pos1, &pos2);
    
    let number: usize = contain_string(&hay, &reverse, &lines, &pos1, &pos2);

    Ok(number.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

//             let input =
// "M.M..
// .A...
// S.S..";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
