// https://adventofcode.com/2017/day/5

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    // Read input to a vector
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));
    let mut jumps: Vec<i32> = Vec::new();
    for line in f.lines() {
        let jump: i32 = line.expect("Reading line failed")
            .parse::<i32>()
            .expect("Parsing int failed");
        jumps.push(jump);
    }
    let jumps = jumps;

    // Go through vector until jump goes past it's end
    let mut pc: i32 = 0;
    let mut steps = 0;
    let mut first_jumps = jumps.clone();
    while pc < first_jumps.len() as i32 {
        let jmp = first_jumps[pc as usize];
        first_jumps[pc as usize] += 1;
        pc += jmp;
        steps += 1;
    }

    println!("{} steps to reach the exit of the first star", steps);

    // Go through vector until jump goes past it's end
    pc = 0;
    steps = 0;
    let mut second_jumps = jumps;
    while pc < second_jumps.len() as i32 {
        let jmp = second_jumps[pc as usize];
        second_jumps[pc as usize] += if jmp > 2 { -1 } else { 1 };
        pc += jmp;
        steps += 1;
    }

    println!("and {} steps to reach the second", steps);
}
