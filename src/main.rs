pub mod puzzles;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let part = args[2].parse::<usize>().unwrap();

    match day {
        1 => puzzles::day1::run(part).unwrap(),
        2 => puzzles::day2::run(part).unwrap(),
        3 => puzzles::day3::run(part).unwrap(),
        4 => puzzles::day4::run(part).unwrap(),
        5 => puzzles::day5::run(part).unwrap(),
        6 => puzzles::day6::run(part).unwrap(),
        7 => puzzles::day7::run(part).unwrap(),
        _ => panic!("day {} not done", day)
    };

}
