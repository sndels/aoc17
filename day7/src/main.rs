// https://adventofcode.com/2017/day/7

extern crate regex;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    // Regex for matching program and weight
    let node_re = Regex::new(r"(\D+[a-z]) \((\d+)\)").unwrap();

    // Read input to a set of name -> (weight, children)
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));
    let mut programs = HashMap::new();
    for line in f.lines() {
        let raw_line = line.expect("Reading line failed");

        // Split node info and children list
        let main_split: Vec<&str> = raw_line.split(" -> ").collect();

        // Match regex on node info
        let cap = node_re.captures_iter(main_split[0]).last().unwrap();
        let name = cap[1].to_string();
        let weight = cap[2].parse::<i32>().unwrap();
    
        // Get list of children
        let children = match main_split.get(1) {
            Some(split) => split
                .split(", ")
                .map(|child| child.to_string())
                .collect(),
            None => Vec::new(),
        };

        // Insert to map
        programs.insert(name, (weight, children));
    }
    let programs = programs;

    // Find root
    let mut all_programs = HashSet::new();
    for key in programs.keys() {
        all_programs.insert(key);
    }
    for (_, value) in programs.iter() {
        if value.1.len() > 0 {
            // Children can't be root
            for child in &value.1 {
                all_programs.remove(child);
            }
        }
    }
    let root = all_programs.iter().last().unwrap();
    println!("The root is {}", root);
}
