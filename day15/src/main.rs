#![feature(map_first_last)]
#![feature(entry_insert)]

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Node {
    Unvisited,
    Touched(u32),
    Visited(u32),
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Could not read file"));
    let risks_map = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        line.chars().map(|c| c.to_digit(10).expect("Could not parse digit")).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut nodes_map = risks_map.iter().map(|row| {
        row.iter().map(|_| Node::Unvisited).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    nodes_map[0][0] = Node::Touched(risks_map[0][0]);

    let mut curr_node_coords = Some((0usize, 0usize));

    let max_x = nodes_map[0].len() - 1;
    let max_y = nodes_map.len() - 1;

    while let Some((curr_x, curr_y)) = curr_node_coords {
        let curr_dist = if curr_x == 0 && curr_y == 0 {
            0
        } else {
            let curr_node = &nodes_map[curr_y][curr_x];
            match curr_node {
                Node::Touched(d) => *d,
                _ => panic!(),
            }
        };

        // println!("({}, {}): {}", curr_x, curr_y, curr_dist);

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let x = curr_x as i32 + dx;
            let y = curr_y as i32 + dy;

            if x >= 0 && x < risks_map[0].len() as i32 && y >= 0 && y < risks_map.len() as i32 {
                let x = x as usize;
                let y = y as usize;

                let risk = risks_map[y][x];

                if let &Node::Touched(other_dist) = &nodes_map[y][x] {
                    if other_dist > curr_dist + risk {
                        nodes_map[y][x] = Node::Touched(curr_dist + risk);
                        // println!("  ({}, {}): {}", x, y, curr_dist + risk);
                    }
                } else if let Node::Unvisited = &nodes_map[y][x] {
                    nodes_map[y][x] = Node::Touched(curr_dist + risk);
                    // println!("  ({}, {}): {}*", x, y, curr_dist + risk);
                }
            }
        }

        nodes_map[curr_y][curr_x] = Node::Visited(curr_dist);

        if curr_x == max_x && curr_y == max_y {
            break;
        }

        // Find the next Touched node with lowest distance
        curr_node_coords = nodes_map.iter().enumerate()
            .flat_map(|(row_index, row)| {
                row.iter().enumerate().map(|(node_index, node)| ((node_index, row_index), node)).collect::<Vec<_>>()
            })
            .filter_map(|(coord, node)| {
                match node {
                    Node::Touched(dist) => Some((coord, dist)),
                    _ => None,
                }
            })
            .min_by_key(|(_coord, dist)| **dist)
            .map(|(coord, _dist)| coord);
    }

    println!("Part 1: {:?}", &nodes_map[max_y][max_x]);


    let big_risks_map: Vec<Vec<u32>> = [0,1,2,3,4].into_iter().flat_map(|y_repeat| {
        risks_map.iter().map(|row| {
            [0,1,2,3,4].into_iter().flat_map(|x_repeat| {
                row.iter().map(|&risk| ((risk + y_repeat + x_repeat - 1) % 9) + 1).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut big_nodes_map = big_risks_map.iter().map(|row| {
        row.iter().map(|_| Node::Unvisited).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    big_nodes_map[0][0] = Node::Touched(big_risks_map[0][0]);

    let mut curr_node_coords = Some((0usize, 0usize));

    let max_x = big_nodes_map[0].len() - 1;
    let max_y = big_nodes_map.len() - 1;

    while let Some((curr_x, curr_y)) = curr_node_coords {
        let curr_dist = if curr_x == 0 && curr_y == 0 {
            0
        } else {
            let curr_node = &big_nodes_map[curr_y][curr_x];
            match curr_node {
                Node::Touched(d) => *d,
                _ => panic!(),
            }
        };

        // println!("({}, {}): {}", curr_x, curr_y, curr_dist);

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let x = curr_x as i32 + dx;
            let y = curr_y as i32 + dy;

            if x >= 0 && x < big_risks_map[0].len() as i32 && y >= 0 && y < big_risks_map.len() as i32 {
                let x = x as usize;
                let y = y as usize;

                let risk = big_risks_map[y][x];

                if let &Node::Touched(other_dist) = &big_nodes_map[y][x] {
                    if other_dist > curr_dist + risk {
                        big_nodes_map[y][x] = Node::Touched(curr_dist + risk);
                        // println!("  ({}, {}): {}", x, y, curr_dist + risk);
                    }
                } else if let Node::Unvisited = &big_nodes_map[y][x] {
                    big_nodes_map[y][x] = Node::Touched(curr_dist + risk);
                    // println!("  ({}, {}): {}*", x, y, curr_dist + risk);
                }
            }
        }

        big_nodes_map[curr_y][curr_x] = Node::Visited(curr_dist);

        if curr_x == max_x && curr_y == max_y {
            break;
        }

        // Find the next Touched node with lowest distance
        curr_node_coords = big_nodes_map.iter().enumerate()
            .flat_map(|(row_index, row)| {
                row.iter().enumerate().map(|(node_index, node)| ((node_index, row_index), node)).collect::<Vec<_>>()
            })
            .filter_map(|(coord, node)| {
                match node {
                    Node::Touched(dist) => Some((coord, dist)),
                    _ => None,
                }
            })
            .min_by_key(|(_coord, dist)| **dist)
            .map(|(coord, _dist)| coord);
    }

    println!("Part 2: {:?}", &big_nodes_map[max_y][max_x]);
}
