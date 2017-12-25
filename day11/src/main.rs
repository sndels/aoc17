// https://adventofcode.com/2017/day/11

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").expect("Opening input.txt failed");
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    // Go through steps in cartesian coordinates
    let mut x = 0;
    let mut y = 0;
    let mut longest_dist = 0;
    for dir in input.split(',') {
        let (dx, dy) = match dir {
            "n" => (0, 2),
            "s" => (0, -2),
            "nw" => (-2, 1),
            "ne" => (2, 1),
            "sw" => (-2, -1),
            "se" => (2, -1),
            _ => panic!("Invalid direction"),
        };
        x += dx;
        y += dy;

        // Check if current cell is further away than previous max
        let (nw, n, ne) = get_hex(x, y);
        let dist = nw.abs() + n.abs() + ne.abs();
        if dist > longest_dist {
            longest_dist = dist;
        }
    }

    let (nw, n, ne) = get_hex(x, y);
    let dist = nw.abs() + n.abs() + ne.abs();
    // Assert to facilitate further tweaks
    assert_eq!(805, dist);
    assert_eq!(1535, longest_dist);

    println!("Child is {} steps away (nw {} n {} ne {})", dist, nw, n, ne);
    println!("Child was at most {} steps away", longest_dist);
}

fn get_hex(x: i32, y: i32) -> (i32, i32, i32) {
    let nw;
    let n;
    let ne;

    // Convert cartesian to hex-grid
    if x == 0 {
        nw = 0;
        n = y / 2;
        ne = 0;
    } else if y == 0 {
        nw = -x / 4;
        n = 0;
        ne = x / 4;
    } else if (y > 0 && x > 0) || (y < 0 && x < 0) {
        nw = 0;
        ne = x / 2;
        n = (y - ne) / 2;
    } else {
        nw = -x / 2;
        n = (y - nw) / 2;
        ne = 0
    }

    (nw, n, ne)
}
