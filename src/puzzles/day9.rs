use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

fn read_file(file: &File) -> Vec<Vec<u8>> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        }).collect()
}

fn find_low_points(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut lows = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if c < row.len()-1 && *v >= row[c+1] {continue;}
            if c > 0 && *v >= row[c-1] {continue;}
            if r < grid.len()-1 && *v >= grid[r+1][c] {continue;}
            if r > 0 && *v >= grid[r-1][c] {continue;}
            lows.push((r, c));
        }
    }
    lows
}

fn find_score(grid: &Vec<Vec<u8>>) -> usize {
    find_low_points(&grid)
        .iter()
        .map(|(r, c)| grid[*r][*c])
        .map(|h| h as usize+1)
        .sum()
}

fn grow_basin(grid: &Vec<Vec<u8>>, coord: (usize, usize), basin: &mut HashSet<(usize, usize)>){
    let (r, c) = coord;
    if r < grid.len()-1 && grid[r+1][c] != 9{
        basin.insert((r+1, c));
    }
    if r > 0 && grid[r-1][c] != 9{
        basin.insert((r-1, c));
    }
    if c < grid[r].len()-1 && grid[r][c+1] != 9{
        basin.insert((r, c+1));
    }
    if c > 0 && grid[r][c-1] != 9{
        basin.insert((r, c-1));
    }
}

fn get_basin_size(grid: &Vec<Vec<u8>>, low: (usize, usize)) -> usize {
    let mut basin = HashSet::new();
    basin.insert(low);
    let mut sz = basin.len();
    while true {
        let cs = basin.clone();
        for c in cs {
            grow_basin(&grid, c, &mut basin);
        }
        if basin.len() == sz {
            return sz;
        }
        sz = basin.len();
    }
    panic!("Cant grow basin");
}

fn find_basin_score(grid: &Vec<Vec<u8>>) -> usize {
    let mut sizes = find_low_points(&grid)
        .iter()
        .map(|p| get_basin_size(&grid, *p))
        .collect::<Vec<usize>>();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0]*sizes[1]*sizes[2]
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day9.txt")?;
    let grid = read_file(&file);
    match part {
        1 => println!("score: {}", find_score(&grid)),
        2 => println!("score: {}", find_basin_score(&grid)),
        _ => panic!("")
    }
    Ok(())
}