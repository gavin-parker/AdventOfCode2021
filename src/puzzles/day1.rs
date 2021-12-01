use std::fs::File;
use std::io::{self, BufRead};

pub fn count_increases(file: &File) -> usize{
    let reader = io::BufReader::new(file);

    let measurements: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let diffs: Vec<i32> = measurements
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    diffs.into_iter()
        .filter(|diff| diff.is_positive())
        .count()
}

pub fn count_increases_window(file: &File) -> usize{
    let reader = io::BufReader::new(file);

    let measurements: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    // Sum groups of 3
    let window_totals: Vec<i32> = measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    
    // Find diffs between group totals
    let diffs: Vec<i32> = window_totals
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    diffs.into_iter()
        .filter(|diff| diff.is_positive())
        .count()
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day1.txt")?;
    match part {
        1 => println!("increases: {}", count_increases(&file)),
        2 => println!("increases: {}", count_increases_window(&file)),
        _ => panic!("")
    }
    Ok(())
}