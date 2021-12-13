use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

enum Fold {
    Up(usize),
    Left(usize)
}
type Grid = HashSet<(usize, usize)>;

fn read_file(file: &File) -> (Grid, Vec<(Fold)>) {
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());
    let mut points = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() { break;}
        let mut vals = line.split(",");
        let x = vals.next().unwrap().parse::<usize>().unwrap();
        let y = vals.next().unwrap().parse::<usize>().unwrap();
        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
        points.insert((x, y));
    }
    let mut folds = Vec::new();
    for line in lines {
        let mut parts = line.split("=");
        let p = parts.next().unwrap();
        let dir = p.chars().nth(p.len()-1).unwrap();
        let p = parts.next().unwrap();
        let c = p.parse::<usize>().unwrap();
        folds.push(match dir {
            'x' => Fold::Left(c),
            'y' => Fold::Up(c),
            _ => panic!("parse errr")
        });
    }
    (points, folds)
}

fn show(grid: &Grid){
    let (max_x, max_y) = grid.iter().fold((0, 0), |max, (x, y)| (std::cmp::max(max.0, *x), std::cmp::max(max.1, *y)));
    let grid_set: HashSet<(usize, usize)> = HashSet::from_iter(grid.iter().cloned());
    for y in 0..=max_y {
        println!();
        for x in 0..=max_x {
            print!("{}", match grid_set.contains(&(x, y)){
                true => '#',
                false => '.'
            });
        }
    }
    println!();
}

fn fold_paper(grid: Grid, fold: &Fold) -> Grid {
    match fold {
        Fold::Up(fold_line) => {
            grid.into_iter()
                .map(|(x, y)| (x, std::cmp::min(y, 2*fold_line-y)))
                .collect()
        },
        Fold::Left(fold_line) => {
            grid.into_iter()
                .map(|(x, y)| (std::cmp::min(2*fold_line-x, x), y))
                .collect()
        }
        _ => panic!("not done yet")
    }
}

fn count(grid: &Grid) -> usize {
    grid.len()
}

pub fn run(part: usize) -> io::Result<()> {
    let (grid, folds) = read_file(&File::open("test_data/day13.txt")?);

    match part {
        1 => {
            let folded = fold_paper(grid, &folds[0]);
            let c = count(&folded);
            println!("Count: {}", c);
        },
        2 => {
            let res = folds.into_iter().fold(grid, |g,f| {
                fold_paper(g, &f)
            });
            show(&res);
            let c = count(&res);
            println!("Count: {}", c);
        },
        _ => panic!("")
    };
    Ok(())
}