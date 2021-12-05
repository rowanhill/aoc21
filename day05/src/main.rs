use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let num_re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let coord_pairs: Vec<_> = reader.lines().map(|line| {
        let line = line.expect("Could not unwrap line");
        let captures = num_re.captures(&line).expect("Could not parse line");
        (
            (
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
            ),
            (
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<usize>().unwrap()
            )
        )
    }).collect();

    let non_diag_pairs: Vec<_> = coord_pairs.iter()
        .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
        .collect();
    let diag_pairs: Vec<_> = coord_pairs.iter()
        .filter(|(a, b)| a.0 != b.0 && a.1 != b.1)
        .collect();

    let mut vent_counts = [[0u8; 1000]; 1000];

    record_vents(&non_diag_pairs, &mut vent_counts);
    println!("Part 1: {}", count_multiple_vents(&mut vent_counts));

    record_vents(&diag_pairs, &mut vent_counts);
    println!("Part 2: {}", count_multiple_vents(&mut vent_counts));

    // print_field(&vent_counts);
}

fn record_vents(pairs: &Vec<&((usize, usize), (usize, usize))>, vent_counts: &mut [[u8; 1000]; 1000]) {
    for (from, to) in pairs {
        let dx: i32 = if to.0 > from.0 { 1 } else if to.0 < from.0 { -1 } else { 0 };
        let dy: i32 = if to.1 > from.1 { 1 } else if to.1 < from.1 { -1 } else { 0 };
        // println!("({},{}) -> ({},{}) [({}, {})]", from.0, from.1, to.0, to.1, dx, dy);

        let mut x = from.0;
        let mut y = from.1;
        loop {
            // println!(" > {}x{}", x, y);
            vent_counts[y][x] += 1;
            if x == to.0 && y == to.1 {
                break;
            }
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;
        }
    }
}

fn count_multiple_vents(vent_counts: &mut [[u8; 1000]; 1000]) -> usize {
    vent_counts.iter()
        .map(|row| {
            row.iter().filter(|n| n >= &&2u8).count()
        })
        .sum()
}

fn print_field(vent_counts: &[[u8; 1000]; 1000]) {
    for row in vent_counts {
        for count in row {
            if count == &0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
}
