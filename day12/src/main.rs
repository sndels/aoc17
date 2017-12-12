// https://adventofcode.com/2017/day/12

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // Read input to a map of name -> connections
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));
    let mut programs = HashMap::new();
    for line in f.lines() {
        let raw_line = line.expect("Reading line failed");

        // Split node info and connection list
        let main_split: Vec<&str> = raw_line.split(" <-> ").collect();
        let name = main_split[0].parse::<u16>().expect("Invalid program");

        // Get list of connections
        let connections = match main_split.get(1) {
            Some(split) => split
                .split(", ")
                .map(|c| c.parse::<u16>().expect("Invalid connection"))
                .collect(),
            None => Vec::new(),
        };

        // Insert to map
        programs.insert(name, connections);
    }
    let programs = programs;

    // First star

    println!(
        "There are {} nodes in program 0's group",
        find_connected(0, &programs).len()
    );

    // Second star

    let mut visited: HashSet<u16> = HashSet::new();
    let mut groups = 0;

    // Run dfs through the whole map
    for (&node, _) in &programs {
        if !visited.contains(&node) {
            // Found a new group
            groups += 1;
            // Find programs in group
            visited.extend(find_connected(node, &programs).iter());
        }
    }

    println!("There are {} groups in total", groups);
}

fn find_connected(start: u16, map: &HashMap<u16, Vec<u16>>) -> HashSet<u16> {
    // Init on start key
    let mut connected = HashSet::new();
    let mut node_stack = Vec::new();
    node_stack.push(start);

    // Do dfs until all connected nodes are found
    while let Some(node) = node_stack.pop() {
        connected.insert(node);
        map.get(&node).expect("Invalid node").iter().for_each(|&n| {
            if !connected.contains(&n) {
                node_stack.push(n)
            }
        });
    }

    connected
}
