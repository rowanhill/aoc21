use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input").expect("Could not read file"));
    let lines = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        line
    }).collect::<Vec<_>>();

    let (sum_corrupted_score, mut incomplete_scores) = lines.iter()
        .fold((0, vec![]), |(cor, mut inc), line| {
            match parse_chunk(line) {
                ChunkParseResult::Corrupted(bracket) => (cor + corrupted_score(&bracket), inc),
                ChunkParseResult::Incomplete(mut open_brackets) => {
                    inc.push(incomplete_score(&mut open_brackets));
                    (cor, inc)
                }
            }
        });

    println!("Part 1: {}", sum_corrupted_score);

    incomplete_scores.sort();
    let median_incomplete_score = incomplete_scores.get(incomplete_scores.len()/2)
        .expect("Could not find median incomplete score");
    println!("Part 2: {}", median_incomplete_score);
}

enum ChunkParseResult {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn parse_chunk(chunk: &str) -> ChunkParseResult {
    let mut open_brackets = vec![];
    for bracket in chunk.chars() {
        match bracket {
            '(' | '[' | '{' | '<' => {
                open_brackets.push(bracket);
            },
            ')' | ']' | '}' | '>' => {
                if open_brackets.is_empty() {
                    return ChunkParseResult::Incomplete(open_brackets);
                } else {
                    let most_recent_open = open_brackets.pop().expect("Could not get most recent bracket");
                    let expected = match bracket {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        '>' => '<',
                        _ => panic!("Unexpected close bracket type")
                    };
                    if most_recent_open != expected {
                        return ChunkParseResult::Corrupted(bracket);
                    }
                }
            }
            _ => panic!("Unexpected bracket char")
        }
    }
    ChunkParseResult::Incomplete(open_brackets) // open_brackets could be []
}

fn corrupted_score(bracket: &char) -> u32 {
    match bracket {
        ')' => 3u32,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected close bracket type")
    }
}

fn incomplete_score(open_brackets: &mut Vec<char>) -> u128 {
    open_brackets.reverse();
    open_brackets.iter().map(|open| {
        match open {
            '(' => 1u128,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected open bracket type")
        }
    }).reduce(|acc, score| {
        (acc * 5) + score
    }).expect("Could not calc total score")
}
