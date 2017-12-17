// https://adventofcode.com/2017/day/17

fn main() {
    let step = 371;

    // First star
    let mut buffer = vec![0];

    // Push 2017 values to the buffer
    let mut pos = 0;
    for i in 1..2018 {
        // Step ahead by step + 1 to get in the right place for insert
        pos = (pos + step + 1) % buffer.len();

        // Insert value at current position
        // Value after "back" will be inserted to "front"
        buffer.insert(pos, i);
    }

    println!(
        "The value after 2017 is {}",
        buffer[(pos + 1) % buffer.len()]
    );

    // Second star
    pos = 0;
    let mut after_zero = 0;
    // We only need to track the element at "index" 0:
    // The value 0 gets pushed to the "end" of the buffer as nothing will be inserted
    // right "after" it
    // [0]
    // [1, 0]
    // [2, 1, 0]
    for i in 1..50000001 {
        // Step ahead by step + 1 to get in the right place for insert
        pos = (pos + step + 1) % i;

        // If position is 0, update tracked value
        if pos == 0 {
            after_zero = i;
        }
    }

    println!("The value after 0 is {}", after_zero);
}
