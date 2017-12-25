// https://adventofcode.com/2017/day/22

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    // Parse starting field
    let input_field: Vec<char> = f.lines()
        .flat_map(|l| l.expect("Invalid line").chars().collect::<Vec<char>>())
        .collect();

    // Simulate first star
    let infections = simulate(10000, &input_field, false);
    // Assert to facilitate further tweaks
    assert_eq!(5570, infections);

    println!("{} bursts caused an infection in first star", infections);

    // Simulate second star
    let infections = simulate(10000000, &input_field, true);
    // Assert to facilitate further tweaks
    assert_eq!(2512022, infections);

    println!("{} bursts caused an infection in second star", infections);
}

// Runs given amount of bursts and returns count of bursts that caused an infection
// extended turns second star features on
fn simulate(bursts: usize, input_field: &Vec<char>, extended: bool) -> usize {
    let mut field = input_field.clone();

    // Set starting position to center facing up
    let mut dim = field_dim(&field);
    let mut x = dim / 2;
    let mut y = x;
    let mut dir = Dir::Up;

    // Run bursts
    let mut infections = 0;
    for _ in 0..bursts {
        // Extend field if carrier is on edge
        if x == 0 || y == 0 || x == dim - 1 || y == dim - 1 {
            let (mut new_field, new_dim, offset) = extend_field(&field);
            dim = new_dim;
            x += offset;
            y += offset;
            field = new_field;
        }

        // Check current node
        match field[y * dim + x] {
            // Carrier turns left on clean node
            // Gets infected on first star, becomes weakened on second
            '.' => {
                match dir {
                    Dir::Up => dir = Dir::Left,
                    Dir::Left => dir = Dir::Down,
                    Dir::Down => dir = Dir::Right,
                    Dir::Right => dir = Dir::Up,
                }
                field[y * dim + x] = if extended { 'W' } else { '#' };
                if !extended {
                    infections += 1;
                }
            }
            // Carrier turns right on infected node
            // Gets cleaned on first star, becomes flagged on second
            '#' => {
                match dir {
                    Dir::Up => dir = Dir::Right,
                    Dir::Right => dir = Dir::Down,
                    Dir::Down => dir = Dir::Left,
                    Dir::Left => dir = Dir::Up,
                }
                field[y * dim + x] = if extended { 'F' } else { '.' };
            }
            // Carrier continues forward on weakened node and node gets infected
            'W' => {
                field[y * dim + x] = '#';
                infections += 1;
            }
            // Carrier turns back on flagged node and node gets cleaned
            'F' => {
                match dir {
                    Dir::Up => dir = Dir::Down,
                    Dir::Down => dir = Dir::Up,
                    Dir::Right => dir = Dir::Left,
                    Dir::Left => dir = Dir::Right,
                }
                field[y * dim + x] = '.';
            }
            c => panic!("Invalid field char '{}'", c),
        }

        // Move
        match dir {
            Dir::Up => y -= 1,
            Dir::Right => x += 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
        }
    }

    infections
}

// Extends field in in all directions by (dim - 1) / 2
fn extend_field(field: &Vec<char>) -> (Vec<char>, usize, usize) {
    // Initialize new larger field
    let dim = field_dim(field);
    let new_dim = 2 * dim - 1;
    let mut new_field = vec!['.'; new_dim * new_dim];

    // Copy old field to center of new field
    let old_off = (dim - 1) / 2;
    for (oy, ny) in (old_off..(old_off + dim)).enumerate() {
        let old_start = oy * dim;
        let new_start = ny * new_dim + old_off;
        new_field.splice(
            (new_start)..(new_start + dim),
            field[(old_start)..(old_start + dim)].iter().cloned(),
        );
    }

    (new_field, new_dim, old_off)
}

// Returns the lengh of the field's side
fn field_dim(field: &Vec<char>) -> usize {
    (field.len() as f64).sqrt() as usize
}

#[allow(dead_code)]
fn print_field(field: &Vec<char>, x: usize, y: usize, dim: usize) {
    for j in 0..dim {
        let line = j * dim;
        for i in 0..dim {
            if i == 0 {
                print!("{}", if j == y && x == 0 { '[' } else { ' ' });
            }
            // Print with carrier
            if j == y && i == x {
                print!("{}]", field[line + i]);
            } else if j == y && i + 1 == x {
                print!("{}[", field[line + i]);
            } else {
                print!("{} ", field[line + i]);
            }
        }
        println!();
    }
    println!();
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
