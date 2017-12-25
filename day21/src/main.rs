// https://adventofcode.com/2017/day/21

#![feature(iterator_step_by)]

use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    // Parse patterns
    let mut rules2 = Vec::new();
    let mut rules3 = Vec::new();
    for line in f.lines() {
        // Parse line
        let raw_line = line.expect("");
        let halves: Vec<&str> = raw_line.split(" => ").collect();
        let pattern: Vec<char> = halves[0].replace("/", "").chars().collect();
        let result: Vec<char> = halves[1].replace("/", "").chars().collect();

        // Generate all unique permutations of the pattern
        let patterns = gen_perms(&pattern);

        // Push pattern permutations and resulting sub image to correct vectors
        match pattern.len() {
            4 => {
                rules2.push((patterns, result));
            }
            9 => {
                rules3.push((patterns, result));
            }
            _ => panic!("Invalid pattern"),
        }
    }
    let rules2 = rules2;
    let rules3 = rules3;

    // Initialize image
    let mut image: Vec<char> = ".#...####".chars().collect();

    // Run first 5 iterations
    for _ in 0..5 {
        image = enhance(&image, &rules2, &rules3);
    }

    // Count pixels that are on
    let on_pixels = image
        .iter()
        .fold(0, |acc, &c| if c == '#' { acc + 1 } else { acc });
    // Assert to facilitate further tweaks
    assert_eq!(197, on_pixels);

    println!("{} pixels on after 5 iterations", on_pixels);

    // Run rest of 18 iterations
    for _ in 5..18 {
        image = enhance(&image, &rules2, &rules3);
    }

    // Count pixels that are on
    let on_pixels = image
        .iter()
        .fold(0, |acc, &c| if c == '#' { acc + 1 } else { acc });
    // Assert to facilitate further tweaks
    assert_eq!(3081737, on_pixels);

    println!(
        "{} pixels on after 18 iterations",
        image
            .iter()
            .fold(0, |acc, &c| if c == '#' { acc + 1 } else { acc })
    );
}

fn enhance(
    image: &Vec<char>,
    rules2: &Vec<(Vec<Vec<char>>, Vec<char>)>,
    rules3: &Vec<(Vec<Vec<char>>, Vec<char>)>,
) -> Vec<char> {
    // Get info on current image
    let dim = (image.len() as f64).sqrt() as usize;
    let sub_dim = if image.len() % 2 == 0 { 2 } else { 3 };

    // Figure out contents of next image
    let mut new_squares = Vec::new();
    for y in (0..dim).step_by(sub_dim) {
        for x in (0..dim).step_by(sub_dim) {
            new_squares.push(get_perm(
                &gen_sub_image(x, y, sub_dim, &image, dim),
                if sub_dim == 2 { &rules2 } else { &rules3 },
            ));
        }
    }
    // Generate next image
    if sub_dim == 2 {
        gen_image(&new_squares, &rules2)
    } else {
        gen_image(&new_squares, &rules3)
    }
}

// Returns index of rule matching the sub image
fn get_perm(sub_image: &Vec<char>, rules: &Vec<(Vec<Vec<char>>, Vec<char>)>) -> usize {
    for (i, rule) in rules.iter().enumerate() {
        for perm in &rule.0 {
            let matching = sub_image
                .iter()
                .zip(perm.iter())
                .filter(|&(a, b)| a == b)
                .count();
            if matching == sub_image.len() {
                return i;
            }
        }
    }
    panic!("Sub image doesn't match any rule permutation");
}

// Returns all unique permutations of the given image
fn gen_perms(image: &Vec<char>) -> Vec<Vec<char>> {
    let mut pattern = image.clone();
    let mut unique = HashSet::new();

    // Generate and test permutations of the pattern
    let mut patterns = Vec::new();
    if unique.insert(pattern.iter().cloned().collect::<String>()) {
        patterns.push(pattern.clone());
    }
    for _ in 0..3 {
        pattern = flip_xy(&pattern);
        if unique.insert(pattern.iter().cloned().collect::<String>()) {
            patterns.push(pattern.clone());
        }
        pattern = mirr_x(&pattern);
        if unique.insert(pattern.iter().cloned().collect::<String>()) {
            patterns.push(pattern.clone());
        }
    }
    pattern = flip_xy(&pattern);
    if unique.insert(pattern.iter().cloned().collect::<String>()) {
        patterns.push(pattern.clone());
    }

    patterns
}

// Returns copy of given sub image
fn gen_sub_image(x: usize, y: usize, sub_dim: usize, image: &Vec<char>, dim: usize) -> Vec<char> {
    let mut sub_image = Vec::new();

    // Copy the sub image to a new vector
    for j in y..(y + sub_dim) {
        for i in x..(x + sub_dim) {
            sub_image.push(image[j * dim + i])
        }
    }

    sub_image
}

// Returns new image generated from the given list of sub images
fn gen_image(sub_images: &Vec<usize>, rules: &Vec<(Vec<Vec<char>>, Vec<char>)>) -> Vec<char> {
    // Get dimension of the new image in sub images
    let dim_subs = (sub_images.len() as f64).sqrt() as usize;
    // Get dimension of the sub images
    let sub_dim = (rules[0].1.len() as f64).sqrt() as usize;

    let mut image = Vec::new();
    // Loop over all rows of sub images
    for sub_line in (0..sub_images.len()).step_by(dim_subs) {
        // Print sub images row by row
        for line in 0..sub_dim {
            let row_range = (line * sub_dim)..((line + 1) * sub_dim);
            for sub_image in 0..dim_subs {
                image.extend(
                    rules[sub_images[sub_line + sub_image]].1[row_range.clone()]
                        .iter()
                        .cloned(),
                );
            }
        }
    }

    image
}

// Prints the given image
#[allow(dead_code)]
fn print_image(image: &Vec<char>) {
    let dim = (image.len() as f64).sqrt() as usize;
    image.chunks(dim).for_each(|l| {
        l.iter().for_each(|&c| print!("{}", c));
        println!("");
    });
}

// Returns the sub image mirrored horizontally
fn mirr_x(sub_image: &Vec<char>) -> Vec<char> {
    let dim = if sub_image.len() == 4 { 2 } else { 3 };
    let mut new_sub = Vec::new();

    for line in (0..(dim * dim)).step_by(dim) {
        for x in (0..dim).rev() {
            new_sub.push(sub_image[line + x]);
        }
    }

    new_sub
}

// Returns the sub image with x and y axis swapped
fn flip_xy(sub_image: &Vec<char>) -> Vec<char> {
    let dim = if sub_image.len() == 4 { 2 } else { 3 };
    let mut new_sub = Vec::new();

    for x in 0..dim {
        for y in 0..dim {
            new_sub.push(sub_image[y * dim + x]);
        }
    }

    new_sub
}
