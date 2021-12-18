use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::mem::replace;
use std::str::Chars;

#[derive(PartialEq, Debug, Clone)]
enum SnailNode {
    Literal(u32),
    Pair(Box<SnailNode>, Box<SnailNode>),
}

trait SnailNumberParser {
    fn parse(input: &str) -> SnailNode {
        Self::parse_pair(&mut input.chars().peekable())
    }

    fn parse_node(chars: &mut Peekable<Chars>) -> SnailNode {
        let start = chars.peek().expect("Could not peek node start char");
        match start {
            '[' => Self::parse_pair(chars),
            '0'..='9' => Self::parse_literal(chars),
            _ => panic!("Unexpected node start char: {}", start)
        }
    }

    fn parse_pair(chars: &mut Peekable<Chars>) -> SnailNode {
        Self::consume('[', chars);
        let left = Self::parse_node(chars);
        Self::consume(',', chars);
        let right = Self::parse_node(chars);
        Self::consume(']', chars);
        SnailNode::Pair(Box::new(left), Box::new(right))
    }

    fn parse_literal(chars: &mut Peekable<Chars>) -> SnailNode {
        let num = chars.next().expect("Could not get literal char");
        SnailNode::Literal(num.to_digit(10).expect("Could not parse literal digit"))
    }

    fn consume(expected: char, chars: &mut Peekable<Chars>) {
        let found = chars.next().expect("Could not consume char");
        assert_eq!(found, expected, "Expected to consume {} but found {}", expected, found);
    }
}
impl SnailNumberParser for SnailNode {}

impl SnailNode {
    fn add(self, other: SnailNode) -> SnailNode {
        let mut result = SnailNode::Pair(Box::new(self), Box::new(other));
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        loop {
            let result = self.explode(0);
            if result.is_none() {
                if !self.split() {
                    // If we didn't explode and didn't split, we're fully reduced
                    break;
                }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailNode::Literal(v) => *v,
            SnailNode::Pair(l, r) => {
                3 * l.magnitude() + 2 * r.magnitude()
            }
        }
    }

    fn explode(&mut self, depth: u8) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            SnailNode::Literal(_) => Option::None,
            SnailNode::Pair(left, right) => {
                if depth == 4 {
                    // Swap out this node with a 0, return a Some result with left and right values to add
                    match replace(self, SnailNode::Literal(0)) {
                        SnailNode::Literal(_) => panic!("Pair turned into literal"),
                        SnailNode::Pair(left, right) => {
                            match (*left, *right) {
                                (SnailNode::Literal(lvalue), SnailNode::Literal(rvalue)) => {
                                    Some((Some(lvalue), Some(rvalue)))
                                }
                                _ => panic!("Pair at depth 4 had non-literal children during explode")
                            }
                        }
                    }
                } else {
                    let result = left.explode(depth + 1);
                    if let Some((lresult, rresult)) = result {
                        if let Some(value) = rresult {
                            right.add_to_leftmost_literal(value);
                            Some((lresult, None))
                        } else {
                            result
                        }
                    } else {
                        let result = right.explode(depth + 1);
                        if let Some((lresult, rresult)) = result {
                            if let Some(value) = lresult {
                                left.add_to_rightmost_literal(value);
                                Some((None, rresult))
                            } else {
                                result
                            }
                        } else {
                            result
                        }
                    }
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailNode::Literal(v) => {
                if *v >= 10 {
                    let left = *v / 2;
                    let right = *v - left;
                    *self = SnailNode::Pair(Box::new(SnailNode::Literal(left)), Box::new(SnailNode::Literal(right)));
                    true
                } else {
                    false
                }
            }
            SnailNode::Pair(l, r) => {
                l.split() || r.split()
            }
        }
    }

    fn add_to_leftmost_literal(&mut self, value: u32) {
        match self {
            SnailNode::Literal(v) => {
                *v += value;
            }
            SnailNode::Pair(l, _) => {
                l.add_to_leftmost_literal(value);
            }
        }
    }

    fn add_to_rightmost_literal(&mut self, value: u32) {
        match self {
            SnailNode::Literal(v) => {
                *v += value;
            }
            SnailNode::Pair(_, r) => {
                r.add_to_rightmost_literal(value);
            }
        }
    }
}

fn read_file(path: &str) -> Vec<SnailNode> {
    BufReader::new(File::open(path).expect("Could not read input"))
        .lines()
        .map(|line| {
            let line = line.expect("Could not read line");
            SnailNode::parse(&line)
        })
        .collect()
}

fn main() {
    let nums: Vec<SnailNode> = read_file("input");

    let sum = nums.iter()
        .map(|sn| sn.clone())
        .reduce(|acc, sn| acc.add(sn))
        .expect("Could not sum");
    println!("Part 1: {}", sum.magnitude());
    
    let mut max_magnitude = 0;
    for outer in nums.iter() {
        for inner in nums.iter() {
            if outer == inner {
                continue;
            }
            let outer = outer.clone();
            let inner = inner.clone();
            let sum = outer.add(inner);
            let magnitude = sum.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    println!("Part 2: {}", max_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_adding() {
        let result = sn("[1,2]").add(sn("[[3,4],5]"));
        assert_eq!(result, sn("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_explode() {
        assert_eq!(ex("[[[[[9,8],1],2],3],4]"), sn("[[[[0,9],2],3],4]"));
        assert_eq!(ex("[7,[6,[5,[4,[3,2]]]]]"), sn("[7,[6,[5,[7,0]]]]"));
        assert_eq!(ex("[[6,[5,[4,[3,2]]]],1]"), sn("[[6,[5,[7,0]]],3]"));
        assert_eq!(ex("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), sn("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        assert_eq!(ex("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), sn("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_split() {
        let mut node = SnailNode::Pair(Box::new(SnailNode::Literal(10)), Box::new(SnailNode::Literal(0)));
        let result = node.split();
        assert_eq!((result, node), (true, sn("[[5,5],0]")));

        let mut node = SnailNode::Pair(Box::new(SnailNode::Literal(11)), Box::new(SnailNode::Literal(0)));
        let result = node.split();
        assert_eq!((result, node), (true, sn("[[5,6],0]")));
    }

    #[test]
    fn test_add_with_reduce() {
        let result = sn("[[[[4,3],4],4],[7,[[8,4],9]]]").add(sn("[1,1]"));
        assert_eq!(result, sn("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(sn("[9,1]").magnitude(), 29);
        assert_eq!(sn("[1,9]").magnitude(), 21);
        assert_eq!(sn("[[9,1],[1,9]]").magnitude(), 129);
        assert_eq!(sn("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
    }

    fn sn(input: &str) -> SnailNode {
        SnailNode::parse(input)
    }

    fn ex(input: &str) -> SnailNode {
        let mut node = sn(input);
        node.explode(0);
        node
    }
}
