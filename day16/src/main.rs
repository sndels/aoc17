// https://adventofcode.com/2017/day/16

#![feature(slice_rotate)]

extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use regex::RegexSet;

fn main() {
    // Create needed regex for parsing input
    let move_regs = RegexSet::new(&[r"s\d{1,2}", r"x\d{1,2}/\d{1,2}", r"p\D{1}/\D{1}"]).unwrap();
    let spin_reg = Regex::new(r"s(\d{1,2})").unwrap();
    let exchange_reg = Regex::new(r"x(\d{1,2})/(\d{1,2})").unwrap();
    let partner_reg = Regex::new(r"p(\D{1})/(\D{1})").unwrap();

    // Read input to string
    let mut f = File::open("input.txt").expect("Opening input.txt failed");
    let mut raw_input = String::new();
    f.read_to_string(&mut raw_input)
        .expect("Reading line failed");
    let raw_input = raw_input;

    // Parse moves
    let mut moves = Vec::new();
    for m in raw_input.split(',') {
        // Check which move it is
        let matches = move_regs.matches(m).into_iter().collect::<Vec<usize>>();
        match matches.get(0).expect("Invalid move") {
            &0 => {
                // Parse spin
                let cap = spin_reg.captures_iter(m).next().unwrap();
                let size = cap[1].parse::<usize>().expect("Invalid spin");
                moves.push((Move::Spin, size, 0));
            }
            &1 => {
                // Parse exchange
                let cap = exchange_reg.captures_iter(m).next().unwrap();
                let a = cap[1].parse::<usize>().expect("Invalid first exchange");
                let b = cap[2].parse::<usize>().expect("Invalid second exchange");
                moves.push((Move::Exchange, a, b));
            }
            &2 => {
                // Parse partner and store as usize
                let cap = partner_reg.captures_iter(m).next().unwrap();
                let a = cap[1].chars().next().expect("Invalid first partner") as usize;
                let b = cap[2].chars().next().expect("Invalid second partner") as usize;
                moves.push((Move::Partner, a, b));
            }
            _ => unreachable!(),
        }
    }

    // Perform first "dance" for list of programs
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut programs = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    dance(&mut programs, &moves);

    print!("Programs after first dance: ");
    programs.iter().for_each(|&p| print!("{}", p));
    println!();

    // Measure the cycle of the dance
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let ordered_programs = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    let mut cycle = 1;
    while programs != ordered_programs {
        dance(&mut programs, &moves);
        cycle += 1;
    }

    // Perform the remainder of one billion / cycle dances
    (0..(1000000000 % cycle)).for_each(|_| dance(&mut programs, &moves));

    print!("Programs after a billion dances: ");
    programs.iter().for_each(|&p| print!("{}", p));
    println!();
}

fn dance(programs: &mut [char; 16], moves: &Vec<(Move, usize, usize)>) {
    for &(ref m, ref a, ref b) in moves {
        match *m {
            Move::Spin => programs.rotate(16 - *a),  // Rotate right by a
            Move::Exchange => programs.swap(*a, *b), // Swap the elements
            Move::Partner => {
                // Find program positions and swap the elements
                let a_pos = programs
                    .iter()
                    .position(|&c| c == *a as u8 as char)
                    .expect("Invalid first partner");
                let b_pos = programs
                    .iter()
                    .position(|&c| c == *b as u8 as char)
                    .expect("Invalid second partner");
                programs.swap(a_pos, b_pos);
            }
        }
    }
}

enum Move {
    Spin,
    Exchange,
    Partner,
}
