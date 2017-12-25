// https://adventofcode.com/2017/day/18

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

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
    let mut reg = [0i64; 5];
    let mut pc = 0;
    let mut snd = VecDeque::new();
    let mut rcv = VecDeque::new();
    while pc < program.len() {
        // Track last emitted sound
        let prev_len = rcv.len();
        let prev_sound = match rcv.back() {
            Some(&val) => Some(val),
            None => None,
        };

        // Step program
        let (new_pc, _) = step(pc, &mut reg, &mut snd, &mut rcv, &program);

        // Move sent messages to other buffer or check for receive
        if !snd.is_empty() {
            rcv.push_back(snd.pop_front().unwrap());
        } else if rcv.len() < prev_len {
            // Output a sound so printout the previous sound and break
            let sound = prev_sound.unwrap();
            // Assert to facilitate further tweaks
            assert_eq!(1187, sound);

            println!("First sound received was {}", sound);
            break;
        }

        // Set new program counter
        pc = new_pc;
    }

    // Execute second star
    let mut pc0 = 0;
    let mut reg0 = [0, 0, 0, 0, 0];
    let mut queue0 = VecDeque::new();
    let mut pc1 = 0;
    let mut reg1 = [0, 0, 0, 0, 1];
    let mut queue1 = VecDeque::new();
    let mut sends = 0;
    while pc0 < program.len() && pc1 < program.len() {
        // Step program 0
        let (new_pc0, waiting1) = step(pc0, &mut reg0, &mut queue1, &mut queue0, &program);
        // Step program 1, track sends
        let prev_len = queue0.len();
        let (new_pc1, waiting2) = step(pc1, &mut reg1, &mut queue0, &mut queue1, &program);
        if prev_len < queue0.len() {
            sends += 1;
        }

        // Stop execution on deadlock
        if waiting1 && waiting2 {
            break;
        }

        // Set program counters
        pc0 = new_pc0;
        pc1 = new_pc1;
    }
    // Assert to facilitate further tweaks
    assert_eq!(5969, sends);

    println!("Program 1 sent a value {} times", sends);
}

// Executes instruction at program[pc] and returns next value of pc
fn step(
    pc: usize,
    registers: &mut [i64; 5],
    snd: &mut VecDeque<i64>,
    rcv: &mut VecDeque<i64>,
    program: &Vec<Instr>,
) -> (usize, bool) {
    let mut pc = pc;
    let mut waiting = false;
    match program[pc] {
        // Send value in target register
        Instr::Snd(target) => snd.push_back(registers[target]),
        // Receive value to target register
        Instr::Rcv(target) => if rcv.is_empty() {
            // Wait if queue is empty
            pc -= 1;
            waiting = true;
        } else {
            registers[target] = rcv.pop_front().unwrap();
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

    (pc, waiting)
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
