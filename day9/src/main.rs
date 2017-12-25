// https://adventofcode.com/2017/day/9

use std::io::prelude::*;
use std::fs::File;

fn main() {
    // Read input into string
    let mut f = File::open("input.txt").expect("Opening input.txt failed");
    let mut raw_input = String::new();
    f.read_to_string(&mut raw_input)
        .expect("Reading input to string failed");

    // Iterate through the input counting group scores and "cleaned" garbage
    let mut score = 0;
    let mut garbage_chars = 0;
    let mut group_val = 0;
    let mut in_garbage = false;
    let mut input_iter = raw_input.chars();
    while let Some(c) = input_iter.next() {
        if in_garbage {
            match c {
                // End garbage
                '>' => in_garbage = false,
                // Escape next character
                '!' => {
                    input_iter.next();
                }
                // Default
                _ => garbage_chars += 1,
            }
        } else {
            match c {
                // Begin new group
                '{' => {
                    group_val += 1;
                    score += group_val;
                }
                // End current group
                '}' => group_val -= 1,
                // Begin garbage
                '<' => in_garbage = true,
                // Default
                _ => {}
            }
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(11898, score);
    assert_eq!(5601, garbage_chars);

    println!("Total score of {}", score);
    println!("{} chars of non-escaped garbage removed", garbage_chars);
}
