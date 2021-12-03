use std::fs::File;
use std::io::{self, BufRead};

pub fn find_destination(file: &File) -> i32{
    let reader = io::BufReader::new(file);

    let mut depth: i32 = 0;
    let mut x: i32 = 0;

    reader
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let dir = words[0];
            let count = words[1].parse::<i32>().unwrap();
            match dir {
                "forward" => x += count,
                "up" => depth -= count,
                "down" => depth += count,
                _ => panic!("unknown direction {}", dir)
            };
        });

    depth*x
}

pub fn find_destination_p2(file: &File) -> i32{
    let reader = io::BufReader::new(file);

    let mut depth: i32 = 0;
    let mut x: i32 = 0;
    let mut aim: i32 = 0;

    reader
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let dir = words[0];
            let count = words[1].parse::<i32>().unwrap();
            match dir {
                "forward" => {
                    x += count;
                    depth += aim*count;
                },
                "up" => aim -= count,
                "down" => aim += count,
                _ => panic!("unknown direction {}", dir)
            };
        });

    depth*x
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day2.txt")?;
    match part {
        1 => println!("destination: {}", find_destination(&file)),
        2 => println!("destination: {}", find_destination_p2(&file)),
        _ => panic!("")
    }
    Ok(())
}