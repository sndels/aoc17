// https://adventofcode.com/2017/day/1

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    let mut digits: Vec<u32> = Vec::new();
    for line in f.lines() {
        digits.extend(
            line.expect("Reading line failed")
                .chars()
                .map(|x| x.to_digit(10).expect("Invalid num char in input")),
        );
    }
    let digits = digits;

    let mut first_sum: u32 = 0;
    for i in 1..digits.len() {
        if digits[i - 1] == digits[i] {
            first_sum += digits[i];
        }
    }
    if digits.first() == digits.last() {
        first_sum += digits.first().expect("List of digits was empty");
    }

    let mut second_sum: u32 = 0;
    let offset = digits.len() / 2;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + offset) % digits.len()] {
            second_sum += digits[i];
        }
    }

    println!(
        "First captcha solution is {} and the second {}",
        first_sum,
        second_sum
    );
}
