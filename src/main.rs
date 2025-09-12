fn main() {
    println!("{}", solver1a());
}

/// Finds the sum of all multiples of 3 and 5 by finding the sums of each individually and then
/// subtracting the sum of multiples of 15 to remove "duplicates"
/// In theory this is faster than iterating on all 1000 integers: 333 + 200 + 66 = 599 < 1000
#[allow(dead_code)]
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
