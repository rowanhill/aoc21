// For various 'paper' working see https://docs.google.com/spreadsheets/d/1i6aJbyZE-37sVAUL5jy029xRr_Q1i8tZva4yxVVP4oA/edit?usp=sharing


// An ALU emulator - waaay too slow

// use crate::BinOp::{Add, Divide, Equal, Modulo, Multiply};
// use crate::Instruction::*;
// use crate::Value::*;
//
// struct RegIndex(usize);
//
// enum Value {
//     Literal(isize),
//     Reference(RegIndex),
// }
// impl Value {
//     fn parse(input: &str) -> Value {
//         input.parse::<isize>()
//             .and_then(|v| Ok(Literal(v)))
//             .unwrap_or_else(|_| Reference(parse_reg_index(input)))
//     }
//
//     fn get(&self, registers: &[isize; 4]) -> isize {
//         match self {
//             Literal(v) => *v,
//             Reference(idx) => registers[idx.0],
//         }
//     }
// }
//
// enum BinOp {
//     Add,
//     Multiply,
//     Divide,
//     Modulo,
//     Equal,
// }
// enum Instruction {
//     Input(RegIndex),
//     Operation(BinOp, RegIndex, Value),
// }
// impl Instruction {
//     fn parse(input: &str) -> Instruction {
//         let (name, rest) = input.split_once(' ').expect("Can't find name - no space");
//         match name {
//             "inp" => {
//                 let reg = parse_reg_index(rest);
//                 Input(reg)
//             },
//             "add" | "mul" | "div" | "mod" | "eql" => {
//                 let (a, b) = rest.split_once(' ').expect("Can't find a, b - no space");
//                 let a = parse_reg_index(a);
//                 let b = Value::parse(b);
//                 let op = match name {
//                     "add" => Add,
//                     "mul" => Multiply,
//                     "div" => Divide,
//                     "mod" => Modulo,
//                     "eql" => Equal,
//                     _ => unreachable!("Unknown two-arg instruction {}", name),
//                 };
//                 Operation(op, a, b)
//             },
//             _ => unreachable!("Unknown instruction name {}", name)
//         }
//     }
// }
// fn parse_reg_index(input: &str) -> RegIndex {
//     match input {
//         "w" => RegIndex(0),
//         "x" => RegIndex(1),
//         "y" => RegIndex(2),
//         "z" => RegIndex(3),
//         _ => unreachable!("Unknown register name {}", input)
//     }
// }
//
// struct Alu {
//     registers: [isize; 4], // w, x, y, z
// }
// impl Alu {
//     fn new() -> Alu {
//         Alu { registers: [0; 4] }
//     }
//
//     fn execute(&mut self, program: &Vec<Instruction>, inputs: &Vec<isize>) {
//         let mut input_iter = inputs.iter();
//         for instruction in program {
//             match instruction {
//                 Input(dest) => {
//                     if let Some(input) = input_iter.next() {
//                         self.registers[dest.0] = *input;
//                     } else {
//                         panic!("Program needs input, but none available");
//                     }
//                 }
//                 Operation(op, src_dest, val) => {
//                     let a_val = self.registers[src_dest.0];
//                     let b_val = val.get(&self.registers);
//                     self.registers[src_dest.0] = match op {
//                         Add => a_val + b_val,
//                         Multiply => a_val * b_val,
//                         Divide => a_val / b_val,
//                         Modulo => a_val % b_val,
//                         Equal => if a_val == b_val { 1 } else { 0 }
//                     }
//                 }
//             }
//         }
//     }
// }
//
// fn parse_program(path: &str) -> Vec<Instruction> {
//     let input = std::fs::read_to_string(path).expect("Could not read input file");
//     input.lines()
//         .map(|line| Instruction::parse(line))
//         .collect()
// }
//
// fn decrement_model_number(digits: &mut [isize; 14]) {
//     let mut index = 13;
//     digits[index] -= 1;
//     while digits[index] == 0 {
//         digits[index] = 9;
//         index -= 1;
//         digits[index] -= 1;
//     }
// }
//
// fn find_highest_model_number(program: &Vec<Instruction>) -> [isize; 14] {
//     let mut model_num = [9; 14];
//
//     loop {
//         let mut alu = Alu::new();
//         alu.execute(program, &model_num.to_vec());
//         if alu.registers[3] == 0 {
//             return model_num;
//         }
//         decrement_model_number(&mut model_num);
//     }
// }


// Decompiled version of my input - still way too slow

// fn monad() -> [isize; 14] {
//     let consts = [
//         (1, 10, 0),
//         (1, 12, 6),
//         (1, 13, 4),
//         (1, 13, 2),
//         (1, 14, 9),
//         (26, -2, 1),
//         (1, 11, 10),
//         (26, -15, 6),
//         (26, -10, 4),
//         (1, 10, 6),
//         (26, -10, 3),
//         (26, -4, 9),
//         (26, -1, 15),
//         (26, -1, 5)
//     ];
//
//     let mut model_num = [9; 14];
//     let mut count = 0u128;
//     loop {
//         let mut total = 0;
//         for i in 0..14 {
//             let digit = model_num[i];
//             let x = (total % 26) + consts[i].1;
//             total = total / consts[i].0;
//             if total > 0 {
//                 // if total is > 0 at this point, it can't decrease (enough? probably?)
//                 break;
//             }
//             if x != digit {
//                 total *= 26;
//                 total += digit + consts[i].2;
//             }
//         }
//         if total == 0 {
//             return model_num;
//         }
//         decrement_model_number(&mut model_num);
//         count += 1;
//         if count % 10000000 == 0 {
//             println!("{}", count);
//         }
//     }
// }

