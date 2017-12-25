// https://adventofcode.com/2017/day/1

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read input to string
    let mut f = File::open("input.txt").expect("Opening input.txt failed");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("Input read failed!");
    let input = input;

    // Parse input to a vector of ints
    let digits: Vec<u32> = input
        .chars()
        .map(|x| x.to_digit(10).expect("Invalid num char in input"))
        .collect();

    // Sum of all digits that match the next digit
    let mut first_sum: u32 = 0;
    for i in 1..digits.len() {
        if digits[i - 1] == digits[i] {
            first_sum += digits[i];
        }
    }
    // Last is compared with the first
    if digits.first() == digits.last() {
        first_sum += digits.first().expect("List of digits was empty");
    }
    // Assert to facilitate further tweaks
    assert_eq!(1203, first_sum);

    // Sum of all digits that match the one halfway around the list
    let mut second_sum: u32 = 0;
    let offset = digits.len() / 2;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + offset) % digits.len()] {
            second_sum += digits[i];
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(1146, second_sum);

    println!(
        "First captcha solution is {} and the second {}",
        first_sum,
        second_sum
    );
}
