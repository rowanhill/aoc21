use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    let reader = BufReader::new(File::open(path)
        .expect("Cannot open input"));
    reader.lines()
}

fn is_mostly_one_at_index(lines: &Vec<&String>, index: usize) -> bool {
    lines.iter().map(|line| line.chars().nth(index).unwrap()).filter(|c| c == &'1').count() * 2 >= lines.len()
}

fn main() {
    const BITS: usize = 12;
    const FILEPATH: &str = "input";
    // const BITS: usize = 5;
    // const FILEPATH: &str = "example";

    let mut bit_counts = [0u32; BITS];
    let mut lines_count = 0;
    for line in get_lines(FILEPATH) {
        let line = line.expect("Could not read line");
        lines_count += 1;
        for (bit_index, char_val) in line.char_indices() {
            match char_val {
                '0' => bit_counts[bit_index] += 1,
                '1' => {},
                _ => panic!("Unexpected character {}", char_val)
            }
        }
    }

    let mut reversed_bit_counts = bit_counts.clone();
    reversed_bit_counts.reverse();
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    for (bit_index, one_count) in reversed_bit_counts.iter().enumerate() {
        let is_majority_ones = one_count * 2 >= lines_count;
        if is_majority_ones {
            gamma_rate += 2u32.pow(bit_index as u32);
        } else {
            epsilon_rate += 2u32.pow(bit_index as u32);
        }
    }

    println!("Part 1: {}", gamma_rate * epsilon_rate);
    
    let lines: Vec<String> = get_lines(FILEPATH).map(|l| l.unwrap()).collect();

    let mut oxy_candidates: Vec<&String> = lines.iter().collect();
    let mut index = 0;
    while oxy_candidates.len() > 1 {
        let is_one = is_mostly_one_at_index(&oxy_candidates, index);
        let target = if is_one { '1' } else { '0' };
        oxy_candidates = oxy_candidates.into_iter().filter(|line| line.chars().nth(index).unwrap() == target).collect();

        index += 1;
    }

    let mut co2_candidates: Vec<&String> = lines.iter().collect();
    index = 0;
    while co2_candidates.len() > 1 {
        let is_one = is_mostly_one_at_index(&co2_candidates, index);
        let target = if is_one { '0' } else { '1' };
        co2_candidates = co2_candidates.into_iter().filter(|line| line.chars().nth(index).unwrap() == target).collect();

        index += 1;
    }

    let oxy_gen = oxy_candidates.first().unwrap();
    let co2_scrub = co2_candidates.first().unwrap();
    let oxy_int = usize::from_str_radix(oxy_gen, 2).unwrap();
    let co2_int = usize::from_str_radix(co2_scrub, 2).unwrap();
    // println!("{} x {}", oxy_gen, co2_scrub);
    // println!("{} x {}", oxy_int, co2_int);
    println!("Part 2: {}", oxy_int * co2_int);
}
