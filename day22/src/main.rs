use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

#[derive(Debug)]
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

    fn contains(&self, point: &Point) -> bool {
        self.x_range.contains(&point.x) &&
        self.y_range.contains(&point.y) &&
        self.z_range.contains(&point.z)
    }

    fn is_contained_by(&self, other: &Cuboid) -> bool {
        other.x_range.contains(self.x_range.start()) && other.x_range.contains(self.x_range.end()) &&
        other.y_range.contains(self.y_range.start()) && other.y_range.contains(self.y_range.end()) &&
        other.z_range.contains(self.z_range.start()) && other.z_range.contains(self.z_range.end())
    }

    fn points(&self) -> Points {
        Points::new(&self)
    }
}

struct Points<'a> {
    cuboid: &'a Cuboid,
    x: i32,
    y: i32,
    z: i32,
}
impl Points<'_> {
    fn new(cuboid: &Cuboid) -> Points {
        Points {
            cuboid,
            x: *cuboid.x_range.start(),
            y: *cuboid.y_range.start(),
            z: *cuboid.z_range.start(),
        }
    }
}
impl Iterator for Points<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.z > self.cuboid.z_range.end() {
            return None;
        }

        let point = Point::new(self.x, self.y, self.z);

        self.x += 1;
        if &self.x > self.cuboid.x_range.end() {
            self.y += 1;
            self.x = *self.cuboid.x_range.start();

            if &self.y > self.cuboid.y_range.end() {
                self.z += 1;
                self.y = *self.cuboid.y_range.start();
            }
        }

        Some(point)
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
    on_cubes: HashSet<Point>,
    initialisation_area: Cuboid,
}
impl ReactorCore {
    fn new() -> ReactorCore {
        ReactorCore {
            on_cubes: HashSet::new(),
            initialisation_area: Cuboid {
                x_range: -50..=50,
                y_range: -50..=50,
                z_range: -50..=50,
            }
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        if instruction.cuboid.is_contained_by(&self.initialisation_area) {
            for point in instruction.cuboid.points() {
                if self.initialisation_area.contains(&point) {
                    if instruction.is_on {
                        self.on_cubes.insert(point);
                    } else {
                        self.on_cubes.remove(&point);
                    }
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Could not read input");
    let instructions = input.lines().map(|l| Instruction::from(l)).collect::<Vec<_>>();
    let mut reactor_core = ReactorCore::new();
    for instruction in instructions {
        reactor_core.process(&instruction);
    }

    println!("Part 1: {}", reactor_core.on_cubes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_iterator() {
        let cuboid = Cuboid {
            x_range: -1..=1,
            y_range: -1..=1,
            z_range: -1..=1,
        };
        let points = cuboid.points().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point::new(-1, -1, -1),
                Point::new(0, -1, -1),
                Point::new(1, -1, -1),
                Point::new(-1, 0, -1),
                Point::new(0, 0, -1),
                Point::new(1, 0, -1),
                Point::new(-1, 1, -1),
                Point::new(0, 1, -1),
                Point::new(1, 1, -1),
                Point::new(-1, -1, 0),
                Point::new(0, -1, 0),
                Point::new(1, -1, 0),
                Point::new(-1, 0, 0),
                Point::new(0, 0, 0),
                Point::new(1, 0, 0),
                Point::new(-1, 1, 0),
                Point::new(0, 1, 0),
                Point::new(1, 1, 0),
                Point::new(-1, -1, 1),
                Point::new(0, -1, 1),
                Point::new(1, -1, 1),
                Point::new(-1, 0, 1),
                Point::new(0, 0, 1),
                Point::new(1, 0, 1),
                Point::new(-1, 1, 1),
                Point::new(0, 1, 1),
                Point::new(1, 1, 1),
            ]
        )
    }
}