/*
The instructions effectively work with a "polynomial" of base 26. There are 14 very similar sections
within the program (one for each digit of the model number).

Each section may do zero or more of the following things:
 1. Decrease the degree of the polynomial (dividing by 26, chopping off any constant degree).
    Whether or not the decrease happens is determined by a constant in the instructions.
 2. Increase the degree of the polynomial and add a new constant term. This will happen iff the
    section's input is NOT equal to the polynomial's constant term plus another constant (specified
    in the instructions). The polynomial's constant term comes from a previous section's input plus
    a constant (specified in that section's instructions). I.e. whether or not the polynomial's
    degree is increased boils down to whether a pair of inputs have a difference of the sum of two
    constants, i.e. whether input[a] == input[b] + c + d.

In my (and, I expect, all) input there are 7 sections which decrease the polynomial. In order for
the final value of z (the value of the polynomial being manipulated) to be 0, those 7 sections must
NOT also increase the degree of the polynomial, and the other 7 sections must do so. This means the
increasing and decreasing cancel out (assuming the degree of the polynomial remains >= 0), and the
final section will drop the final (constant) term, resulting in 0.

This means that for a model number to be valid, the the 'decrease' sections' inputs must be the
specified difference from the paired section's input. The paired sections can be determined via a
stack - pushing the input to the stack when increasing the polynomial, popping when decreasing.

Each section of instructions is identical except for three literal numbers:
 0: 1 ("div total by 1" - i.e. do nothing) or 26 ("div total by 26" - i.e. decrease the polynomial's
    degree)
 1: An arbitrary constant
 2: An arbitrary constant
The required difference between pairs of inputs is "constant 2 from the increase section plus
constant 1 from the decrease section".

So, finally, we can understand this function. It keeps two arrays of 14 digits - one for the minimum
acceptable model number, one for the maximum. It iterates through each notional 'section' of
instructions. For 'increase' sections, it pushes the section/digit index plus constant 2, for later
use. For 'decrease' sections, it pops from the stack, at which point it has a pair, so can determine
the max and min valid values for each digit:
- adding the popped const 2 with the current section's const 1 gives the required difference between
  the two digits.
- this may be positive (so current digit >= popped digit) or negative (the reverse)
- for the min model number, whichever digit is the smaller must be 1, and the other 1 + abs(diff)
- for the max model number, whichever digit is the greater must be 9, and the other 9 - abs(diff)
 */
fn max_min_model_num(consts: &[(i32, i32, i32); 14]) -> (usize, usize) {
    let mut max = [0; 14];
    let mut min = [0; 14];

    let mut stack = vec![];

    for digit_index in 0..14 {
        // if the 0th constant is 1, this is an 'increase' section (the program divides z by 1,
        // rather than by 26)
        if consts[digit_index].0 == 1 {
            stack.push((digit_index, consts[digit_index].2));
        } else {
            let (prev_digit_index, prev_y_adjustment) = stack.pop().unwrap();
            // The required difference between the prev and current digits
            let diff = prev_y_adjustment + consts[digit_index].1;
            if diff >= 0 {
                // previous digit is smaller than (or equal to) current digit

                // min is 1 and 1 + diff
                min[prev_digit_index] = 1;
                min[digit_index] = 1 + diff;

                // max is 9 - diff and 9
                max[prev_digit_index] = 9 - diff;
                max[digit_index] = 9;
            } else {
                // previous digit is greater than current digit

                // min is 1 and 1 + abs(diff)
                min[digit_index] = 1;
                min[prev_digit_index] = 1 - diff;

                // max is 9 - abs(diff) and 9
                max[digit_index] = 9 + diff;
                max[prev_digit_index] = 9;
            }
        }
    }

    (join_to_int(max), join_to_int(min))
}

fn extract_constants_from_path(path: &str) -> [(i32, i32, i32); 14] {
    let mut result = [(0,0,0); 14];

    let input = std::fs::read_to_string(path).expect("Could not read input file");
    let sections = input.split("inp w\n");
    assert_eq!(sections.clone().count(), 15);

    for (i, section) in sections.skip(1).enumerate() {
        let lines = section.lines().collect::<Vec<_>>();
        result[i].0 = {
            let divz = lines[3];
            assert!(divz.starts_with("div z "));
            divz[6..].parse().expect("Could not parse div z")
        };
        result[i].1 = {
            let addx = lines[4];
            assert!(addx.starts_with("add x "));
            addx[6..].parse().expect("Could not parse add x")
        };
        result[i].2 = {
            let addy = lines[14];
            assert!(addy.starts_with("add y "));
            addy[6..].parse().expect("Could not parse add y")
        };
    }

    result
}

fn join_to_int(digits: [i32; 14]) -> usize {
    digits.into_iter().fold(0usize,|acc, d| acc * 10 + d as usize)
}

fn main() {
    // let program = parse_program("input");
    // let part1 = find_highest_model_number(&program);

    // let part1 = monad();
    // println!("{:?}", part1);

    let (max, min) = max_min_model_num(&extract_constants_from_path("input"));

    println!("Part 1: {}", max);
    println!("Part 2: {}", min);
}
