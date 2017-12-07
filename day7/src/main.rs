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

    // Read input to a map of name -> (weight, children) and set of all names
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));
    let mut program_nodes = HashMap::new();
    let mut program_names = HashSet::new();
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
            Some(split) => split.split(", ").map(|child| child.to_string()).collect(),
            None => Vec::new(),
        };

        // Insert to map
        program_nodes.insert(name.clone(), (weight, children));
        program_names.insert(name);
    }
    let program_nodes = program_nodes;

    // First star

    // Find root by removing all children from set of names
    for (_, value) in program_nodes.iter() {
        if value.1.len() > 0 {
            // Children can't be root
            for child in &value.1 {
                program_names.remove(child);
            }
        }
    }
    let root = program_names.into_iter().last().unwrap();

    println!("The root is {}", root);

    // Second star

    // Init DFS with root
    let mut node_stack = Vec::new();
    let mut weight_stack = Vec::new();
    node_stack.push(vec![root; 1]);

    // Do dfs while accumulating stack sums until the incorrect node is found
    'dfs: while node_stack.len() > 0 {
        // Check if unhandled nodes remain in topmost list
        if node_stack.last_mut().unwrap().len() > 0 {
            // Get the next node
            let next_name = node_stack.last_mut().unwrap().last().unwrap().clone();
            let next_info = program_nodes.get(&next_name).unwrap();
            if next_info.1.len() > 0 {
                // Parents
                // Push children to stack
                node_stack.push(next_info.1.clone());
                // Init weights for this node's children
                weight_stack.push(Vec::new());
            } else {
                // Leaf nodes
                // Push own weight to topmost weights
                weight_stack
                    .last_mut()
                    .unwrap()
                    .push((next_info.0, next_info.0));
                // Pop self from nodes
                node_stack.last_mut().unwrap().pop().unwrap();
            }
        } else {
            // Pop empty node list
            node_stack.pop().unwrap();
            // Pop weights
            let weights = weight_stack.pop().unwrap();
            // Pop parent the children weights belong to
            let parent_name = node_stack.last_mut().unwrap().pop().unwrap();

            // Compare children's weights
            for i in 1..(weights.len() - 1) {
                if weights[i - 1].1 != weights[i].1 {
                    let corrected_weight = if weights[i].1 != weights[i + 1].1 {
                        // Current weight is incorrect
                        weights[i].0 + (weights[i - 1].1 - weights[i].1)
                    } else {
                        // Previous weight is incorrect
                        weights[i - 1].0 + (weights[i].1 - weights[i - 1].1)
                    };
                    println!("Corrected weight should be {}", corrected_weight);
                    break 'dfs;
                }
            }

            // Calculate combined stack weight and push to parent's parent's weight frame
            let weight = program_nodes.get(&parent_name).unwrap().0;
            weight_stack
                .last_mut()
                .unwrap()
                .push((weight, weight + weights[0].1 * weights.len() as i32));
        }
    }
}
