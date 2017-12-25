// https://adventofcode.com/2017/day/23

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    // First star
    let program = parse_program("input.txt");
    let mut reg = [0i64; 8];
    let mut pc = 0;
    let mut mults = 0;
    while pc < program.len() {
        let (new_pc, mult) = step(pc, &mut reg, &program);
        if mult {
            mults += 1;
        }
        pc = new_pc;
    }
    // Assert to facilitate further tweaks
    assert_eq!(9409, mults);

    println!("Mul is invoked {} times", mults);

    // Second star
    // Write out in rust, minimal optimizations to accurately "emulate" for fun ;)
    // Counts composites between b and c with stepsize 17?
    // More optimized would be
    // 'i: for i in (b..(c + 17)).step_by(17) {
    //     for d in 2..(i / 2) {
    //         if i % d == 0 {
    //             h += 1;
    //             continue 'i;
    //         }
    //     }
    // }
    let mut b = 99 * 100 + 100000;
    let c = b + 17000;
    let mut h = 0;
    loop {
        let mut f = 1;
        'd: for d in 2..b {
            'e: for e in 2..b {
                let g = d * e - b;
                if g > 0 {
                    // No need to check products after d * e > b
                    break 'e;
                }
                if g == 0 {
                    f = 0;
                    // f remains 0 so we can break on first hit
                    break 'd;
                }
            }
        }
        if f == 0 {
            h += 1;
        }
        if b == c {
            break;
        }
        b += 17;
    }
    // Assert to facilitate further tweaks
    assert_eq!(913, h);

    println!("Register h holds {}", h);
}

// Executes instruction at program[pc] and returns next value of pc
fn step(pc: usize, registers: &mut [i64; 8], program: &Vec<Instr>) -> (usize, bool) {
    let mut pc = pc;
    let mut mult = false;
    match program[pc] {
        // Sets target register to given value
        Instr::Set(target, ref operand, _) => match *operand {
            Oper::Reg(source) => registers[target] = registers[source],
            Oper::Val(val) => registers[target] = val,
        },
        // Subtracts given (negative) value from target register
        Instr::Sub(target, ref operand, _) => match *operand {
            Oper::Reg(source) => registers[target] -= registers[source],
            Oper::Val(val) => registers[target] -= val,
        },
        // Multiplies target register with given value
        Instr::Mul(target, ref operand, _) => {
            match *operand {
                Oper::Reg(source) => registers[target] *= registers[source],
                Oper::Val(val) => registers[target] *= val,
            };
            mult = true;
        }
        // Jumps with given offset if condition value is greater than zero
        Instr::Jnz(ref cond, ref oper, _) => {
            let lhs = match *cond {
                Oper::Reg(target) => registers[target],
                Oper::Val(val) => val,
            };
            if lhs != 0 {
                let offset = match *oper {
                    Oper::Reg(source) => registers[source],
                    Oper::Val(val) => val,
                };
                pc = (pc as i64 + offset - 1) as usize; // Subtract 0 to negate pc increment
            }
        }
    }
    pc += 1;

    (pc, mult)
}

fn parse_program(file: &str) -> Vec<Instr> {
    let f = BufReader::new(File::open(file).expect("Opening input.txt failed"));

    // Parse instructions
    let mut program = Vec::new();
    for line in f.lines() {
        // Get instruction and target register
        let raw_line = line.expect("Reading line failed");
        let split: Vec<&str> = raw_line.split(' ').collect();
        let instruction = split[0];
        let target = split[1].chars().nth(0).expect("No target register");

        // Push instruction
        match instruction {
            "set" => program.push(Instr::Set(
                get_reg(target),
                parse_oper(split[2]),
                raw_line.clone(),
            )),
            "sub" => program.push(Instr::Sub(
                get_reg(target),
                parse_oper(split[2]),
                raw_line.clone(),
            )),
            "mul" => program.push(Instr::Mul(
                get_reg(target),
                parse_oper(split[2]),
                raw_line.clone(),
            )),
            "jnz" => program.push(Instr::Jnz(
                parse_oper(split[1]),
                parse_oper(split[2]),
                raw_line.clone(),
            )),
            _ => panic!("Invalid instruction"),
        }
    }

    program
}

fn get_reg(name: char) -> usize {
    match name {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid register {}", name),
    }
}

fn parse_oper(operand: &str) -> Oper {
    // Try to parse a value from the given operand
    match operand.parse::<i64>() {
        Ok(val) => Oper::Val(val), // Success
        Err(_) => {
            // Not a value, try getting register name
            let reg_name = operand.chars().nth(0).expect("No operand to parse");
            Oper::Reg(get_reg(reg_name))
        }
    }
}

#[derive(Debug)]
enum Instr {
    Set(usize, Oper, String),
    Sub(usize, Oper, String),
    Mul(usize, Oper, String),
    Jnz(Oper, Oper, String),
}

#[derive(Debug)]
enum Oper {
    Val(i64),
    Reg(usize),
}
