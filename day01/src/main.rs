use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_increases(nums: &Vec<i32>) -> u32 {
    let mut increases = 0;
    let mut last = nums.first().unwrap();
    for num in nums {
        if num > last {
            increases += 1;
        }
        last = num;
    }
    increases
}

fn main() {
    let reader = BufReader::new(File::open("src/input")
        .expect("Cannot open input"));
    let nums: Vec<i32> = reader.lines().map(|l| {
        let line = l.expect("Could not read line");
        line.parse::<i32>().expect("Could not parse number")
    }).collect();

    println!("Part 1: {}", count_increases(&nums));

    let sums: Vec<i32> = nums.windows(3).map(|w| w.into_iter().sum::<i32>()).collect();
    println!("Part 2: {}", count_increases(&sums));
}
