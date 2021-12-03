pub mod puzzles;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    let part = args[2].parse::<usize>().unwrap();

    match day {
        1 => puzzles::day1::run(part).unwrap(),
        2 => puzzles::day2::run(part).unwrap(),
        _ => panic!("day {} not done", day)
    };

}
