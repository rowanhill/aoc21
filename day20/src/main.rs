use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}

trait Widenable {
    fn widen_by(&self, delta: isize) -> Self;
}
impl Widenable for RangeInclusive<isize> {
    fn widen_by(&self, delta: isize) -> Self {
        (self.start() - delta)..=(self.end() + delta)
    }
}

struct Bounds {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}
impl Bounds {
    fn grow(&self) -> Bounds {
        Bounds {
            x_range: self.x_range.widen_by(2),
            y_range: self.y_range.widen_by(2),
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.x_range.contains(&coord.x) && self.y_range.contains(&coord.y)
    }
}

struct Image {
    bounds: Bounds,
    lit_pixels: HashSet<Coord>,
    bg_is_lit: bool,
}
impl Image {
    fn parse(input: &str) -> Image {
        let lit_pixels = input.lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Coord::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .fold(HashSet::new(), |mut acc, coords| {
                coords.for_each(|c| {
                    acc.insert(c);
                });
                acc
            });
        let max_y = input.lines().count() as isize - 1;
        let max_x = input.find("\n").expect("Could not find line ending") as isize - 1;

        Image {
            bounds: Bounds { x_range: 0..=max_x, y_range: 0..=max_y },
            lit_pixels,
            bg_is_lit: false,
        }
    }

    fn step(&self, enh_alg: &Vec<bool>) -> Image {
        let new_bounds = self.bounds.grow();

        let mut new_lit_pixels = HashSet::new();
        for x in new_bounds.x_range.clone() {
            for y in new_bounds.y_range.clone() {
                let coord = Coord::new(x, y);
                let neighbourhood_num = self.num_from_pixel_neighbourhood(&coord);
                let should_be_lit = enh_alg.is_pixel_lit(neighbourhood_num);
                if should_be_lit {
                    new_lit_pixels.insert(coord);
                }
            }
        }

        let out_of_bounds = Coord::new(self.bounds.x_range.start() - 10, self.bounds.y_range.start() - 10);
        let oob_neighbourhood_num = self.num_from_pixel_neighbourhood(&out_of_bounds);
        let new_bg_is_lit = enh_alg.is_pixel_lit(oob_neighbourhood_num);
        
        Image {
            bounds: new_bounds,
            lit_pixels: new_lit_pixels,
            bg_is_lit: new_bg_is_lit,
        }
    }

    fn is_pixel_lit(&self, coord: &Coord) -> bool {
        if self.bounds.contains(coord) {
            self.lit_pixels.contains(coord)
        } else {
            self.bg_is_lit
        }
    }

    fn num_from_pixel(&self, coord: &Coord) -> usize {
        if self.is_pixel_lit(coord) {
            1
        } else {
            0
        }
    }

    fn num_from_pixel_neighbourhood(&self, coord: &Coord) -> usize {
        let offsets = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        offsets.map(|(dx, dy)| Coord::new(coord.x + dx, coord.y + dy))
            .map(|c| self.num_from_pixel(&c))
            .into_iter()
            .fold(0, |acc, i| {
                acc * 2 + i
            })
    }

    #[allow(dead_code)]
    fn as_debug_string(&self, xrange: RangeInclusive<isize>, yrange: RangeInclusive<isize>) -> String {
        let mut result = String::new();
        for y in yrange.clone() {
            for x in xrange.clone() {
                let coord = Coord::new(x, y);
                let char = if self.is_pixel_lit(&coord) { '#' } else { '.' };
                result.push(char);
            }
            if &y < yrange.end() {
                result.push('\n');
            }
        }
        result
    }
}

trait ImageEnhancementAlgorithm {
    fn is_pixel_lit(&self, index: usize) -> bool;
}
impl ImageEnhancementAlgorithm for Vec<bool> {
    fn is_pixel_lit(&self, index: usize) -> bool {
        self[index]
    }
}
fn parse_image_enhancement_algorithm(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '#').collect()
}

fn parse_input_file(path: &str) -> (Vec<bool>, Image) {
    let input = std::fs::read_to_string(path).expect("Could not read input file");
    let (alg_input, image_input) = input.split_once("\n\n").expect("Could not split input");
    let enh_alg = parse_image_enhancement_algorithm(alg_input);
    let image = Image::parse(image_input);
    (enh_alg, image)
}

fn main() {
    let (enh_alg, mut image) = parse_input_file("input");

    for i in 1..=50 {
        image = image.step(&enh_alg);
        if i == 2 {
            println!("Part 1: {}", image.lit_pixels.len());
        }
    }
    println!("Part 2: {}", image.lit_pixels.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_centre_pixel_neighbourhood_num() {
        let (_, image) = parse_input_file("example");
        let centre = Coord::new(2, 2);
        let num = image.num_from_pixel_neighbourhood(&centre);
        assert_eq!(num, 34);
    }

    #[test]
    fn test_example_input_centre_pixel_is_lit_next_step() {
        let (enh_alg, _) = parse_input_file("example");
        assert_eq!(enh_alg.is_pixel_lit(34), true);
    }

    #[test]
    fn test_example_pixels_that_should_light_after_step() {
        let (enh_alg, image) = parse_input_file("example");

        let coord = Coord::new(-1, 0);
        let num = image.num_from_pixel_neighbourhood(&coord);
        let will_be_lit = enh_alg.is_pixel_lit(num);
        assert_eq!(will_be_lit, true);

        let coord = Coord::new(0, -1);
        let num = image.num_from_pixel_neighbourhood(&coord);
        let will_be_lit = enh_alg.is_pixel_lit(num);
        assert_eq!(will_be_lit, true);
    }

    #[test]
    fn test_example_pixels_are_lit_after_step() {
        let (enh_alg, image) = parse_input_file("example");
        let image = image.step(&enh_alg);

        let coord = Coord::new(-1, 0);
        let is_lit = image.is_pixel_lit(&coord);
        assert_eq!(is_lit, true);

        let coord = Coord::new(0, -1);
        let is_lit = image.is_pixel_lit(&coord);
        assert_eq!(is_lit, true);
    }

    #[test]
    fn test_debug_string() {
        let (_, image) = parse_input_file("example");
        let expected = "#..#.
#....
##..#
..#..
..###";
        assert_eq!(image.as_debug_string(0..=4, 0..=4), expected);
    }

    #[test]
    fn test_example_after_one_step() {
        let (enh_alg, image) = parse_input_file("example");
        let image = image.step(&enh_alg);
        let expected = "...............
...............
...............
...............
.....##.##.....
....#..#.#.....
....##.#..#....
....####..#....
.....#..##.....
......##..#....
.......#.#.....
...............
...............
...............
...............";
        assert_eq!(image.as_debug_string(-5..=9, -5..=9), expected);
    }

    #[test]
    fn test_example_lit_pixels_after_two_steps() {
        let (enh_alg, image) = parse_input_file("example");
        let new_image = image.step(&enh_alg).step(&enh_alg);
        assert_eq!(new_image.lit_pixels.len(), 35);
    }
}
