// https://adventofcode.com/2017/day/6

use std::collections::HashMap;

fn pop_largest(banks: &mut [u8]) -> (usize, u8) {
    let mut largest = 0;
    let mut i_largest = 0;
    for (i, blocks) in banks.iter().enumerate() {
        if *blocks > largest {
            largest = *blocks;
            i_largest = i;
        }
    }
    banks[i_largest] = 0;
    (i_largest, largest)
}

fn distribute_blocks(start_index: usize, blocks: u8, banks: &mut [u8]) {
    let mut i = start_index;
    let mut blocks = blocks; // Shadow parameter to make mutable
    while blocks > 0 {
        banks[i] += 1;
        blocks -= 1;
        i += 1;
        if i >= banks.len() {
            i = 0;
        }
    }
}

fn main() {
    let mut banks = [5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6];
    let mut states = HashMap::new();

    // Loop until a previous state is encountered
    let mut cycles = 0;
    let loop_cycles;
    loop {
        // Get state as string and compare to previously seen
        let state = format!("{:?}", banks);
        if let Some(prev_cycles) = states.get(&state) {
            loop_cycles = cycles - prev_cycles;
            break;
        }
        states.insert(state, cycles);

        // Pop largest bank and distribute blocks
        let (index, blocks) = pop_largest(&mut banks);
        if index < banks.len() - 1 {
            distribute_blocks(index + 1, blocks, &mut banks);
        } else {
            distribute_blocks(0, blocks, &mut banks);
        }
        cycles += 1;
    }
    // Assert to facilitate further tweaks
    assert_eq!(5042, cycles);
    assert_eq!(1086, loop_cycles);

    println!("{} cycles needed, {} cycles in loop", cycles, loop_cycles);
}
