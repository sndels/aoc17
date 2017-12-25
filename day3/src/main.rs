// https://adventofcode.com/2017/day/3

const INPUT: i32 = 312051;

fn main() {
    // First star
    // Go through spiral with full "strides", length increases by one in two corners
    // * < * * *
    // * * < * ^
    // * v > ^ *
    // v * * > *
    // * * * * >
    let mut stride_len = 1;
    let mut remaining = INPUT;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = Direction::Right;
    while remaining - stride_len > 0 {
        remaining -= stride_len;
        match dir {
            Direction::Up => {
                y += stride_len;
                dir = Direction::Left;
                stride_len += 1;
            }
            Direction::Left => {
                x -= stride_len;
                dir = Direction::Down;
            }
            Direction::Down => {
                y -= stride_len;
                dir = Direction::Right;
                stride_len += 1;
            }
            Direction::Right => {
                x += stride_len;
                dir = Direction::Up;
            }
        }
    }

    // Take remaining steps
    match dir {
        Direction::Up => y += remaining - 1,
        Direction::Left => x -= remaining - 1,
        Direction::Down => y -= remaining - 1,
        Direction::Right => x += remaining - 1,
    }

    let path_len = x.abs() + y.abs();
    // Assert to facilitate further tweaks
    assert_eq!(430, path_len);

    println!(
        "Square located at x:{} y:{}, shortest path to start is {}",
        x,
        y,
        path_len
    );

    // Second star
    let mut stride_len = 1;
    let mut steps = 0;
    let mut current = 1;
    let mut x: i32 = 1;
    let mut y: i32 = 0;
    let mut dir = Direction::Up;
    let mut sums: Vec<i32> = Vec::new();
    sums.push(1);

    // Take single steps from first square until one past given index
    while *sums.last().expect("Empty vector") <= INPUT {
        // Get neighbors
        let neighbors = match dir {
            Direction::Up => [
                get_tile_index(x, y - 1),
                get_tile_index(x - 1, y - 1),
                get_tile_index(x - 1, y),
                get_tile_index(x - 1, y + 1),
            ],
            Direction::Left => [
                get_tile_index(x + 1, y),
                get_tile_index(x - 1, y - 1),
                get_tile_index(x, y - 1),
                get_tile_index(x + 1, y - 1),
            ],
            Direction::Down => [
                get_tile_index(x, y + 1),
                get_tile_index(x + 1, y + 1),
                get_tile_index(x + 1, y),
                get_tile_index(x + 1, y - 1),
            ],
            Direction::Right => [
                get_tile_index(x - 1, y),
                get_tile_index(x - 1, y + 1),
                get_tile_index(x, y + 1),
                get_tile_index(x + 1, y + 1),
            ],
        };

        // Sum valid neighbors
        let mut sum: i32 = 0;
        for i in 0..4 {
            if neighbors[i] < current {
                sum += sums[neighbors[i] as usize];
            }
        }
        // Set value of current node
        sums.push(sum);

        // Step
        steps += 1;
        current += 1;
        match dir {
            Direction::Up => y += 1,
            Direction::Left => x -= 1,
            Direction::Down => y -= 1,
            Direction::Right => x += 1,
        }

        // Handle corners
        if steps >= stride_len {
            match dir {
                Direction::Up => {
                    dir = Direction::Left;
                    stride_len += 1;
                }
                Direction::Left => dir = Direction::Down,
                Direction::Down => {
                    dir = Direction::Right;
                    stride_len += 1;
                }
                Direction::Right => dir = Direction::Up,
            }
            steps = 0;
        }
    }

    let next = *sums.last().expect("Sums empty");
    // Assert to facilitate further tweaks
    assert_eq!(312453, next);
    println!("Next written value would be {}", next);
}

// Returns linear tile index based on coordinates
fn get_tile_index(x: i32, y: i32) -> i32 {
    if y * y >= x * x {
        if y < x {
            4 * y * y - y - x - 2 * (y - x)
        } else {
            4 * y * y - y - x
        }
    } else {
        if y < x {
            4 * x * x - y - x + 2 * (y - x)
        } else {
            4 * x * x - y - x
        }
    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}
