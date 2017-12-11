// https://adventofcode.com/2017/day/11

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").expect("Opening input.txt failed");
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    // Go through steps in hex-coordinates
    let mut nw = 0;
    let mut n = 0;
    let mut ne = 0;
    let mut longest_dist = 0;
    for dir in input.split(',') {
        match dir {
            "n" => n += 1,
            "ne" => ne += 1,
            "se" => nw -= 1,
            "s" => n -= 1,
            "sw" => ne -= 1,
            "nw" => nw += 1,
            _ => panic!("Invalid direction"),
        }
        let dist = get_hexdist(nw, n, ne);
        if dist > longest_dist {
            longest_dist = dist;
        }
    }

    println!(
        "Child is {} steps away (nw {} n {} ne {})",
        get_hexdist(nw, n, ne),
        nw,
        n,
        ne
    );
    println!("Child was at most {} steps away", longest_dist);
}

fn get_hexdist(nw: i32, n: i32, ne: i32) -> i32 {
    let a_nw = nw.abs();
    let a_n = n.abs();
    let a_ne = ne.abs();
    if a_nw < a_n && a_nw < a_ne {
        (n + ne).abs()
    } else if a_n < a_nw && a_n < a_ne {
        (nw + ne).abs()
    } else {
        (nw + n).abs()
    }
}
