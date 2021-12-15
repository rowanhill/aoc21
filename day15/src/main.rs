use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input").expect("Could not read file"));
    let risks_map = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        line.chars().map(|c| c.to_digit(10).expect("Could not parse digit")).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let max_x = risks_map[0].len() - 1;
    let max_y = risks_map.len() - 1;
    let part1 = shortest_path(&risks_map, (0, 0), (max_x, max_y));
    println!("Part 1: {:?}", part1);


    let big_risks_map: Vec<Vec<u32>> = [0,1,2,3,4].into_iter().flat_map(|y_repeat| {
        risks_map.iter().map(|row| {
            [0,1,2,3,4].into_iter().flat_map(|x_repeat| {
                row.iter().map(|&risk| ((risk + y_repeat + x_repeat - 1) % 9) + 1).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let max_x = big_risks_map[0].len() - 1;
    let max_y = big_risks_map.len() - 1;
    let part2 = shortest_path(&big_risks_map, (0, 0), (max_x, max_y));
    println!("Part 2: {:?}", part2);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(entry_cost_map: &Vec<Vec<u32>>, start: (usize, usize), goal: (usize, usize)) -> Option<u32> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = entry_cost_map.iter()
        .map(|row| (0..row.len()).map(|_| u32::MAX).collect())
        .collect::<Vec<Vec<_>>>();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.1][start.0] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0] { continue; }

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let x = position.0 as i32 + dx;
            let y = position.1 as i32 + dy;

            if x >= 0 && x < entry_cost_map[0].len() as i32 && y >= 0 && y < entry_cost_map.len() as i32 {
                let x = x as usize;
                let y = y as usize;

                let entry_cost = &entry_cost_map[y][x];

                let next = State { cost: cost + entry_cost, position: (x, y) };

                // If so, add it to the frontier and continue
                if next.cost < dist[y][x] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[y][x] = next.cost;
                }
            }
        }
    }

    // Goal not reachable
    None
}