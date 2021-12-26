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
within the program (one for each digit of the model number). Each section either increases the
degree of the polynomial (by multiplying by 26) and adds a new term, or decreases the degree of the
polynomial (chopping off any constant term). If a 'decrease' section tries to add something non-zero
to the newly constant term, the final result will not be 0 - either the model number will be invalid.
Whether a 'decrease' section will add a non-zero constant depends on whether the section's input
plus a constant is equal to the decreased polynomial's constant term. That constant term is just a
previous sections input plus a different constant. So whether a model number is valid boils down to
whether pairs of inputs are separated by some constant (input[a] == input[b] + c + d). The pairs
can be derived via a stack - push the input to the stack for the 'increase' sections, and pop off
again on the 'decrease' sections. Each section of instructions is identical except for three literal
numbers:
 0: 1 or 26, defining decrease or increase respectively
 1: An arbitrary constant
 2: An arbitrary constant
The required difference between pairs of inputs is "constant 2 from the increase section plus
constant 1from the decrease section".

So, finally, we can understand this function. It keeps two arrays of 14 digits - one for the minimum
acceptable model number, one for the maximum. It iterates through each notional 'section' of
instructions. For 'increase' sections, it pushes the section/digit index plus constant 2, for later
use. For 'decrease' sections, it pops from the stack, at which point it has a pair, so can determine
the max and min values for each digit:
- adding the popped const 2 with the current section's const 1 gives the required difference between
  the two digits.
- this may be positive (so current digit >= popped digit) or negative (the reverse)
- for the min model number, whichever digit is the smaller must be 1, and the other 1 + abs(diff)
- for the max model number, whichever digit is the greater must be 9, and the other 9 - abs(diff)
 */
fn max_min_model_num() {
    // These constants are hardcoded from my input for expediency
    let consts = [
        (1, 10, 0),
        (1, 12, 6),
        (1, 13, 4),
        (1, 13, 2),
        (1, 14, 9),
        (26, -2, 1),
        (1, 11, 10),
        (26, -15, 6),
        (26, -10, 4),
        (1, 10, 6),
        (26, -10, 3),
        (26, -4, 9),
        (26, -1, 15),
        (26, -1, 5)
    ];

    let mut max = [0; 14];
    let mut min = [0; 14];

    let mut stack = vec![];

    for digit_index in 0..14 {
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

    println!("Part 1: {}", join_to_string(max));
    println!("Part 2: {}", join_to_string(min));
}

fn join_to_string(digits: [i32; 14]) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        digits[0],
        digits[1],
        digits[2],
        digits[3],
        digits[4],
        digits[5],
        digits[6],
        digits[7],
        digits[8],
        digits[9],
        digits[10],
        digits[11],
        digits[12],
        digits[13],
    )
}

fn main() {
    // let program = parse_program("input");
    // let part1 = find_highest_model_number(&program);

    // let part1 = monad();
    // println!("{:?}", part1);

    max_min_model_num();
}
