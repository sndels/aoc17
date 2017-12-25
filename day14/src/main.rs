// https://adventofcode.com/2017/day/14

fn main() {
    let input = "ljoxqyyw";

    // Generate disk grid and count used space
    let mut grid = Vec::new();
    let mut used_space = 0;
    for i in 0..128 {
        // Convert row input to utf-8 bytes and generate knot hash
        let mut row_input: Vec<u8> = (input.to_string() + "-" + i.to_string().as_str())
            .bytes()
            .collect();

        let hash = knot_hash(&row_input);

        grid.push(hash_to_bits(&hash));

        used_space += hash.iter()
            .map(|&b| {
                (SET_BITS[(b >> 4) as usize] + SET_BITS[(b & 0xF) as usize]) as u32
            })
            .sum::<u32>();
    }
    // Assert to facilitate further tweaks
    assert_eq!(8316, used_space);

    println!("{} spaces used", used_space);

    // Do dfs through the grid to count distinct regions
    let mut visited = [[false; 128]; 128];
    let mut regions = 0;
    for (j, row) in grid.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell {
                if !visited[j][i] {
                    // New region
                    regions += 1;
                    let mut stack = vec![(i, j)];
                    while let Some((x, y)) = stack.pop() {
                        visited[y][x] = true;
                        if grid[y][x] {
                            for &(nx, ny) in get_neighbours(x, y).iter() {
                                if !visited[ny][nx] {
                                    stack.push((nx, ny));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(1074, regions);

    println!("{} regions present", regions);
}

fn knot_hash(input: &Vec<u8>) -> Vec<u8> {
    let mut list: Vec<u8> = (0..255).collect();
    list.push(255); // Can't range collect u8s with 256

    // Perform knots
    let mut slice_start = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for &length in input.iter().chain(KNOT_PAD) {
            let slice_end = slice_start + length as usize;

            // Check that slice doesn't wrap around
            if slice_end <= 256 {
                &list[slice_start..slice_end].reverse();
            } else {
                // Reverse slice that wraps around
                for i in 0..(length / 2) as usize {
                    let e1 = (slice_start + i) % 256;
                    let e2 = (slice_start + length as usize - 1 - i) % 256;
                    let tmp = list[e2];
                    list[e2] = list[e1];
                    list[e1] = tmp;
                }
            }
            slice_start = (slice_start + length as usize + skip_size) % 256;
            skip_size += 1;
        }
    }

    // XOR 16 byte chunks to form final hash
    list.chunks(16)
        .map(|c| c.iter().fold(0, |acc, &x| acc ^ x))
        .collect()
}

fn hash_to_bits(hash: &Vec<u8>) -> Vec<bool> {
    let mut row = Vec::new();
    hash.iter().for_each(|&b| {
        for i in (0..8).rev() {
            row.push(b & (0x1 << i) > 0);
        }
    });
    row
}

fn get_neighbours(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x, y.saturating_sub(1)),
        (if x + 1 < 128 { x + 1 } else { 127 }, y),
        (x, if y + 1 < 128 { y + 1 } else { 127 }),
        (x.saturating_sub(1), y),
    ]
}

static KNOT_PAD: &'static [u8] = &[17, 31, 73, 47, 23];
static SET_BITS: &'static [u8] = &[0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];
