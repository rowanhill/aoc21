use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }

    fn face_inverse_x(&self) -> Point {
        self.rotate_around_y_times(2)
    }
    fn face_inverse_y(&self) -> Point {
        self.rotate_around_z_times(2)
    }
    fn face_inverse_z(&self) -> Point {
        self.rotate_around_x_times(2)
    }

    fn rotate_around_x(&self) -> Point {
        Point { x: self.x, y: -1*self.z, z: self.y }
    }
    fn rotate_around_y(&self) -> Point {
        Point { y: self.y, z: -1*self.x, x: self.z }
    }
    fn rotate_around_z(&self) -> Point {
        Point { z: self.z, y: -1*self.x, x: self.y }
    }
    fn rotate_around_x_times(&self, times: u8) -> Point {
        self.rotate_times(times, |p| p.rotate_around_x())
    }
    fn rotate_around_y_times(&self, times: u8) -> Point {
        self.rotate_times(times, |p| p.rotate_around_y())
    }
    fn rotate_around_z_times(&self, times: u8) -> Point {
        self.rotate_times(times, |p| p.rotate_around_z())
    }
    fn rotate_times(&self, times: u8, transform: fn(&Point) -> Point) -> Point {
        assert!(times > 0 && times < 4);
        let mut count = 1;
        let mut result = transform(self);
        while count < times {
            result = transform(&result);
            count += 1;
        }
        result
    }

    fn translation_needed_to(&self, other: &Point) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    fn translate(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn dist_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

struct Scan {
    scanner_id: usize,
    points: Vec<Point>,
}

impl Scan {
    fn transform<F>(&self, t: F) -> Scan where F: Fn(&Point) -> Point {
        Scan {
            scanner_id: self.scanner_id,
            points: self.points.iter().map(|p| t(p)).collect()
        }
    }

    fn translate(&self, point: &Point) -> Scan {
        self.transform(|p| p.translate(point))
    }

    fn overlaps(&self, other: &Scan) -> Option<Point> {
        let mut translation_counts = HashMap::<Point, u32>::new();
        for other_point in &other.points {
            for matched_point in &self.points {
                let translation = matched_point.translation_needed_to(other_point);
                let count = translation_counts.entry(translation).or_default();
                *count += 1;
                if *count >= 12 {
                    let translation = matched_point.translation_needed_to(other_point);
                    return Some(translation);
                }
            }
        }
        None
    }
}

fn parse_to_all_orientations(input: &str) -> (Scan, HashMap<usize, Vec<Scan>>) {
    let mut iter = input.split("\n\n");

    let scanner0_scan = parse_scanner_input(iter.next().expect("Could not get scanner 0's chunk"));

    let scans = iter.map(|chunk| parse_scanner_input(chunk))
        .map(|scan| (scan.scanner_id, transform_scan(scan)))
        .collect();

    (scanner0_scan, scans)
}

fn parse_scanner_input(input: &str) -> Scan {
    let mut iter = input.lines();
    let top_line = iter.next().expect("Could not read top line");
    let id = top_line[12..(top_line.len()-4)].parse().expect("Could not parse id");
    let points = iter.map(|line| {
        let parts = line.split(',')
            .map(|part| part.parse().expect("Could not part point part"))
            .collect::<Vec<_>>();
        assert_eq!(parts.len(), 3);
        Point { x: parts[0], y: parts[1], z: parts[2] }
    }).collect();

    Scan { scanner_id: id, points }
}

fn transform_scan(base_scan: Scan) -> Vec<Scan> {
    let transforms: Vec<fn(&Point) -> Point> = vec![
        // +ve x
        // |point| point,
        |point| point.rotate_around_x_times(1),
        |point| point.rotate_around_x_times(2),
        |point| point.rotate_around_x_times(3),
        // -ve x
        |point| point.face_inverse_x(),
        |point| point.face_inverse_x().rotate_around_x_times(1),
        |point| point.face_inverse_x().rotate_around_x_times(2),
        |point| point.face_inverse_x().rotate_around_x_times(3),
        // -ve y
        |point| point.rotate_around_z(),
        |point| point.rotate_around_z().rotate_around_y_times(1),
        |point| point.rotate_around_z().rotate_around_y_times(2),
        |point| point.rotate_around_z().rotate_around_y_times(3),
        // +ve y
        |point| point.rotate_around_z().face_inverse_y(),
        |point| point.rotate_around_z().face_inverse_y().rotate_around_y_times(1),
        |point| point.rotate_around_z().face_inverse_y().rotate_around_y_times(2),
        |point| point.rotate_around_z().face_inverse_y().rotate_around_y_times(3),
        // -ve z
        |point| point.rotate_around_y(),
        |point| point.rotate_around_y().rotate_around_z_times(1),
        |point| point.rotate_around_y().rotate_around_z_times(2),
        |point| point.rotate_around_y().rotate_around_z_times(3),
        // +ve z
        |point| point.rotate_around_y().face_inverse_z(),
        |point| point.rotate_around_y().face_inverse_z().rotate_around_z_times(1),
        |point| point.rotate_around_y().face_inverse_z().rotate_around_z_times(2),
        |point| point.rotate_around_y().face_inverse_z().rotate_around_z_times(3),
    ];

    let mut scans: Vec<Scan> = transforms.into_iter().map(|transform| {
        base_scan.transform(transform)
    }).collect();
    scans.insert(0, base_scan);
    scans
}

struct OceanMapper {
    absolute_scans: Vec<(Point, Scan)>,
    relative_scans_by_id: HashMap<usize, Vec<Scan>>,
    unprocessed_absolute_scan_index_queue: Vec<usize>,
}

impl OceanMapper {
    fn new(path: &str) -> OceanMapper {
        let input = fs::read_to_string(path).expect("Could not read input");
        let (scanner_0_scan, relative_scans_by_id) = parse_to_all_orientations(&input);

        let absolute_scans = vec![(Point::origin(), scanner_0_scan)];
        let unprocessed_absolute_scan_index_queue = vec![0];

        OceanMapper {
            absolute_scans,
            relative_scans_by_id,
            unprocessed_absolute_scan_index_queue,
        }
    }

    fn triangulate_scanners(&mut self) {
        while let Some(absolute_scan_index) = self.unprocessed_absolute_scan_index_queue.pop() {
            let mut positioned_scanners = vec![];
            {
                let (_, absolute_scan) = self.absolute_scans.get(absolute_scan_index).unwrap();
                for (_, relative_scans) in &self.relative_scans_by_id {
                    for oriented_scan in relative_scans {
                        if let Some(offset) = oriented_scan.overlaps(absolute_scan) {
                            positioned_scanners.push((offset, oriented_scan.translate(&offset)));
                            break;
                        }
                    }
                }
            }
            while let Some(positioned_scan) = positioned_scanners.pop() {
                self.relative_scans_by_id.remove(&positioned_scan.1.scanner_id);
                self.absolute_scans.push(positioned_scan);
                self.unprocessed_absolute_scan_index_queue.push(self.absolute_scans.len() - 1);
            }
        }
    }

    fn count_distinct_points(&self) -> usize {
        let mut distinct_points = HashSet::new();
        for (_, abs_scan) in &self.absolute_scans {
            for p in &abs_scan.points {
                distinct_points.insert(p);
            }
        }
        distinct_points.len()
    }

    fn max_distance_between_scanners(&self) -> i32 {
        let mut max_dist = 0;
        for (pos_a, _) in &self.absolute_scans {
            for (pos_b, _) in &self.absolute_scans {
                let dist = pos_a.dist_to(pos_b);
                max_dist = std::cmp::max(max_dist, dist);
            }
        }
        max_dist
    }
}

fn main() {
    let mut ocean_mapper = OceanMapper::new("input");
    ocean_mapper.triangulate_scanners();

    println!("Part 1: {}", ocean_mapper.count_distinct_points());
    println!("Part 2: {}", ocean_mapper.max_distance_between_scanners());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translated_required() {
        let scanner_0_point = Point { x: -618, y: -824, z: -621};
        let scanner_1_point = Point { x: 686, y: 422, z: 578 };
        let scanner_1_scan = Scan { scanner_id: 1, points: vec![scanner_1_point] };
        let scanner_1_scans = transform_scan(scanner_1_scan);

        for scan in scanner_1_scans {
            let point = scan.points[0];
            let trans = point.translation_needed_to(&scanner_0_point);
            println!("{:?} => {:?}", point, trans);
        }
    }

    #[test]
    fn test_overlapping_example_0_and_1() {
        let example_input = std::fs::read_to_string("example").unwrap();
        let (scan_0, scans) = parse_to_all_orientations(&example_input);
        for scan_1 in &scans[&1] {
            if let Some(trans) = scan_1.overlaps(&scan_0) {
                println!("{:?}", trans);
            }
        }
    }
}
