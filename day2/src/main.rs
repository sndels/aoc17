// https://adventofcode.com/2017/day/2

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input.txt").expect("Opening input.txt failed"));

    let mut rows: Vec<Vec<u32>> = f.lines()
        .map(|l| {
            l.expect("Reading line failed")
                .split('\t')
                .map(|x| x.parse::<u32>().expect("Invalid number"))
                .collect()
        })
        .collect();
    // Sort to make second part nicer, also trivializes first part
    rows.iter_mut().for_each(|row| row.sort_unstable());
    let rows = rows;

    // Sum sums of the smallest and largest values per row
    let mut first_checksum: u32 = 0;
    for row in &rows {
        first_checksum += row.last().expect("Row Empty") - row.first().expect("Row Empty");
    }
    // Assert to facilitate further tweaks
    assert_eq!(42378, first_checksum);

    // Sum division results of the two values per row that divide evenly
    let mut second_checksum: u32 = 0;
    for row in &rows {
        'row_loop: for i in 0..row.len() {
            for j in (i + 1)..row.len() {
                if row[j] % row[i] == 0 {
                    second_checksum += row[j] / row[i];
                    break 'row_loop;
                }
            }
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(246, second_checksum);

    println!(
        "The first checksum is {} and the second {}",
        first_checksum,
        second_checksum
    );
}
