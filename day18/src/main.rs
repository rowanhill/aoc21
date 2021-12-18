use std::fs::File;
use std::io::{BufRead, BufReader};
use typed_arena::Arena;
use crate::snail_numbers::SnailNode;

mod snail_numbers {
    use typed_arena::Arena;
    use crate::snail_numbers::explosive::{ExplodeResult, Explosive};
    use crate::snail_numbers::splittable::Splittable;

    #[derive(PartialEq, Debug, Clone)]
    pub(crate) enum SnailNode<'a> {
        Literal(u32),
        Pair(&'a SnailNode<'a>, &'a SnailNode<'a>),
    }

    impl <'a> SnailNode<'a> {
        pub(crate) fn parse(input: &str, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
            parsing::parse_pair(&mut input.chars().peekable(), arena)
        }

        pub(crate) fn add(&'a self, other: &'a SnailNode<'a>, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
            let result = arena.alloc(SnailNode::Pair(self, other));
            result.reduce(arena)
        }

        pub(crate) fn magnitude(&self) -> u32 {
            match self {
                SnailNode::Literal(v) => *v,
                SnailNode::Pair(l, r) => {
                    3 * l.magnitude() + 2 * r.magnitude()
                }
            }
        }

        fn reduce(&'a self, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode {
            let mut result = self;
            loop {
                result = match result.explode(0, arena) {
                    ExplodeResult::Exploded(n, _) => {
                        n
                    },
                    ExplodeResult::Unchanged => {
                        let split_result = result.split(arena);
                        let split_match_result = match split_result {
                            Some(n) => {
                                n
                            },
                            None => {
                                break;
                            }
                        };
                        split_match_result
                    }
                }
            }
            result
        }
    }

    pub(crate) mod explosive {
        use typed_arena::Arena;
        use crate::SnailNode;

        type Remainder = (Option<u32>, Option<u32>);
        pub(crate) enum ExplodeResult<'a> {
            Exploded(&'a SnailNode<'a>, Remainder),
            Unchanged
        }

        pub(crate) trait Explosive<'a> {
            fn explode(&'a self, depth: u8, arena: &'a Arena<SnailNode<'a>>) -> ExplodeResult<'a>;
        }

        impl <'a> Explosive<'a> for SnailNode<'a> {
            /*
            Checks for explosion in this subtree

            Returns Exploded(_,_) if the subtree exploded, or Unchanged if not

            The values within the Exploded remainder tuple are the literal values to be added to the nearest literal to
            the left and to the right respectively. They start as Some(l) and Some(r), but once consumed
            (i.e. once added to a literal) they turn into None, to prevent them being added again further
            up the tree.

            Once a subtree has exploded, it will not try to explode any remaining portion of the tree.

            Proceeds depth-first left-to-right.
            */
            fn explode(&'a self, depth: u8, arena: &'a Arena<SnailNode<'a>>) -> ExplodeResult {
                match self {
                    SnailNode::Literal(_) => ExplodeResult::Unchanged,
                    SnailNode::Pair(left, right) => {
                        if depth == 4 {
                            match (left, right) {
                                (SnailNode::Literal(lvalue), SnailNode::Literal(rvalue)) => {
                                    ExplodeResult::Exploded(
                                        arena.alloc(SnailNode::Literal(0)),
                                        (Some(*lvalue), Some(*rvalue))
                                    )
                                },
                                _ => panic!("Pair at depth 4 had non-literal children during explode")
                            }
                        } else {
                            let left_result = left.explode(depth + 1, arena);
                            match left_result {
                                ExplodeResult::Exploded(new_left, remainder) => {
                                    match remainder {
                                        (lrem, Some(rrem)) => {
                                            let new_right = right.add_to_leftmost_literal(rrem, arena);
                                            let new_rem = (lrem, None);
                                            ExplodeResult::Exploded(
                                                arena.alloc(SnailNode::Pair(new_left, new_right)),
                                                new_rem
                                            )
                                        },
                                        (_, None) => {
                                            ExplodeResult::Exploded(
                                                arena.alloc(SnailNode::Pair(new_left, right)),
                                                remainder
                                            )
                                        }
                                    }
                                }
                                ExplodeResult::Unchanged => {
                                    let right_result = right.explode(depth + 1, arena);
                                    match right_result {
                                        ExplodeResult::Exploded(new_right, remainder) => {
                                            match remainder {
                                                (Some(lrem), rrem) => {
                                                    let new_left = left.add_to_rightmost_literal(lrem, arena);
                                                    let new_rem = (None, rrem);
                                                    ExplodeResult::Exploded(
                                                        arena.alloc(SnailNode::Pair(new_left, new_right)),
                                                        new_rem
                                                    )
                                                },
                                                (None, _) => {
                                                    ExplodeResult::Exploded(
                                                        arena.alloc(SnailNode::Pair(left, new_right)),
                                                        remainder
                                                    )
                                                }
                                            }
                                        }
                                        ExplodeResult::Unchanged => ExplodeResult::Unchanged
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        trait ExplosivePrivate<'a> {
            fn add_to_leftmost_literal(&'a self, value: u32, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode;
            fn add_to_rightmost_literal(&'a self, value: u32, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode;
        }
        impl <'a> ExplosivePrivate<'a> for SnailNode<'a> {
            fn add_to_leftmost_literal(&'a self, value: u32, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
                match self {
                    SnailNode::Literal(v) => {
                        arena.alloc(SnailNode::Literal(*v + value))
                    }
                    SnailNode::Pair(l, r) => {
                        let new_left = l.add_to_leftmost_literal(value, arena);
                        arena.alloc(SnailNode::Pair(new_left, r))
                    }
                }
            }

            fn add_to_rightmost_literal(&'a self, value: u32, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
                match self {
                    SnailNode::Literal(v) => {
                        arena.alloc(SnailNode::Literal(*v + value))
                    }
                    SnailNode::Pair(l, r) => {
                        let new_right = r.add_to_rightmost_literal(value, arena);
                        arena.alloc(SnailNode::Pair(l, new_right))
                    }
                }
            }
        }
    }

    pub(crate) mod splittable {
        use typed_arena::Arena;
        use crate::SnailNode;

        pub(crate) trait Splittable<'a> {
            fn split(&'a self, arena: &'a Arena<SnailNode<'a>>) -> Option<&'a SnailNode<'a>>;
        }

        impl <'a> Splittable<'a> for SnailNode<'a> {
            fn split(&'a self, arena: &'a Arena<SnailNode<'a>>) -> Option<&'a SnailNode<'a>> {
                match self {
                    SnailNode::Literal(v) => {
                        if *v >= 10 {
                            let left = *v / 2;
                            let right = *v - left;
                            let left_node = arena.alloc(SnailNode::Literal(left));
                            let right_node = arena.alloc(SnailNode::Literal(right));
                            let node = arena.alloc(SnailNode::Pair(left_node, right_node));
                            Some(node)
                        } else {
                            None
                        }
                    }
                    SnailNode::Pair(l, r) => {
                        match l.split(arena) {
                            Some(new_left) => {
                                let node = arena.alloc(SnailNode::Pair(new_left, r));
                                Some(node)
                            }
                            None => {
                                match r.split(arena) {
                                    Some(new_right) => {
                                        let node = arena.alloc(SnailNode::Pair(l, new_right));
                                        Some(node)
                                    },
                                    None => None
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    mod parsing {
        use std::iter::Peekable;
        use std::str::Chars;
        use typed_arena::Arena;
        use crate::SnailNode;

        pub(crate) fn parse_pair<'a>(chars: &mut Peekable<Chars>, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
            consume('[', chars);
            let left = parse_node(chars, arena);
            consume(',', chars);
            let right = parse_node(chars, arena);
            consume(']', chars);
            arena.alloc(SnailNode::Pair(left, right))
        }

        fn parse_node<'a>(chars: &mut Peekable<Chars>, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
            let start = chars.peek().expect("Could not peek node start char");
            match start {
                '[' => parse_pair(chars, arena),
                '0'..='9' => parse_literal(chars, arena),
                _ => panic!("Unexpected node start char: {}", start)
            }
        }

        fn parse_literal<'a>(chars: &mut Peekable<Chars>, arena: &'a Arena<SnailNode<'a>>) -> &'a SnailNode<'a> {
            let num = chars.next().expect("Could not get literal char");
            let num = num.to_digit(10).expect("Could not parse literal digit");
            arena.alloc(SnailNode::Literal(num))
        }

        fn consume(expected: char, chars: &mut Peekable<Chars>) {
            let found = chars.next().expect("Could not consume char");
            assert_eq!(found, expected, "Expected to consume {} but found {}", expected, found);
        }
    }
}

fn read_file<'a>(path: &str, arena: &'a Arena<SnailNode<'a>>) -> Vec<&'a SnailNode<'a>> {
    BufReader::new(File::open(path).expect("Could not read input"))
        .lines()
        .map(|line| {
            let line = line.expect("Could not read line");
            SnailNode::parse(&line, arena)
        })
        .collect()
}

fn main() {
    let arena = Arena::new();

    let nums: Vec<&SnailNode> = read_file("input", &arena);

    let sum = nums.iter()
        .map(|n| *n)
        .reduce(|acc, sn| acc.add(sn, &arena))
        .expect("Could not sum");
    println!("Part 1: {}", sum.magnitude());
    
    let mut max_magnitude = 0;
    for outer in nums.iter() {
        for inner in nums.iter() {
            if outer == inner {
                continue;
            }
            let sum = outer.add(inner, &arena);
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
    use crate::snail_numbers::splittable::Splittable;
    use crate::snail_numbers::explosive::{ExplodeResult, Explosive};
    use super::*;

    #[test]
    fn test_simple_adding() {
        let f = f();
        let result = f.sn("[1,2]").add(&f.sn("[[3,4],5]"), &f.arena);
        assert_eq!(result, f.sn("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_explode() {
        let f = f();
        assert_eq!(f.ex("[[[[[9,8],1],2],3],4]"), f.sn("[[[[0,9],2],3],4]"));
        assert_eq!(f.ex("[7,[6,[5,[4,[3,2]]]]]"), f.sn("[7,[6,[5,[7,0]]]]"));
        assert_eq!(f.ex("[[6,[5,[4,[3,2]]]],1]"), f.sn("[[6,[5,[7,0]]],3]"));
        assert_eq!(f.ex("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), f.sn("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        assert_eq!(f.ex("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), f.sn("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_split() {
        let f = f();
        let node = SnailNode::Pair(&SnailNode::Literal(10), &SnailNode::Literal(0));
        let result = node.split(&f.arena);
        assert_eq!(result, Some(f.sn("[[5,5],0]")));

        let node = SnailNode::Pair(&SnailNode::Literal(11), &SnailNode::Literal(0));
        let result = node.split(&f.arena);
        assert_eq!(result, Some(f.sn("[[5,6],0]")));
    }

    #[test]
    fn test_add_with_reduce() {
        let f = f();
        let result = f.sn("[[[[4,3],4],4],[7,[[8,4],9]]]").add(&f.sn("[1,1]"), &f.arena);
        assert_eq!(result, f.sn("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_magnitude() {
        let f = f();
        assert_eq!(f.sn("[9,1]").magnitude(), 29);
        assert_eq!(f.sn("[1,9]").magnitude(), 21);
        assert_eq!(f.sn("[[9,1],[1,9]]").magnitude(), 129);
        assert_eq!(f.sn("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
    }

    fn f<'a>() -> Factory<'a> {
        Factory::new()
    }

    struct Factory<'a> {
        arena: Arena<SnailNode<'a>>
    }
    impl <'a> Factory<'a> {
        fn new() -> Factory<'a> {
            Factory { arena: Arena::new() }
        }

        fn sn(&'a self, input: &str) -> &'a SnailNode<'a> {
            SnailNode::parse(input, &self.arena)
        }

        fn ex(&'a self, input: &str) -> &'a SnailNode<'a> {
            let node = self.sn(input);
            match node.explode(0, &self.arena) {
                ExplodeResult::Exploded(n, _) => n,
                ExplodeResult::Unchanged => node
            }
        }
    }
}
