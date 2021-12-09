use std::collections::hash_map::RandomState;
use std::collections::hash_set::SymmetricDifference;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input").expect("Could not read file"));

    let observations: Vec<([String; 10],[String; 4])> = reader.lines().map(|line| {
        let line = line.expect("Could not read line");

        let (samples, output_value) = line.split_once(" | ").expect("Could not split on delimiter");

        let mut samples_arr = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
        for (i, sample) in samples.splitn(10, " ").enumerate() {
            samples_arr[i].push_str(sample);
        }

        let mut output_value_arr = [String::new(), String::new(), String::new(), String::new()];
        for (i, sample) in output_value.splitn(4, " ").enumerate() {
            output_value_arr[i].push_str(sample);
        }
        (samples_arr, output_value_arr)
    }).collect();

    let num_unique_len_digits: u32 = observations.iter().map(|(_, digits)| {
        digits.iter().filter(|digit| {
            match digit.len() {
                2 | 3 | 4 | 7 => true,
                _ => false
            }
        }).count() as u32
    }).sum();

    println!("Part 1: {}", num_unique_len_digits);

    let sum: u32 = observations.iter().map(|(signals, output_digits)| {
        // find 2-len: "1": ab
        let one_signal = signals.iter().find(|s| s.len() == 2).expect("Could not find '1'");

        // find 3-len: "7": dab
        let seven_signal = signals.iter().find(|s| s.len() == 3).expect("Could not find '7'");

        // => top = ("7" - "1") = d
        let top = extra_char(seven_signal, one_signal);

        // find 5-len overlapping with "7": "3": fbcad
        let three_signal = signals.iter()
            .find(|s| s.len() == 5 && contains_all_chars(s, seven_signal))
            .expect("Could not find '3'");

        // find 4-len "4": eafb
        let four_signal = signals.iter().find(|s| s.len() == 4).expect("Could not find '4'");

        // => middle = intersection of ("3" - "1") and ("4" - "1") = intersection of (fcd) and (ef) = f
        let middle = {
            let three_minus_one = subtract(three_signal, one_signal);
            let four_minus_one = subtract(four_signal, one_signal);
            let mut intersection = three_minus_one.intersection(&four_minus_one);
            let char = intersection.next().expect("Could not find intersecting char");
            assert!(intersection.next().is_none(), "Found more than one intersecting char");
            *char
        };

        // => top-left = "4" - "1" - middle = e
        let top_left = {
            let mut four_minus_one = subtract(four_signal, one_signal);
            four_minus_one.remove(&middle);
            let mut remaining = four_minus_one.iter();
            let char = remaining.next().expect("Could not find remaining char");
            assert!(remaining.next().is_none(), "Found more than one remaining char");
            *char
        };

        // find 6-len overlapping with "1" and with middle: "9": cefabd
        let nine_signal = signals.iter()
            .find(|s| s.len() == 6 &&
                contains_all_chars(s, one_signal) &&
                s.contains(|c| c == middle))
            .expect("Could not find '9'");

        // find 7-len: "8"
        let eight_signal = signals.iter().find(|s| s.len() == 7).expect("Could not find '8'");

        // => bottom-left = "8" - "9" = g
        let bottom_left = {
            let eight_minus_nine = subtract(eight_signal, nine_signal);
            let mut remaining = eight_minus_nine.iter();
            let char = remaining.next().expect("Could not find remaining char");
            assert!(remaining.next().is_none(), "Found more than one remaining char");
            *char
        };

        // find other 6-len with middle: "6": cdfgeb
        let six_signal = signals.iter()
            .find(|s| s.len() == 6 &&
                *s != nine_signal &&
                s.contains(|c| c == middle))
            .expect("Could not find '6'");

        // => top-right = "8" - "6" = a
        let top_right = {
            let eight_minus_six = subtract(eight_signal, six_signal);
            let mut remaining = eight_minus_six.iter();
            let char = remaining.next().expect("Could not find remaining char");
            assert!(remaining.next().is_none(), "Found more than one remaining char");
            *char
        };

        // => bottom-right = "1" - top-right = b
        let bottom_right = {
            let one_minus_top_right = subtract(one_signal, &top_right.to_string());
            let mut remaining = one_minus_top_right.iter();
            let char = remaining.next().expect("Could not find remaining char");
            assert!(remaining.next().is_none(), "Found more than one remaining char");
            *char
        };

        // => bottom = "8" - top - middle - top-left - bottom-left - top-right - bottom-right = c
        let bottom = {
            let mut eight_chars: HashSet<char, RandomState> = HashSet::from_iter(eight_signal.chars());
            eight_chars.remove(&top);
            eight_chars.remove(&middle);
            eight_chars.remove(&top_left);
            eight_chars.remove(&bottom_left);
            eight_chars.remove(&top_right);
            eight_chars.remove(&bottom_right);
            let mut remaining = eight_chars.iter();
            let char = remaining.next().expect("Could not find remaining char");
            assert!(remaining.next().is_none(), "Found more than one remaining char");
            *char
        };

        let mut numbers = String::new();
        for digit in output_digits {
            let pattern = [&top, &top_left, &top_right, &middle, &bottom_left, &bottom_right, &bottom]
                .map(|c| digit.contains(|dc| &dc == c));

            let number = match pattern {
                [true, true, true, false, true, true, true] => '0',
                [false, false, true, false, false, true, false] => '1',
                [true, false, true, true, true, false, true] => '2',
                [true, false, true, true, false, true, true] => '3',
                [false, true, true, true, false, true, false] => '4',
                [true, true, false, true, false, true, true] => '5',
                [true, true, false, true, true, true, true] => '6',
                [true, false, true, false, false, true, false] => '7',
                [true, true, true, true, true, true, true] => '8',
                [true, true, true, true, false, true, true] => '9',
                _ => panic!("Unexpected 7-segment pattern")
            };
            numbers.push(number);
        }

        let numbers_int = numbers.parse::<u32>().expect("Could not parse numbers");
        // println!("{} => {}", numbers, numbers_int);
        numbers_int
    }).sum();

    println!("Part 2: {}", sum);
}

fn extra_char(a: &str, b: &str) -> char {
    let a_chars_set = HashSet::from_iter(a.chars());
    let b_chars_set = HashSet::from_iter(b.chars());
    let mut diff_chars: SymmetricDifference<char, RandomState> = a_chars_set.symmetric_difference(&b_chars_set);
    let char = diff_chars.next().expect("Could not find a symmetric difference character");
    assert!(diff_chars.next().is_none(), "Found more than one symmetric difference character");
    *char
}

fn contains_all_chars(longer: &str, shorter: &str) -> bool {
    let longer_chars_set: HashSet<char, RandomState> = HashSet::from_iter(longer.chars());
    let shorter_chars_set: HashSet<char, RandomState> = HashSet::from_iter(shorter.chars());
    longer_chars_set.is_superset(&shorter_chars_set)
}

fn subtract(longer: &str, shorter: &str) -> HashSet<char> {
    let longer_chars_set: HashSet<char, RandomState> = HashSet::from_iter(longer.chars());
    let shorter_chars_set: HashSet<char, RandomState> = HashSet::from_iter(shorter.chars());
    longer_chars_set.difference(&shorter_chars_set).map(|c| *c).collect()
}