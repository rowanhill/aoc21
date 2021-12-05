use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut horizontal = 0;
    let mut depth1 = 0;
    let mut aim = 0;
    let mut depth2 = 0;
    for line in reader.lines() {
        let line = line.expect("Could not unrwap line");
        match line {
            _ if line.starts_with("forward ") => {
                let num = line[8..].parse::<u32>().expect(&format!("Could not parse {}", line));
                horizontal += num;
                depth2 += num * aim;
            },
            _ if line.starts_with("down ") => {
                let num = line[5..].parse::<u32>().expect(&format!("Could not parse {}", line));
                depth1 += num;
                aim += num;
            },
            _ if line.starts_with("up ") => {
                let num = line[3..].parse::<u32>().expect(&format!("Could not parse {}", line));
                depth1 -= num;
                aim -= num;
            },
            _ => panic!("Unexpected input: {}", line)
        }
    }

    println!("Part 1: {}", horizontal * depth1);
    println!("Part 2: {}", horizontal * depth2);
}
