// https://adventofcode.com/2017/day/8

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn check_cnd(split: &Vec<&str>, registers: &mut HashMap<String, i32>) -> bool {
    let lhs = registers.entry(split[4].to_string()).or_insert(0);
    let rhs = split[6].parse::<i32>().unwrap();
    match split[5] {
        "<" => *lhs < rhs,
        "<=" => *lhs <= rhs,
        "==" => *lhs == rhs,
        "!=" => *lhs != rhs,
        ">=" => *lhs >= rhs,
        ">" => *lhs > rhs,
        _ => panic!("incorrect operator \"{}\"", split[5]),
    }
}

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    let mut largest_during = 0;
    let mut registers = HashMap::new();
    for line in f.lines() {
        let raw_line = line.expect("Reading line failed");
        let split: Vec<&str> = raw_line.split(' ').collect();

        if check_cnd(&split, &mut registers) {
            let target_reg = registers.entry(split[0].to_string()).or_insert(0);
            let amount = split[2].parse::<i32>().unwrap();

            *target_reg = match split[1] {
                "inc" => *target_reg + amount,
                "dec" => *target_reg - amount,
                _ => panic!("incorrect instruction \"{}\"", split[1]),
            };

            if *target_reg > largest_during {
                largest_during = *target_reg;
            }
        }
    }

    let mut largest_after = 0;
    for &v in registers.values() {
        if v > largest_after {
            largest_after = v;
        }
    }

    println!("Largest value after execution is {}", largest_after);
    println!("Largest value during execution was {}", largest_during);
}
