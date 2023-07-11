const GEN_A_START: usize = 516;
const GEN_B_START: usize = 190;

const GEN_A_MUL: usize = 16807;
const GEN_B_MUL: usize = 48271;

const DIVISOR: usize = 2147483647;

const BIT_MASK_16: usize = u16::MAX as usize;

fn main() {
    part2();
}

fn part1() {
    let mut gen_a = GEN_A_START;
    let mut gen_b = GEN_B_START;

    let mut bit_match = 0;
    for _ in 0..40000000 {
        gen_a = (gen_a * GEN_A_MUL) % DIVISOR;
        gen_b = (gen_b * GEN_B_MUL) % DIVISOR;

        if gen_a & BIT_MASK_16 == gen_b & BIT_MASK_16 {
            bit_match += 1;
        }
    }
    println!("Matches: {}", bit_match);
}

fn part2() {
    let mut gen_a = GEN_A_START;
    let mut gen_b = GEN_B_START;

    let mut a_values = Vec::new();
    let mut b_values = Vec::new();

    for _ in 0.. {
        gen_a = (gen_a * GEN_A_MUL) % DIVISOR;
        gen_b = (gen_b * GEN_B_MUL) % DIVISOR;

        if gen_a % 4 == 0 {
            a_values.push(gen_a);
        }

        if gen_b % 8 == 0 {
            b_values.push(gen_b);
        }

        if a_values.len() > 5000000 && b_values.len() > 5000000 {
            break;
        }
    }

    let mut bit_matches = 0;
    for i in 0..a_values.len().min(b_values.len()) {
        if a_values[i] & BIT_MASK_16 == b_values[i] & BIT_MASK_16 {
            bit_matches += 1;
        }
    }
    println!("Bit Matches: {}", bit_matches);
}
