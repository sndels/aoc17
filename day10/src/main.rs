// https://adventofcode.com/2017/day/10

fn main() {
    // First star
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let input = [120, 93, 0, 90, 5, 80, 129, 74, 1, 165, 204, 255, 254, 2, 50, 113].to_vec();

    let sparse_hash = knot_hash(&input, 1, 0, 0);
    let product = sparse_hash[0] as u16 * sparse_hash[1] as u16;
    // Assert to facilitate further tweaks
    assert_eq!(826, product);

    println!("The product of the first two elements is {}", product);

    // Second star
    // Convert input to a vector of ASCII codes
    let mut input: Vec<u8> = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113"
        .bytes()
        .collect();
    // Append given codes
    input.extend(&[17, 31, 73, 47, 23]);

    // Get sparse hash
    let sparse_hash = knot_hash(&input, 64, 0, 0);

    // XOR 16 byte chunks to form final hash string
    let mut dense_hash = String::new();
    for chunk in 0..16 {
        let mut output = 0;
        let first = chunk * 16;
        sparse_hash[first..(first + 16)]
            .iter()
            .for_each(|e| output ^= e);
        dense_hash += &format!("{:02x}", output);
    }
    // Assert to facilitate further tweaks
    assert_eq!("d067d3f14d07e09c2e7308c3926605c4", dense_hash);

    println!("The dense hash is {}", dense_hash);
}

fn knot_hash(input: &Vec<u8>, rounds: usize, skip_size: usize, slice_start: usize) -> Vec<u8> {
    let mut list: Vec<u8> = (0..255).collect();
    list.push(255); // Can't range collect u8s with 256

    // Shadow parameters to make mutable
    let mut skip_size = skip_size;
    let mut slice_start = slice_start;

    // Perform rounds of "ties"
    for _ in 0..rounds {
        for &length in input {
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
    list
}
