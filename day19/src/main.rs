// https://adventofcode.com/2017/day/19

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    // Dump routing diagram to table
    let diagram = f.lines()
        .map(|line| line.expect("Invalid line").chars().collect())
        .collect::<Vec<Vec<char>>>();

    // Find starting position
    let mut y = 0;
    let mut x = diagram[0]
        .iter()
        .position(|&c| c == '|')
        .expect("No '|' on first line");
    let mut dir = Dir::Down;

    // Follow route and count steps
    let mut letters = String::new();
    let mut steps = 1;
    while dir != Dir::Stop {
        steps += 1;
        println!("At {} {} with value {}", x, y, diagram[y][x]);
        // Check current character
        match diagram[y][x] {
            // Keep track of letters and check if packet should stop
            c @ 'A'...'Z' => {
                letters.push(c);
                dir = check_stop(x, y, dir, &diagram);
            }
            // Find next direction at corner
            '+' => dir = check_dir(x, y, dir, &diagram),
            // Straight pipes won't affect direction
            '|' | '-' => {}
            c @ _ => panic!("Invalid diagram character {}", c),
        }

        // Move one step to current direction
        match dir {
            Dir::Up => y -= 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
            Dir::Stop => {}
        }
    }

    println!(
        "The packet saw letters '{}' and took {} steps",
        letters,
        steps
    );
}

fn check_dir(x: usize, y: usize, dir: Dir, diagram: &Vec<Vec<char>>) -> Dir {
    // Check if packet shoud continue up
    if dir != Dir::Down && diagram[y.saturating_sub(1)][x] == '|' {
        return Dir::Up;
    }
    // Check if packet shoud continue down
    if dir != Dir::Up && diagram[y.saturating_add(1)][x] == '|' {
        return Dir::Down;
    }
    // Check if packet shoud continue left
    if dir != Dir::Right && diagram[y][x.saturating_sub(1)] == '-' {
        return Dir::Left;
    }
    // Expect correct input so should go right
    return Dir::Right;
}

fn check_stop(x: usize, y: usize, dir: Dir, diagram: &Vec<Vec<char>>) -> Dir {
    // Get next character on line
    let next = match dir {
        Dir::Up => diagram[y - 1][x],
        Dir::Down => diagram[y + 1][x],
        Dir::Left => diagram[y][x - 1],
        Dir::Right => diagram[y][x + 1],
        Dir::Stop => unreachable!(),
    };

    // Check if packet can continue
    match next {
        // Can continue
        'A'...'Z' | '|' | '-' | '+' => return dir,
        // Otherwise stops
        _ => return Dir::Stop,
    }
}

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Stop,
}
