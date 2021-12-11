pub mod puzzles;
use std::env;
use std::time::{Duration, Instant};

fn main() {

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let part = args[2].parse::<usize>().unwrap();
    let now = Instant::now();

    match day {
        1 => puzzles::day1::run(part).unwrap(),
        2 => puzzles::day2::run(part).unwrap(),
        3 => puzzles::day3::run(part).unwrap(),
        4 => puzzles::day4::run(part).unwrap(),
        5 => puzzles::day5::run(part).unwrap(),
        6 => puzzles::day6::run(part).unwrap(),
        7 => puzzles::day7::run(part).unwrap(),
        8 => puzzles::day8::run(part).unwrap(),
        9 => puzzles::day9::run(part).unwrap(),
        10 => puzzles::day10::run(part).unwrap(),
        11 => puzzles::day11::run(part).unwrap(),
        _ => panic!("day {} not done", day)
    };
    println!("took: {}ms", now.elapsed().as_millis());

}
