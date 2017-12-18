// https://adventofcode.com/2017/day/18

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

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
            "snd" => program.push(Instr::Snd(get_reg(target))),
            "rcv" => program.push(Instr::Rcv(get_reg(target))),
            "set" => program.push(Instr::Set(get_reg(target), parse_oper(split[2]))),
            "add" => program.push(Instr::Add(get_reg(target), parse_oper(split[2]))),
            "mul" => program.push(Instr::Mul(get_reg(target), parse_oper(split[2]))),
            "mod" => program.push(Instr::Mod(get_reg(target), parse_oper(split[2]))),
            "jgz" => program.push(Instr::Jgz(parse_oper(split[1]), parse_oper(split[2]))),
            _ => panic!("Invalid instruction"),
        }
    }

    // Execute first star
    let mut registers = [0i64; 5];
    let mut pc = 0;
    let mut last_sound = 0;
    while pc < program.len() {
        match program[pc] {
            // Outputs sound in target register
            Instr::Snd(target) => last_sound = registers[target],
            // Recovers last sound played if target register is not 0
            Instr::Rcv(target) => if registers[target] != 0 {
                registers[target] = last_sound;
                // First star asks for this so stop here
                println!("First sound received was {}", last_sound);
                break;
            },
            // Sets target register to given value
            Instr::Set(target, ref operand) => match *operand {
                Oper::Reg(source) => registers[target] = registers[source],
                Oper::Val(val) => registers[target] = val,
            },
            // Adds given value to target register
            Instr::Add(target, ref operand) => match *operand {
                Oper::Reg(source) => registers[target] += registers[source],
                Oper::Val(val) => registers[target] += val,
            },
            // Multiplies target register with given value
            Instr::Mul(target, ref operand) => match *operand {
                Oper::Reg(source) => registers[target] *= registers[source],
                Oper::Val(val) => registers[target] *= val,
            },
            // Stores remainder of register divided by given value to the register
            Instr::Mod(target, ref operand) => match *operand {
                Oper::Reg(source) => registers[target] %= registers[source],
                Oper::Val(val) => registers[target] %= val,
            },
            // Jumps with given offset if condition value is greater than zero
            Instr::Jgz(ref cond, ref oper) => {
                let lhs = match *cond {
                    Oper::Reg(target) => registers[target],
                    Oper::Val(val) => val,
                };
                if lhs > 0 {
                    let offset = match *oper {
                        Oper::Reg(source) => registers[source],
                        Oper::Val(val) => val,
                    };
                    pc = (pc as i64 + offset - 1) as usize; // Subtract 0 to negate pc increment
                }
            }
        }
        pc += 1;
    }
}

fn get_reg(name: char) -> usize {
    match name {
        'a' => 0,
        'b' => 1,
        'f' => 2,
        'i' => 3,
        'p' => 4,
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
    Snd(usize),
    Rcv(usize),
    Set(usize, Oper),
    Add(usize, Oper),
    Mul(usize, Oper),
    Mod(usize, Oper),
    Jgz(Oper, Oper),
}

#[derive(Debug)]
enum Oper {
    Val(i64),
    Reg(usize),
}
