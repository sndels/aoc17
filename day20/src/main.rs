// https://adventofcode.com/2017/day/19

extern crate regex;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;
use std::iter::FromIterator;
use regex::Regex;

fn main() {
    // Set up regex for matching the different vectors, concat! as raws don't support multiline
    let particle_reg = Regex::new(concat!(
        r"p=<(\D*\d+),(\D*\d+),(\D*\d+)>, ",
        r"v=<(\D*\d+),(\D*\d+),(\D*\d+)>, ",
        r"a=<(\D*\d+),(\D*\d+),(\D*\d+)>"
    )).unwrap();
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    // Parse input
    let mut particles = Vec::new();
    for line in f.lines() {
        let raw_line = line.expect("");
        let vec_caps = particle_reg
            .captures_iter(&raw_line)
            .nth(0)
            .expect("Invalid particle") // Get first and only match of the regex
            .iter()
            .skip(1) // Skip first element as it holds the entire match
            .map(|i| i.unwrap().as_str().parse::<i32>().unwrap()) // Parse subsequent matches to i32
            .collect::<Vec<i32>>();

        // Push pos, vel, acc, alive for particle
        particles.push((
            [vec_caps[0], vec_caps[1], vec_caps[2]],
            [vec_caps[3], vec_caps[4], vec_caps[5]],
            [vec_caps[6], vec_caps[7], vec_caps[8]],
        ));
    }

    // Simulate system until all particles are accelerating away from origin and
    // no recent collisions have happened
    let mut some_decelerating = true;
    let mut some_closing_in = true;
    let mut last_collision = 0;
    let mut alive: HashSet<usize> = HashSet::from_iter(0..particles.len());
    while some_decelerating || some_closing_in || last_collision < 20 {
        some_decelerating = false;
        some_closing_in = false;
        last_collision += 1;
        println!("Tick {} since collision", last_collision);
        // Simulate
        for &i in &alive {
            let p = &mut particles[i];
            let old_vel = p.1.clone();
            let old_pos = p.0.clone();
            p.1 = sum3(&p.1, &p.2);
            p.0 = sum3(&p.0, &p.1);
            if mlen(&old_vel) > mlen(&p.1) {
                some_decelerating = true;
            }
            if mlen(&old_pos) > mlen(&p.0) {
                some_closing_in = true;
            }
        }

        // Handle collisions
        let mut new_alive = alive.clone();
        for &i in &alive {
            for &j in &alive {
                if i != j {
                    if particles[i].0 == particles[j].0 {
                        new_alive.remove(&i);
                        new_alive.remove(&j);
                        last_collision = 0;
                    }
                }
            }
        }
        if last_collision == 0 {
            alive = new_alive;
        }
    }

    // Get particle with smallest acceleration (smallest speed, closest to origin if multiple)
    // This might not be strictly correct (should probably compare manhattan distance changing
    // speed as velocity and increase of that as acceleration), but it already worked ":D"
    let mut closest = (0, std::u32::MAX, std::u32::MAX, std::u32::MAX);
    for (i, p) in particles.iter().enumerate() {
        let pos = mlen(&p.0);
        let vel = mlen(&p.1);
        let acc = mlen(&p.2);
        if acc < closest.3 {
            closest = (i, pos, vel, acc);
        } else if acc == closest.3 {
            if vel < closest.2 {
                closest = (i, pos, vel, acc);
            } else if vel == closest.2 {
                if pos < closest.1 {
                    closest = (i, pos, vel, acc);
                }
            }
        }
    }
    println!("Particle {} will stay closest", closest.0);
    println!("{} particles remain", alive.len());
}

fn sum3(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn mlen(v: &[i32; 3]) -> u32 {
    v.iter().fold(0u32, |acc, &c| acc + c.abs() as u32)
}
