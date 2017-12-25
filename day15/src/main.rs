// https://adventofcode.com/2017/day/15

fn main() {
    // First star
    let mut gen_a: u64 = 512;
    let mut gen_b: u64 = 191;
    let mut matches = 0;
    for _ in 0..40000000 {
        // Run generators
        gen_a = gen_a.wrapping_mul(16807) % 2147483647;
        gen_b = gen_b.wrapping_mul(48271) % 2147483647;

        // Compare lowest 16 bits
        if gen_a & 0xFFFF == gen_b & 0xFFFF {
            matches += 1;
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(567, matches);

    println!("{} out of 40 million pairs match", matches);

    // Second star
    gen_a = 512;
    gen_b = 191;
    matches = 0;
    for _ in 0..5000000 {
        // Get a value that is a multiple of 4
        gen_a = gen_a.wrapping_mul(16807) % 2147483647;
        while gen_a % 4 != 0 {
            gen_a = gen_a.wrapping_mul(16807) % 2147483647;
        }

        // Get a value that is a multiple of 8
        gen_b = gen_b.wrapping_mul(48271) % 2147483647;
        while gen_b % 8 != 0 {
            gen_b = gen_b.wrapping_mul(48271) % 2147483647;
        }

        // Compare lowest 16 bits
        if gen_a & 0xFFFF == gen_b & 0xFFFF {
            matches += 1;
        }
    }
    // Assert to facilitate further tweaks
    assert_eq!(323, matches);

    println!("{} out of 5 million pairs match", matches);
}
