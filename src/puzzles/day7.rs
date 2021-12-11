use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use lazy_static::lazy_static;

fn read_file(file: &File) -> Vec<i32> {
    let line = io::BufReader::new(file).lines().map(|x| x.unwrap()).next().unwrap();
    line.split(",").map(|c| c.parse::<i32>().unwrap()).collect()
}

fn cost_p1(crab_pos: i32, crab_dest: i32) -> i32 {
    (crab_pos - crab_dest).abs()
}

fn cost_p2(crab_pos: i32, crab_dest: i32) -> i32 {
    let diff = (crab_pos - crab_dest).abs();
    diff + (diff+1) / 2
}

fn find_cheapest_pos(crabs: Vec<i32>, cost: fn(i32, i32) -> i32) -> i32 {
    let min = *crabs.iter()
        .min().unwrap() as i32;
    let max = *crabs.iter()
        .max().unwrap() as i32;

    (min..=max)
        .map(|i| {
            crabs.iter()
                .map(|x|cost(i, *x))
                .sum()
        })
        .min()
        .unwrap()
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day7.txt")?;
    let crabs = read_file(&file);

    match part {
        1 => {
            println!("fish: {}", find_cheapest_pos(crabs, cost_p1));
        },
        2 => {
            println!("fish: {}", find_cheapest_pos(crabs, cost_p2));
        },
        _ => panic!("")
    }
    Ok(())
}