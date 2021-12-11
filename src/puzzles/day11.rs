use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

type Grid = Vec<Vec<u32>>;
fn read_file(file: &File) -> Grid {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
        )
        .collect()
}

fn show(grid: &Grid) {
    for row in grid {
        println!();
        for val in row {
            let v = match val {
                1..=9 => val.to_string(),
                _ => "X".to_string()
            };
            print!("{}", v);
        }
    }
    println!();
}

fn flash_if(val: &mut u32){
    if *val == 0 { return;}
    *val += 1;
}

fn flash(pos: (usize, usize ), grid: &mut Grid, dim: (usize, usize)) {
    let (row, col) = pos;
    let (height, width) = dim;
    let (maxh, maxw) = (height-1, width-1);
    // row above
    if row > 0 && col > 0 { flash_if(&mut grid[row-1][col-1])}
    if row > 0 { flash_if(&mut grid[row-1][col])}
    if row > 0 && col < maxw { flash_if(&mut grid[row-1][col+1])}
    // row below
    if row < maxh && col > 0 { flash_if(&mut grid[row+1][col-1])}
    if row < maxh { flash_if(&mut grid[row+1][col])}
    if row < maxh && col < maxw { flash_if(&mut grid[row+1][col+1])}

    if col > 0  { flash_if(&mut grid[row][col-1])}
    if col < maxw { flash_if(&mut grid[row][col+1])}
    grid[row][col] = 0;
}

fn simulate(steps: usize, target: usize, mut grid: Grid) -> usize {
    println!("Steps left: {}", steps);
    show(&grid);
    let width = grid[0].len();
    let height = grid.len();
    for row in 0..height {
        for col in 0..width {
            grid[row][col] = grid[row][col]+1;
        }
    }
    let mut did_flash = true;
    let mut flashes = 0;
    while did_flash {
        did_flash = false;
        for row in 0..height {
            for col in 0..width {
                if grid[row][col] > 9 {
                    flash((row, col), &mut grid, (height, width));
                    did_flash = true;
                    flashes += 1;
                }
            }
        }
    }
    if flashes == 100 { println!("All flashed on step {}", steps)}
    if steps == target { return flashes;}
    flashes + simulate(steps+1, target, grid)
}

pub fn run(part: usize) -> io::Result<()> {
    let jellies = File::open("test_data/day11.txt")?;
    let grid = read_file(&jellies);
    match part {
        1 => println!("flashes: {}", simulate(1, 99, grid)),
        2 => println!("flashes: {}", simulate(1, 1000, grid)),
        _ => panic!("")
    }
    Ok(())
}