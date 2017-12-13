// https://adventofcode.com/2017/day/13

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    // Read input to a map of name -> connections
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));
    let mut layers = Vec::new();
    for line in f.lines() {
        let raw_line = line.expect("Reading line failed");
        let split: Vec<&str> = raw_line.split(": ").collect();

        let layer_num = split[0].parse::<usize>().unwrap();
        let layer_depth = split[1].parse::<usize>().unwrap();
        while layers.len() < layer_num {
            layers.push(0);
        }
        layers.push(layer_depth);
    }
    let layers = layers;

    println!(
        "Severity of the straight trip is {}",
        check_severity(0, &layers)
    );

    let mut delay = 0;
    while !check_success(delay, &layers) {
        delay += 1;
    }

    println!("Shortest delay to get through unscathed is {} ps", delay);
}

fn check_severity(delay: usize, layers: &Vec<usize>) -> usize {
    let mut severity = 0;
    for (i, &depth) in layers.iter().enumerate() {
        if depth > 0 {
            // Check if scanner is at 0
            let scanner_cycle = (i + delay) % (2 * depth - 2);
            if scanner_cycle == 0 {
                severity += i * depth;
            }
        }
    }
    severity
}

fn check_success(delay: usize, layers: &Vec<usize>) -> bool {
    for (i, &depth) in layers.iter().enumerate() {
        if depth > 0 {
            // Check if scanner is at 0
            let scanner_cycle = (i + delay) % (2 * depth - 2);
            if scanner_cycle == 0 {
                return false;
            }
        }
    }
    true
}
