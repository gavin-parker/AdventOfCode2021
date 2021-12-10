use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;


lazy_static! {
    static ref CLOSE_TO_OPEN: HashMap<char, char> = {
        HashMap::from([
            ('}', '{'),
            (')', '('),
            (']', '['),
            ('>', '<')
        ])
    };
    static ref SCORE: HashMap<char, usize> = {
        HashMap::from([
            ('}', 1197),
            (')', 3),
            (']', 57),
            ('>', 25137)
        ])
    };
}

fn find_line_error_score(line: &str) -> usize {
    let mut stack = Vec::new();
    for c in line.chars(){
        match c {
            '{'|'('|'['|'<' => stack.push(c),
            _ => {
                let close = c;
                let exp = stack.pop();
                match exp {
                    None => return 0,
                    Some(open) =>  {
                        if *CLOSE_TO_OPEN.get(&close).unwrap() != open {
                            return *SCORE.get(&close).unwrap();
                        }
                    }
                }
            }

        }
    }
    0
}

fn complete_score(stack: Vec<char>) -> usize
{
    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!()
        }
    }
    score
}

fn find_line_complete_score(line: &str) -> usize {
    let mut stack = Vec::new();
    for c in line.chars(){
        match c {
            '{'|'('|'['|'<' => stack.push(c),
            _ => {
                let close = c;
                let exp = stack.pop();
                match exp {
                    None => return 0,
                    Some(open) =>  {
                        if *CLOSE_TO_OPEN.get(&close).unwrap() != open {
                            return 0;
                        }
                    }
                }
            }

        }
    }
    complete_score(stack)
}

fn find_score(file: &File, f: fn(&str)-> usize) -> usize {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|line| f(&line))
        .sum()
}

fn find_score_p2(file: &File, f: fn(&str)-> usize) -> usize {
    let mut scores: Vec<usize> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|line| f(&line))
        .filter(|s| *s > 0)
        .collect();
    scores.sort();
    let mid = scores.len() / 2;
    scores[mid]
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day10.txt")?;
    match part {
        1 => println!("score: {}", find_score(&file, find_line_error_score)),
        2 => println!("score: {}", find_score_p2(&file, find_line_complete_score)),
        _ => panic!("")
    }
    Ok(())
}