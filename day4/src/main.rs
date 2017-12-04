// https://adventofcode.com/2017/day/4

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    let mut valid_passphrases = 0;
    for line in f.lines() {
        let raw_line = line.expect("Reading line failed");

        // Get a split of words without the redundant last one
        let mut words: Vec<&str> = raw_line.split(' ').collect();
        words.pop();

        // Check if some of the words occurs more than once
        valid_passphrases += 1;
        let mut offset = 0;
        for (i, word) in words.iter().enumerate() {
            offset += words[i].len() + 1;
            if raw_line[offset..].find(word) != None {
                valid_passphrases -= 1;
                break;
            }
        }
    }
    println!("{} valid passpharases", valid_passphrases);
}
