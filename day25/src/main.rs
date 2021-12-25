use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq)]
enum Cuke {
    East,
    South,
    None
}

#[derive(Clone)]
struct Seabed {
    cukes: Vec<Vec<Cuke>>,
}

impl Seabed {
    fn from_file(path: &str) -> Seabed {
        let reader = BufReader::new(File::open(path).expect("Could not read file"));
        let cukes = reader.lines().map(|line| {
            let line = line.expect("Could not read line");
            line.bytes().map(|b| {
                match b {
                    b'>' => Cuke::East,
                    b'v' => Cuke::South,
                    b'.' => Cuke::None,
                    _ => unreachable!("Unexpected byte {}", b),
                }
            }).collect()
        }).collect();
        Seabed { cukes }
    }

    fn steps_to_stopped(&mut self) -> usize {
        let mut count = 0;
        loop {
            let num_moves = self.step();
            count += 1;
            if num_moves == 0 {
                return count;
            }
        }
    }

    fn step(&mut self) -> usize {
        self.step_herd(Cuke::East, Self::coord_east) +
            self.step_herd(Cuke::South, Self::coord_south)
    }

    fn step_herd(&mut self, herd_type: Cuke, neighbour_gen: fn(&Self, usize, usize) -> (usize, usize)) -> usize {
        let mut count = 0;
        let mut new_seabed = self.clone();

        for y in 0..self.cukes.len() {
            for x in 0..self.cukes[0].len() {
                if &self.cukes[y][x] == &herd_type {
                    let (neighbour_x, neighbour_y) = neighbour_gen(self, x, y);
                    if let Cuke::None = &self.cukes[neighbour_y][neighbour_x] {
                        new_seabed.swap_cukes((x, y), (neighbour_x, neighbour_y));
                        count += 1;
                    }
                }
            }
        }

        self.cukes = new_seabed.cukes;
        count
    }

    fn coord_east(&self, x: usize, y: usize) -> (usize, usize) {
        let x = if x + 1 >= self.cukes[0].len() {
            x + 1 - self.cukes[0].len()
        } else {
            x + 1
        };
        (x, y)
    }

    fn coord_south(&self, x: usize, y: usize) -> (usize, usize) {
        let y = if y + 1 >= self.cukes.len() {
            y + 1 - self.cukes.len()
        } else {
            y + 1
        };
        (x, y)
    }

    fn swap_cukes(&mut self, coord1: (usize, usize), coord2: (usize, usize)) {
        let cuke1 = std::mem::replace(&mut self.cukes[coord1.1][coord1.0], Cuke::None);
        let cuke2 = std::mem::replace(&mut self.cukes[coord2.1][coord2.0], Cuke::None);

        self.cukes[coord1.1][coord1.0] = cuke2;
        self.cukes[coord2.1][coord2.0] = cuke1;
    }

    fn debug_string(&self) -> String {
        let mut result = String::new();
        for row in &self.cukes {
            for cuke in row {
                let char = match cuke {
                    Cuke::East => '>',
                    Cuke::South => 'v',
                    Cuke::None => '.',
                };
                result.push(char);
            }
            result.push('\n');
        }
        result
    }
}

fn main() {
    let mut seabed = Seabed::from_file("input");
    let steps = seabed.steps_to_stopped();
    println!("Part 1: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_step_1() {
        let mut seabed = Seabed::from_file("example");
        seabed.step();
        let str = seabed.debug_string();
        assert_eq!(&str, "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v
")
    }

    #[test]
    fn test_example_steps_to_stopped() {
        let mut seabed = Seabed::from_file("example");
        let steps = seabed.steps_to_stopped();
        assert_eq!(steps, 58);
    }
}
