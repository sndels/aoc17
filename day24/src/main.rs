// https://adventofcode.com/2017/day/24

use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    // Parse components
    let mut components = Vec::new();
    for line in f.lines() {
        let raw_line = line.expect("Reading line failed");
        let mut halves = raw_line.split('/');
        components.push((
            halves
                .next()
                .expect("Invalid component")
                .parse::<u32>()
                .expect("Invalid port"),
            halves
                .next()
                .expect("Invalid component")
                .parse::<u32>()
                .expect("Invalid port"),
        ));
    }

    // Get strength of the strongest bridge
    let strength = get_max_strength(0, 0, HashSet::from_iter(components.iter().cloned()));
    // Assert to facilitate further tweaks
    assert_eq!(1511, strength);

    println!("Strength of the strongest bridge is {}", strength);

    // Get strength of the longest (strongest of if multiple) bridge
    let strength =
        get_longest_max_strength(0, 0, 0, HashSet::from_iter(components.iter().cloned())).1;
    // Assert to facilitate further tweaks
    assert_eq!(1471, strength);

    println!("Max strength of the longest bridge is {}", strength);
}

// Finds the max strength achievable from the open port with unused components
fn get_max_strength(strength: u32, open_port: u32, unused_components: HashSet<(u32, u32)>) -> u32 {
    let mut max_strength = strength;

    for c in &unused_components {
        // Chech if component is viable
        if c.0 == open_port || c.1 == open_port {
            // Create copy of unused components without the match
            let mut new_components = unused_components.clone();
            new_components.remove(c);
            // Find max strength from the match
            let new_open = if c.0 == open_port { c.1 } else { c.0 };
            let new_strength = get_max_strength(strength + c.0 + c.1, new_open, new_components);
            if new_strength > max_strength {
                max_strength = new_strength;
            }
        }
    }

    max_strength
}


// Finds the max strength of the longest achievable bridge from the open port with unused components
fn get_longest_max_strength(
    length: u32,
    strength: u32,
    open_port: u32,
    unused_components: HashSet<(u32, u32)>,
) -> (u32, u32) {
    let mut max_length = length;
    let mut max_strength = strength;

    for c in &unused_components {
        // Chech if component is viable
        if c.0 == open_port || c.1 == open_port {
            // Create copy of unused components without the match
            let mut new_components = unused_components.clone();
            new_components.remove(c);
            // Find longest bridge with max strength from the match
            let new_open = if c.0 == open_port { c.1 } else { c.0 };
            let (new_length, new_strength) = get_longest_max_strength(
                length + 1,
                strength + c.0 + c.1,
                new_open,
                new_components,
            );
            if new_length >= max_length && new_strength > max_strength {
                max_length = new_length;
                max_strength = new_strength;
            }
        }
    }

    (max_length, max_strength)
}
