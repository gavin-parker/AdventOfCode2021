use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::cmp;
use regex::Regex;
use lazy_static::lazy_static;

struct Vec2D{
    x: u32,
    y: u32
}

struct Line{
    a: Vec2D,
    b: Vec2D
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([\d]+),([\d]+) -> ([\d]+)+,([\d]+)").unwrap();
        }
        let cap = RE.captures_iter(text).next().unwrap();
        let x1 = cap[1].parse::<u32>().unwrap();
        let y1 = cap[2].parse::<u32>().unwrap();
        let x2 = cap[3].parse::<u32>().unwrap();
        let y2 = cap[4].parse::<u32>().unwrap();
        // make sure lines go left->right
        match x1 < x2 {
            true => Ok(Line{
                a: Vec2D{x: x1, y:y1},
                b: Vec2D{x: x2, y: y2}
            }),
            false => Ok(Line{
                a: Vec2D{x: x2, y: y2},
                b: Vec2D{x: x1, y:y1}
            })
        }
    }
}

fn read_lines(file: &File) -> Vec<Line> {
    let str_lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    str_lines
        .map(|line| Line::from_str(line.as_str()).unwrap())
        .collect()
}

fn build_map(lines: &Vec<Line>) -> Vec<Vec<u32>> {
    let max_x = lines.iter()
        .map(|line| cmp::max(line.a.x, line.b.x))
        .max()
        .unwrap();
    let max_y = lines.iter()
        .map(|line| cmp::max(line.a.y, line.b.y))
        .max()
        .unwrap();

    let map: Vec<Vec<u32>> = (0..(max_y+1))
        .map(|_| vec![0; (max_x+1) as usize])
        .collect();

    map
}

fn build_map_basic(map: &mut Vec<Vec<u32>>, lines: &Vec<Line>) {

    for line in lines {
        if line.a.x == line.b.x {
            let y1 = cmp::min(line.a.y, line.b.y);
            let y2 = cmp::max(line.a.y, line.b.y);
            for i in y1..=y2 {
                map[i as usize][line.a.x as usize] += 1;
            }
        }
        if line.a.y == line.b.y {
            let x1 = cmp::min(line.a.x, line.b.x);
            let x2 = cmp::max(line.a.x, line.b.x);
            for i in x1..=x2 {
                map[line.a.y as usize][i as usize] += 1;
            }
        }
    }
}

fn build_map_diag(map: &mut Vec<Vec<u32>>, lines: &Vec<Line>) {
    for line in lines {
        let y1 = cmp::min(line.a.y, line.b.y);
        let y2 = cmp::max(line.a.y, line.b.y);
        let x1 = cmp::min(line.a.x, line.b.x);
        let x2 = cmp::max(line.a.x, line.b.x);
        if line.a.x == line.b.x {
            for i in y1..=y2 {
                map[i as usize][line.a.x as usize] += 1;
            }
        }
        else if line.a.y == line.b.y {
            for i in x1..=x2 {
                map[line.a.y as usize][i as usize] += 1;
            }
        }
        else {
            if line.a.y > line.b.y {
                for (i, j) in (line.a.x..=line.b.x).zip((line.b.y..=line.a.y).rev()){
                    map[j as usize][i as usize] += 1;
                }
            }else {
                for (i, j) in (line.a.x..=line.b.x).zip(line.a.y..=line.b.y){
                    map[j as usize][i as usize] += 1;
                }
            }

        }
    }

}

pub fn score(map: &Vec<Vec<u32>> ) -> u32 {
    map.into_iter()
        .flatten()
        .filter(|count| *count > &1)
        .count() as u32
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day5.txt")?;
    let lines = read_lines(&file);
    let mut map = build_map(&lines);
    match part {
        1 => {
            build_map_basic(&mut map, &lines);
            println!("score: {}", score(&map));
        },
        2 => {
            build_map_diag(&mut map, &lines);
            println!("score: {}", score(&map));
        },
        _ => panic!("")
    }
    Ok(())
}