#![feature(array_windows)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input").expect("Could not read file"));
    let rules = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        let (pair, insert) = line.split_once(" -> ").expect("Could not parse rule");

        let mut bytes_iter = pair.chars();
        let pair = [bytes_iter.next().unwrap(), bytes_iter.next().unwrap()];

        (pair, insert.chars().next().expect("Could not get insert char"))
    }).collect::<HashMap<_, _>>();

    let template = String::from("CPSSSFCFOFVFNVPKBFVN");
    // let template = String::from("NNCB");

    let mut pair_counts: HashMap<[char; 2], u64> = template.chars()
        .collect::<Vec<_>>()
        .array_windows()
        .fold(HashMap::new(), |mut acc, pair| {
            *acc.entry(*pair).or_default() += 1u64;
            acc
        });

    for i in 0..40 {
        let mut new_pair_counts: HashMap<[char; 2], u64> = HashMap::new();
        for (pair, count) in pair_counts {
            if let Some(&insert) = rules.get(&pair) {
                *new_pair_counts.entry([pair[0], insert]).or_default() += count;
                *new_pair_counts.entry([insert, pair[1]]).or_default() += count;
            } else {
                *new_pair_counts.entry(pair).or_default() += count;
            }
        }

        pair_counts = new_pair_counts;

        if i == 9 {
            println!("Part 1: {}", max_minus_min_chars(&pair_counts));
        }
    }

    println!("Part 2: {}", max_minus_min_chars(&pair_counts));
}

fn max_minus_min_chars(pair_counts: &HashMap<[char; 2], u64>) -> u64 {
    let char_counts: HashMap<char, u64> = pair_counts.iter()
        .fold(HashMap::new(), |mut acc, (pair, count)| {
            *acc.entry(pair[0]).or_default() += count;
            *acc.entry(pair[1]).or_default() += count;
            acc
        });

    let max = char_counts.iter().max_by_key(|(_, v)| *v).unwrap();
    let min = char_counts.iter().min_by_key(|(_, v)| *v).unwrap();
    let max_count = if max.1 % 2 == 0 { max.1 / 2} else { (max.1 + 1) / 2 };
    let min_count = if min.1 % 2 == 0 { min.1 / 2} else { (min.1 + 1) / 2 };
    // println!("{:?}, {:?}", (max.0, max_count), (min.0, min_count));
    max_count - min_count
}
