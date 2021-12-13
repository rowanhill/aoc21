use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Fold {
    X(u32),
    Y(u32)
}

fn main() {
    let reader = BufReader::new(File::open("dots").expect("Could not read file"));
    let mut dots = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        let (xstr, ystr) = line.split_once(',').expect("Could not split coord");
        let x: u32 = xstr.parse().expect("Could not parse x");
        let y: u32 = ystr.parse().expect("Could not parse y");
        (x, y)
    }).collect::<Vec<_>>();

    let folds = BufReader::new(File::open("folds").expect("Could not read file"))
        .lines().map(|line| {
        let line = line.expect("Could not read line");
        let axis = &line[11..12];
        let numstr = &line[13..];
        let num: u32 = numstr.parse().expect("Could not parse fold number");
        match axis {
            "x" => Fold::X(num),
            "y" => Fold::Y(num),
            _ => panic!("Unknown axis for fold"),
        }
    }).collect::<Vec<_>>();

    let mut is_first = true;
    for fold in folds {
        dots = dots.iter_mut().map(|(x, y)| {
            match fold {
                Fold::X(fx) => if *x > fx { *x = fx - (*x - fx); }
                Fold::Y(fy) => if *y > fy { *y = fy - (*y - fy); }
            }
            (*x, *y)
        }).collect();
        if is_first {
            is_first = false;

            let set: HashSet<&(u32, u32), RandomState> = HashSet::from_iter(dots.iter());
            println!("Part 1: {} (unique after one fold)", set.len());
        }
    }

    println!();
    println!("Part 2:");
    let max_x = dots.iter().map(|(x, _)| x).max().expect("Could not find max x");
    let max_y = dots.iter().map(|(_, y)| y).max().expect("Could not find max y");
    let set: HashSet<&(u32, u32), RandomState> = HashSet::from_iter(dots.iter());
    for y in 0..(max_y+1) {
        for x in 0..(max_x+1) {
            if set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
