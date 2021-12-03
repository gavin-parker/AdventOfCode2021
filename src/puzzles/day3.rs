use std::fs::File;
use std::io::{self, BufRead};

const WIDTH: usize = 12;
pub fn read_file(file: &File) -> Vec::<u32>
{
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|line| isize::from_str_radix(line.unwrap().as_str(), 2).unwrap() as u32)
        .collect()
}

pub fn find_most_common_bits(nums: &Vec::<u32>) -> Vec::<u32>
{
    let mut total = 0;
    let mut totals = vec![0; WIDTH];

    nums.iter()
        .for_each(|num| {
            for i in 0..WIDTH{
                totals[i] += num >> ((WIDTH-1) - i) & 1;
            }
            total += 1;
        });

    let thresh = total/2 as u32;
    totals.iter()
        .map(|c| (c >= &thresh) as u32)
        .collect()
}

pub fn find_power(file: &File) -> u32{
    let nums = read_file(file);
    let mcb = find_most_common_bits(&nums);
    let width  = WIDTH;

    let gamma = mcb.iter()
        .enumerate()
        .fold(0, |x, (i, bit)| {
            x | (bit << ((width-1)-i))
        });

    let mask = (1<<WIDTH)-1;
    let epsilon = !gamma & mask;
    epsilon*gamma
}

pub fn keep_common(nums: Vec<u32>, bit: usize, most: bool) -> Vec<u32>
{
    let len = nums.len() as u32;
    let ones: u32 = nums.iter()
            .map(|x| x >> bit & 1)
            .sum();
    let zeros = len - ones;
    
    let mcb: u32 = match most{
        true => (ones >= zeros) as u32,
        false => !(ones >= zeros) as u32
    };
    nums.into_iter()
        .filter(|x| (x >> bit&1) == mcb )
        .collect()
}

pub fn find_oxygen(file: &File) -> u32 {
    let mut nums = read_file(file);

    for i in (0..WIDTH).rev(){
        nums = keep_common(nums, i, true);
        if nums.len() == 1{
            return nums[0];
        }
    }
    panic!("Didn't find answer");
}

pub fn find_co2(file: &File) -> u32 {
    let mut nums = read_file(file);

    for i in (0..WIDTH).rev(){
        nums = keep_common(nums, i, false);
        if nums.len() == 1{
            return nums[0];
        }
    }
    panic!("Didn't find answer");
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day3.txt")?;
    match part {
        1 => println!("power: {}", find_power(&file)),
        2 => println!("co2: {}", find_co2(&File::open("test_data/day3.txt")?)*find_oxygen(&File::open("test_data/day3.txt")?)),
        _ => panic!("")
    }
    Ok(())
}