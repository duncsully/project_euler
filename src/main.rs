#![allow(dead_code)]

fn main() {
    println!("{}", solver2a());
}

/// Finds the sum of all multiples of 3 and 5 by finding the sums of each individually and then
/// subtracting the sum of multiples of 15 to remove "duplicates"
/// In theory this is faster than iterating on all 1000 integers: 333 + 200 + 66 = 599 < 1000
fn solver1() -> i32 {
    (3..1000).step_by(3).sum::<i32>() + (5..1000).step_by(5).sum::<i32>()
        - (15..1000).step_by(15).sum::<i32>()
}

/// Returns sum of arithmetic series from step to 1000 incrementing by step
fn sum_arithmetic_series(step: u32) -> u32 {
    let terms_count = 999 / step;
    let last_term = terms_count * step;
    terms_count * (step + last_term) / 2
}

/// More efficient solver using arithmetic series formula
fn solver1a() -> u32 {
    sum_arithmetic_series(3) + sum_arithmetic_series(5) - sum_arithmetic_series(15)
}

/// Finds the sum of all even Fibonacci sequence numbers below four million.
/// Uses naive approach of manually calculating and checking every number. There is likely
/// a math trick I am missing.
fn solver2() -> u32 {
    let mut term1 = 1;
    let mut term2 = 2;
    let mut sum = 0;

    while term2 < 4_000_000 {
        if term2 % 2 == 0 {
            sum += term2;
        }
        let next = term1 + term2;
        term1 = term2;
        term2 = next;
    }
    sum
}

/// Yep, I didn't come up with this on my own but wanted to implement it. This uses a formula
/// for getting only even numbers in the sequence.
fn solver2a() -> u32 {
    let mut term1 = 2;
    let mut term2 = 8;
    let mut sum = 2;

    while term2 < 4_000_000 {
        sum += term2;
        let next = term1 + term2 * 4;
        term1 = term2;
        term2 = next;
    }
    sum
}
