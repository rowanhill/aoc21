use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Cuboid {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
}
impl Cuboid {
    fn from(input: &str) -> Cuboid {
        let mut ranges = input.split(',')
            .map(|r| &r[2..])
            .map(|r| {
                let (from, to) = r.split_once("..").expect("No .. split");
                let from: i32 = from.parse().expect("Could not parse from");
                let to: i32 = to.parse().expect("Could not parse to");
                from..=to
            })
            .collect::<Vec<_>>();
        assert_eq!(ranges.len(), 3);
        Cuboid {
            x_range: ranges.remove(0),
            y_range: ranges.remove(0),
            z_range: ranges.remove(0),
        }
    }

    fn clone_from(x_range: &RangeInclusive<i32>, y_range: &RangeInclusive<i32>, z_range: &RangeInclusive<i32>) -> Cuboid {
        Cuboid {
            x_range: x_range.clone(),
            y_range: y_range.clone(),
            z_range: z_range.clone(),
        }
    }

    fn overlaps_with(&self, other: &Cuboid) -> bool {
        ranges_overlap(&self.x_range, &other.x_range) &&
        ranges_overlap(&self.y_range, &other.y_range) &&
        ranges_overlap(&self.z_range, &other.z_range)
    }

    fn is_contained_by(&self, other: &Cuboid) -> bool {
        other.x_range.contains(self.x_range.start()) && other.x_range.contains(self.x_range.end()) &&
        other.y_range.contains(self.y_range.start()) && other.y_range.contains(self.y_range.end()) &&
        other.z_range.contains(self.z_range.start()) && other.z_range.contains(self.z_range.end())
    }

