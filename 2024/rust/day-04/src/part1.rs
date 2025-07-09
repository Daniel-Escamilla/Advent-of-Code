// use itertools::{rev, Itertools};
// use nom::InputIter;

pub fn contain_string(hay: &str, lines: &Vec<String>) -> usize {

    let mut number = 0;
    for line in lines {
        number += line.match_indices(hay).count();
    }
    number
}

pub fn up_to_down(grid: &[Vec<char>]) -> Vec<String>
{
    let mut lines: Vec<String> = vec![];

    let cols = grid[0].len();

    for i in 0..cols {
        let line: String = grid.iter().map(|row| row[i]).collect();
        lines.push(line);
    }

    lines
}

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
            // println!("{x},{y}");
            if y == lenght - 1 {
                break;
            }
            y += 1;
            x += 1;
        }
        // println!("\n");
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
            // println!("{x},{y}");
            if x == lenght - 1 {
                break;
            }
            y += 1;
            x += 1;
        }
        // println!("\n");
        positions.push(pos);
    }
    positions
}

pub fn diagonal(grid: &[Vec<char>]) -> Vec<String>
{
    let mut lines: Vec<String> = vec![];

    let positions = positions_to_diagonal(grid.len());
    let positions2 = positions_to_other_diagonal(grid.len());
    for pos in &positions {
        let line: String = pos.iter().map(|&MemPos{x, y}| grid[x][y]).collect();
        lines.push(line);
    }
    for pos in &positions2 {
        let line: String = pos.iter().map(|&MemPos{x, y}| grid[x][y]).collect();
        lines.push(line);
    }
    lines
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let hay: String = "XMAS".to_string();
    let reverse: String = hay.chars().rev().collect::<String>();

    let grid: Vec<Vec<char>> = _input.lines().map(|line|line.chars().collect()).collect();

    // let lines: Vec<String> = 
    let mut number = contain_string(&hay, &_input.lines().map(|line| line.to_string()).collect());

    number += contain_string(&reverse, &_input.lines().map(|line| line.to_string()).collect());

    let mut lines: Vec<String> = up_to_down(&grid);

    number += contain_string(&hay, &lines);

    number += contain_string(&reverse, &lines);
    
    lines = diagonal(&grid);
    
    number += contain_string(&hay, &lines);
    number += contain_string(&reverse, &lines);
    
    // for line in &lines {
    //     println!("{}", line);
    // }
    // println!("{}", number);


    Ok(number.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
