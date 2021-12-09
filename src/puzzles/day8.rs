use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

struct Screen {
    inputs: Vec<String>,
    outputs: Vec<String>
}

fn read_file(file: &File) -> Vec<Screen> {
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut screens = Vec::new();
    for line in lines {
        let parts = line.split("|");
        let mut parts = parts
            .map(|p: &str| p.split_whitespace().map(|x| x.to_string()).collect());
        let left: Vec<String> = parts.next().unwrap();
        let right = parts.next().unwrap();
        screens.push(Screen{inputs: left, outputs: right})
    }
    screens
}

fn count_uniques(screens: &Vec<Screen>) -> usize {
    screens.into_iter()
        .map(|screen| {
            screen.outputs.iter()
                .filter(|combo| match combo.len() {
                    2|4|3|7 => true,
                    _ => false
                })
                .count()
        }).sum()
}

lazy_static! {
    static ref LEN_TO_NUMS: HashMap<usize, Vec<usize>> = {
        let mut m = HashMap::new();
        m.insert(6, vec![0, 6, 9]);
        m.insert(5, vec![2, 3, 5]);
        m.insert(2, vec![1]);
        m.insert(3, vec![7]);
        m.insert(4, vec![4]);
        m.insert(7, vec![8]);
        m
    };
    static ref NUM_TO_SEGS: HashMap<usize, Vec<u8>> = {
        let mut m = HashMap::new();
        m.insert(0, vec![0, 1, 2 ,4 ,5 ,6]);
        m.insert(1, vec![2, 5]);
        m.insert(2, vec![0, 2,3,4,6]);
        m.insert(3, vec![0,2,3,5,6]);
        m.insert(4, vec![1,2,3,5]);
        m.insert(5, vec![0,1,3,5,6]);
        m.insert(6, vec![0,1,3,4,5,6]);
        m.insert(7, vec![0,2,5]);
        m.insert(8, (0..7).collect());
        m.insert(9, vec![01,2,3,5,6]);
        m
    };
    static ref SEG_TO_NUMS: HashMap<u8, Vec<u8>> = {
        let mut m = HashMap::<u8, Vec<u8>>::new();
            for (num, segs) in NUM_TO_SEGS.iter() {
                for seg in segs {
                    let nums = m.entry(*seg as u8).or_insert(Vec::new());
                    nums.push(*num as u8);
                }
            }
        m
    };
}

fn find_positions(signal: char, inputs: &Vec<String>, known: &HashSet<u8>) -> Vec<u8> {
    let mut poss = (0..7).collect::<Vec<u8>>();
    poss.retain(|p| !known.contains(p));
    for combo in inputs.iter().filter(|combo| combo.contains(signal)) {
        let mut new_pos = HashSet::new();
        for num in LEN_TO_NUMS.get(&combo.len()).unwrap() {
            for seg in NUM_TO_SEGS.get(num).unwrap() {
                new_pos.insert(seg);
            }
        }
        poss.retain(|x| new_pos.contains(x));
    }
    return poss
}

fn find_all_positions(inputs: &Vec<String>, positions: &HashMap<char, u8>) -> Option<HashMap<char, u8>>
{
    let known = positions.iter().map(|(k, v)| *v).collect();

    let mut char_to_choices = HashMap::new();
    let mut new_positions = positions.clone();
    for c in "abcdefg".chars() {
        if new_positions.contains_key(&c) {
            continue;
        }
        let pos = find_positions(c, &inputs, &known);
        if pos.len() == 1{
            new_positions.insert(c, pos[0]);
        }
        char_to_choices.insert(c, pos);
    }
    
    if new_positions.len() == "abcdefg".len() {
        return Some(new_positions);
    }
    for (c, ps) in char_to_choices {
        for p in ps {
            let mut test_positions = new_positions.clone();
            test_positions.insert(c, p);
            match find_all_positions(inputs, &test_positions){
                Some(new_positions) => return Some(new_positions),
                None => {}
            };
        }
    }

    panic!("Fail");
}

fn read_output(charmap: &HashMap<char, u8>, words: &Vec<String>) -> usize {
    for word in words {
        // println!("");
        let mut nums: Vec<u8> = (0..=9).collect();
        for c in word.chars() {
            let seg = charmap.get(&c).unwrap();
            print!("{}", seg);
            // let allowed = SEG_TO_NUMS.get(&seg).unwrap();
            // nums.retain(|n| allowed.contains(n));
        }
        println!("");
        // print!
        // assert!(nums.len() == 1);
        // print!("{}", nums[0])
    }
    0
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day8.txt")?;
    let screens = read_file(&file);
    match part {
        1 => {
            println!("uniques: {}", count_uniques(&screens));
        },
        2 => {
            for screen in screens {
                let m = find_all_positions(&screen.inputs, &HashMap::new()).unwrap();
                println!("");
                read_output(&m, &screen.outputs);
            }
            println!("");
            // println!("ab: {}", poss[0]);
        },
        _ => panic!("")
    }
    Ok(())
}