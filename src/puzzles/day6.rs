use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_file(file: &File) -> Vec<u32> {
    let line = io::BufReader::new(file).lines().map(|x| x.unwrap()).next().unwrap();
    line.split(",").map(|c| c.parse::<u32>().unwrap()).collect()
}

// Used for part1. Iterative approach
fn simulate(mut fish: Vec<u32>, days: u32) -> usize {
    for day in 0..days {
        let mut new_fish = Vec::new();
        for f in &mut fish {
            match *f {
                0 => {
                    *f = 6;
                    new_fish.push(8);
                },
                _ => {
                    *f -= 1;
                }
            }
            *f -= 0;
        }
        fish.extend_from_slice(&new_fish);
        println!("day: {} fish: {}", day, fish.len());
    }
    fish.len()
}

// Used for part2. Dynamic programming approach
fn new_fish(fish: u32, days: u32, cache: &mut HashMap<(u32, u32), usize>) -> usize {
    // base case: 1 fish is 1 fish on the last day
    if days == 0 {return 1}
    // If we've already simulated a fish of age fish for days, get that answer
    match cache.get(&(fish, days)) {
        Some(f) => return *f,
        None => {}
    };
    // Otherwise simulate the fish
    let f = match fish {
        0 => new_fish(8, days-1, cache) + new_fish(6, days-1, cache),
        x => new_fish(x-1, days-1, cache)
    };
    // Store result for fish simulated for days
    cache.insert((fish, days), f);
    f
}

fn simulate_dyn(fish: Vec<u32>, days: u32) -> usize {
    let mut cache = HashMap::new();
    fish.iter()
        .map(|f| new_fish(*f, days, &mut cache))
        .sum()
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day6.txt")?;
    let fish = read_file(&file);

    match part {
        1 => {
            println!("fish: {}", simulate_dyn(fish, 80));
        },
        2 => {
            println!("fish: {}", simulate_dyn(fish, 256));
        },
        _ => panic!("")
    }
    Ok(())
}