    fn subtract(self, other: &Cuboid, output: &mut Vec<Cuboid>) {
        // If this cuboid and the other don't intersect, then this cuboid won't be split, and can
        // push itself onto the output without modification
        if !self.overlaps_with(other) {
            output.push(self);
            return;
        }

        for x_op in [range_before, range_overlap, range_after] {
            if let Some(new_x) = x_op(&self.x_range, &other.x_range) {
                for y_op in [range_before, range_overlap, range_after] {
                    if let Some(new_y) = y_op(&self.y_range, &other.y_range) {
                        for z_op in [range_before, range_overlap, range_after] {
                            if x_op as usize != range_overlap as usize ||
                                y_op as usize != range_overlap as usize ||
                                z_op as usize != range_overlap as usize
                            {
                                if let Some(new_z) = z_op(&self.z_range, &other.z_range) {
                                    output.push(Cuboid::clone_from(&new_x, &new_y, &new_z));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn volume(&self) -> usize {
        (self.x_range.end() - self.x_range.start() + 1) as usize *
            (self.y_range.end() - self.y_range.start() + 1) as usize *
            (self.z_range.end() - self.z_range.start() + 1) as usize
    }
}

fn ranges_overlap(range: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> bool {
    range.start() <= other.end() && range.end() >= other.start()
}

fn range_before(range: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    if range.start() < other.start() {
        let new_end = std::cmp::min(*range.end(), other.start() - 1);
        Some(*range.start()..=new_end)
    } else {
        None
    }
}

fn range_overlap(range: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    if ranges_overlap(range, other) {
        let overlap_start = std::cmp::max(range.start(), other.start());
        let overlap_end = std::cmp::min(range.end(), other.end());
        Some(*overlap_start..=*overlap_end)
    } else {
        None
    }
}

fn range_after(range: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    if range.end() > other.end() {
        let new_start = std::cmp::max(other.end() + 1, *range.start());
        Some(new_start..=*range.end())
    } else {
        None
    }
}

#[derive(Debug)]
struct Instruction {
    is_on: bool,
    cuboid: Cuboid,
}
impl Instruction {
    fn from(input: &str) -> Instruction {
        let (is_on, ranges) = input.split_once(' ').expect("No space split");
        let is_on = is_on == "on";
        let cuboid = Cuboid::from(ranges);
        Instruction {
            is_on,
            cuboid
        }
    }
}

struct ReactorCore {
    on_cuboids: Vec<Cuboid>,
}
impl ReactorCore {
    fn new() -> ReactorCore {
        ReactorCore {
            on_cuboids: vec![],
        }
    }

    fn initialise(&mut self, instructions: &Vec<Instruction>) {
        let initialisation_area = Cuboid { x_range: -50..=50, y_range: -50..=50, z_range: -50..=50 };
        let init_instructions = instructions.iter()
            .filter(|i| i.cuboid.is_contained_by(&initialisation_area));
        for instruction in init_instructions {
            self.process(instruction);
        }
    }

    fn reboot(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.process(instruction);
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        let old_cuboids = std::mem::replace(&mut self.on_cuboids, vec![]);
        for cuboid in old_cuboids {
            cuboid.subtract(&instruction.cuboid, &mut self.on_cuboids);
        }
        if instruction.is_on {
            self.on_cuboids.push(instruction.cuboid.clone())
        }
    }

    fn count_on_cubes(&self) -> usize {
        self.on_cuboids.iter().map(|c| c.volume()).sum()
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Could not read input");
    let instructions = input.lines().map(|l| Instruction::from(l)).collect::<Vec<_>>();

    let mut reactor_core = ReactorCore::new();
    reactor_core.initialise(&instructions);
    println!("Part 1: {}", reactor_core.count_on_cubes());

    let mut reactor_core = ReactorCore::new();
    reactor_core.reboot(&instructions);
    println!("Part 2: {}", reactor_core.count_on_cubes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuboid_volume() {
        let cuboid = Cuboid {
            x_range: -1..=1,
            y_range: -1..=1,
            z_range: -1..=1,
        };
        assert_eq!(cuboid.volume(), 27);
    }

    #[test]
    fn test_subtracting_wholly_overlapped_cube() {
        let bigger = Cuboid {
            x_range: -1..=1,
            y_range: -1..=1,
            z_range: -1..=1,
        };
        let smaller = Cuboid {
            x_range: 0..=0,
            y_range: 0..=0,
            z_range: 0..=0,
        };
        let mut results = vec![];
        bigger.subtract(&smaller, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 26);
    }

    #[test]
    fn test_subtracting_wholly_overlapping_cube() {
        let bigger = Cuboid {
            x_range: -1..=1,
            y_range: -1..=1,
            z_range: -1..=1,
        };
        let smaller = Cuboid {
            x_range: 0..=0,
            y_range: 0..=0,
            z_range: 0..=0,
        };
        let mut results = vec![];
        smaller.subtract(&bigger, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 0);
    }

    #[test]
    fn test_subtracting_partially_overlapping_cube() {
        let first = Cuboid {
            x_range: -1..=1,
            y_range: -1..=1,
            z_range: -1..=1,
        };
        let second = Cuboid {
            x_range: 0..=2,
            y_range: 0..=2,
            z_range: 0..=2,
        };
        let mut results = vec![];
        first.subtract(&second, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 19);
    }

    #[test]
    fn test_non_overlapping_cube() {
        let first = Cuboid {
            x_range: -1..=1,
            y_range: -1..=1,
            z_range: -1..=1,
        };
        let second = Cuboid {
            x_range: 2..=3,
            y_range: 2..=3,
            z_range: 2..=3,
        };
        let mut results = vec![];
        first.subtract(&second, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 27);
    }

    #[test]
    fn test_subtracting_first_two_instructions() {
        let first = Cuboid {
            x_range: -44..=9,
            y_range: -9..=44,
            z_range: -34..=13,
        };
        assert_eq!(first.volume(), 139968);
        let second = Cuboid {
            x_range: -42..=11,
            y_range: -16..=33,
            z_range: -2..=48,
        };
        let mut results = vec![];
        first.subtract(&second, &mut results);
        assert_eq!(results.len(), 7);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 104192);
    }

    #[test]
    fn test_example_1_part_1() {
        let input = std::fs::read_to_string("example1").expect("Could not read input");
        let instructions = input.lines().map(|l| Instruction::from(l)).collect::<Vec<_>>();
        let mut reactor_core = ReactorCore::new();
        reactor_core.initialise(&instructions);
        assert_eq!(reactor_core.count_on_cubes(), 39);
    }
}