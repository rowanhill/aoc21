use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input").expect("Could not read file"));

    let map: Vec<Vec<u32>> = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        line.chars().map(|c| c.to_digit(10).expect("Could not parse digit")).collect()
    }).collect();

    let mut risk = 0;
    let mut lows = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let left = row.get((x as i32 - 1) as usize).unwrap_or_else(|| &100);
            let right = row.get(x + 1).unwrap_or_else(|| &100);
            let top_row = map.get((y as i32 - 1) as usize);
            let top = match top_row {
                Some(r) => r.get(x).unwrap(),
                None => &100
            };
            let bottom_row = map.get(y + 1);
            let bottom = match bottom_row {
                Some(r) => r.get(x).unwrap(),
                None => &100
            };

            if height < left && height < right && height < top && height < bottom {
                risk += height + 1;
                lows.push((x, y));
            }
        }
    }
    println!("Part 1: {}", risk);

    let mut basin_sizes = lows.iter().map(|(x, y)| {
        let mut basin_points = HashSet::new();
        let mut visited =  HashSet::new();
        visited.insert((*x, *y));
        let mut queue = vec![(*x, *y)];

        let should_explore = |visited: &HashSet<(usize, usize)>, x: &usize, y: &usize| {
            if visited.contains(&(*x, *y)) {
                return false
            }
            match get_height(&map, x, y) {
                Some(h) => h != &9,
                None => false
            }
        };

        // println!("Basin ({}, {})", x, y);
        while let Some((next_x, next_y)) = queue.pop() {
            basin_points.insert((next_x, next_y));
            // println!(" > ({}, {})", next_x, next_y);

            let left = ((next_x as i32) - 1) as usize;
            if should_explore(&visited, &left, &next_y) {
                queue.push((left, next_y));
                visited.insert((left, next_y));
            }

            let right = next_x + 1;
            if should_explore(&visited, &right, &next_y) {
                queue.push((right, next_y));
                visited.insert((right, next_y));
            }

            let top = ((next_y as i32) - 1) as usize;
            if should_explore(&visited, &next_x, &top) {
                queue.push((next_x, top));
                visited.insert((next_x, top));
            }

            let bottom = next_y + 1;
            if should_explore(&visited, &next_x, &bottom) {
                queue.push((next_x, bottom));
                visited.insert((next_x, bottom));
            }
        }
        basin_points.len() as u32
    }).collect::<Vec<u32>>();

    basin_sizes.sort();
    basin_sizes.reverse();
    let top_three_multiplied = basin_sizes.iter()
        .take(3)
        .map(|x| *x)
        .reduce(|acum, item| acum * item)
        .expect("Could not multiply top three");
    println!("Part 2: {}", top_three_multiplied);
}

fn get_height<'a>(map: &'a Vec<Vec<u32>>, x: &usize, y: &usize) -> Option<&'a u32> {
    let row = map.get(*y);
    match row {
        Some(r) => {
            r.get(*x)
        },
        None => None
    }
}
