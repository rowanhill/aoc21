use std::ops::RangeInclusive;
use itertools::Itertools;

trait Intersectable<T> {
    fn ranges_overlap(range: &RangeInclusive<T>, other: &RangeInclusive<T>) -> bool;
    fn spans(&self, other: &RangeInclusive<T>) -> bool;
    fn range_before(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;
    fn range_overlap(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;
    fn range_after(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;
}
impl Intersectable<i32> for RangeInclusive<i32> {
    fn ranges_overlap(range: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> bool {
        range.start() <= other.end() && range.end() >= other.start()
    }

    fn spans(&self, other: &RangeInclusive<i32>) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn range_before(&self, other: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
        if self.start() < other.start() {
            let new_end = std::cmp::min(*self.end(), other.start() - 1);
            Some(*self.start()..=new_end)
        } else {
            None
        }
    }

    fn range_overlap(&self, other: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
        if Self::ranges_overlap(self, other) {
            let overlap_start = std::cmp::max(self.start(), other.start());
            let overlap_end = std::cmp::min(self.end(), other.end());
            Some(*overlap_start..=*overlap_end)
        } else {
            None
        }
    }

    fn range_after(&self, other: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
        if self.end() > other.end() {
            let new_start = std::cmp::max(other.end() + 1, *self.start());
            Some(new_start..=*self.end())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Volume<const N: usize> {
    bounds: [RangeInclusive<i32>; N],
}
const EMPTY_RANGE: RangeInclusive<i32> = 0..=0;
impl<const N: usize> Volume<N> {
    fn from(input: &str) -> Volume<N> {
        let mut ranges = input.split(',')
            .map(|r| &r[2..])
            .map(|r| {
                let (from, to) = r.split_once("..").expect("No .. split");
                let from: i32 = from.parse().expect("Could not parse from");
                let to: i32 = to.parse().expect("Could not parse to");
                from..=to
            });
        Self::from_iter(&mut ranges)
    }

    fn from_iter(vec: &mut dyn Iterator<Item=RangeInclusive<i32>>) -> Volume<N> {
        let mut bounds = [EMPTY_RANGE; N];
        let mut count = 0;
        for (i, r) in vec.enumerate() {
            bounds[i] = r;
            count += 1;
        }
        assert_eq!(count, N);
        Volume { bounds }
    }

    fn overlaps_with(&self, other: &Volume<N>) -> bool {
        (0..self.bounds.len()).all(|i| {
            RangeInclusive::ranges_overlap(&self.bounds[i], &other.bounds[i])
        })
    }

    fn is_contained_by(&self, other: &Volume<N>) -> bool {
        (0..self.bounds.len()).all(|i| {
            other.bounds[i].spans(&self.bounds[i])
        })
    }

    fn subtract(self, other: &Volume<N>, output: &mut Vec<Volume<N>>) {
        // If this volume and the other don't intersect, then this volume won't be split, and can
        // push itself onto the output without modification
        if !self.overlaps_with(other) {
            output.push(self);
            return;
        }

        let subbounds_by_axis = (0..self.bounds.len()).map(|i| {
            [
                (0u8, self.bounds[i].range_before(&other.bounds[i])),
                (1, self.bounds[i].range_overlap(&other.bounds[i])),
                (2, self.bounds[i].range_after(&other.bounds[i])),
            ].into_iter()
        });

        for op_index_and_subbounds in subbounds_by_axis.multi_cartesian_product() {
            // Ignore the subdivision where all ranges overlap - this is the part being subtracted
            if op_index_and_subbounds.iter().all(|(i, _)| i == &1) {
                continue;
            }
            // Ignore any subvolumes where at least one subdivision is None
            if op_index_and_subbounds.iter().any(|(_, r)| r.is_none()) {
                continue;
            }
            let mut subbounds = op_index_and_subbounds.into_iter().map(|(_, r)| r.unwrap());
            let subvolume = Volume::from_iter(&mut subbounds);
            output.push(subvolume);
        }
    }

    fn volume(&self) -> usize {
        self.bounds.iter()
            .map(|range| range.end() - range.start() + 1)
            .fold(1, |acc, i| acc * i as usize)
    }
}

#[derive(Debug)]
struct Instruction {
    is_on: bool,
    cuboid: Volume<3>,
}
impl Instruction {
    fn from(input: &str) -> Instruction {
        let (is_on, ranges) = input.split_once(' ').expect("No space split");
        let is_on = is_on == "on";
        let cuboid = Volume::<3>::from(ranges);
        Instruction {
            is_on,
            cuboid
        }
    }
}

struct ReactorCore {
    on_cuboids: Vec<Volume<3>>,
}
impl ReactorCore {
    fn new() -> ReactorCore {
        ReactorCore {
            on_cuboids: vec![],
        }
    }

    fn initialise(&mut self, instructions: &Vec<Instruction>) {
        let initialisation_area = Volume { bounds: [-50..=50, -50..=50, -50..=50] };
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
        let cuboid = Volume {
            bounds: [-1..=1, -1..=1, -1..=1],
        };
        assert_eq!(cuboid.volume(), 27);
    }

    #[test]
    fn test_subtracting_wholly_overlapping_cube() {
        let bigger = Volume {
            bounds: [-1..=1, -1..=1, -1..=1],
        };
        let smaller = Volume {
            bounds: [0..=0, 0..=0, 0..=0],
        };
        let mut results = vec![];
        smaller.subtract(&bigger, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 0);
    }

    #[test]
    fn test_subtracting_partially_overlapping_cube() {
        let first = Volume {
            bounds: [-1..=1, -1..=1, -1..=1],
        };
        let second = Volume {
            bounds: [0..=2, 0..=2, 0..=2],
        };
        let mut results = vec![];
        first.subtract(&second, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 19);
    }

    #[test]
    fn test_non_overlapping_cube() {
        let first = Volume {
            bounds: [-1..=1, -1..=1, -1..=1],
        };
        let second = Volume {
            bounds: [2..=3, 2..=3, 2..=3],
        };
        let mut results = vec![];
        first.subtract(&second, &mut results);
        let volume: usize = results.iter().map(|c| c.volume()).sum();
        assert_eq!(volume, 27);
    }

    #[test]
    fn test_subtracting_first_two_instructions() {
        let first = Volume {
            bounds: [-44..=9, -9..=44, -34..=13],
        };
        assert_eq!(first.volume(), 139968);
        let second = Volume {
            bounds: [-42..=11, -16..=33, -2..=48],
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