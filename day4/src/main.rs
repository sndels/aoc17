// https://adventofcode.com/2017/day/4

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));
    let lines: Vec<String> = f.lines()
        .map(|l| l.expect("Reading lines failed"))
        .collect();

    // First star
    let mut valid_first = 0;
    for line in &lines {
        // Get list of words without the last one
        let mut words: Vec<&str> = line.split(' ').collect();
        words.pop();

        valid_first += 1;
        // Try to find each word in the remaining line
        let mut offset = 0;
        for (i, word) in words.iter().enumerate() {
            offset += words[i].len() + 1;
            if line[offset..].find(word) != None {
                // Word found in remaining phrase, phrase is invalid
                valid_first -= 1;
                break;
            }
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(477, valid_first);

    // Second star
    let mut valid_second = 0;
    'phrases: for line in &lines {
        // Save lengths and character counts for each word in phrase
        let mut counts: Vec<(usize, Vec<u32>)> = Vec::new();
        for word in line.split(' ') {
            let mut chars: Vec<u32> = vec![0; 26];
            for c in word.chars() {
                chars[(c as u8 - 'a' as u8) as usize] += 1;
            }
            counts.push((word.len(), chars));
        }

        // Check every word against remaining words
        for (i, ref_count) in counts.iter().enumerate() {
            'remaining_words: for count in &counts[(i + 1)..counts.len()] {
                // Only check if lengths match
                if ref_count.0 == count.0 {
                    for c in count.1.iter().zip(ref_count.1.iter()) {
                        if c.0 != c.1 {
                            // Words are not anagrams, check next word
                            continue 'remaining_words;
                        }
                    }
                    // Phrase is invalid, continue to next phrase
                    continue 'phrases;
                }
            }
        }
        // No matches found
        valid_second += 1;
    }
    // Assert to facilitate further tweaks
    assert_eq!(167, valid_second);

    println!("{}, {} valid passpharases", valid_first, valid_second);
}
