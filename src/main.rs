#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs,
    thread::{self, JoinHandle},
};

fn main() {
    println!("{}", solver13());
}

/// Alright, this was probably a little unorthodox. I finally decided to try out parsing
/// a file for the input! As for solving, first off, it's pretty clear we would overflow
/// with brute math so I got the idea to do elementary addition digit by digit. I collect
/// the sum of each placement into a vector.
///
/// Now, one idea I had originally was to go through the effort to "carry over" any
/// overflow into the next indices working from so that I'd have exactly 1 digit in each
/// index in the vector, and then I could just grab the range of the "last" 10 items, but
/// this seemed like needless work when I knew I needed only 10 of the 50+ digits.
///
/// Instead I realized that since each place is x10 from the next, I can just keep "shifting"
/// my number and adding the next sum of the digits in a place. However, since there was
/// still the possibility of carry-over, I needed to overshoot and cross my fingers that
/// I had enough that nothing else was likely to affect the first 10 digits. 20 sums resulted in
/// overflows so I settled for 15 sums, though after confirming the answer it so happens
/// you only need the 11th sum, but of course this can't be guaranteed AFAIK. As usual
/// there's probably some mathgic I'm missing, though.
fn solver13() -> String {
    let contents = fs::read_to_string("inputs/problem13.txt").expect("Unable to load input file");
    // Create flat array with everything "rotated" so that the 1s digits are together, then 10s, etc.
    // i.e. 50 virtual "rows" (placements) of 100 "columns" (digits in placement)
    let mut digits = [0u8; 5000];
    contents.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .rev()
            .map(|char| char.to_digit(10).expect("Not a digit") as u8)
            .enumerate()
            .for_each(|(column, digit)| digits[100 * column + row] = digit);
    });
    // Worst case scenario = 9 * 100 = 900
    let digit_sums: Vec<u16> = (0..5000)
        .step_by(100)
        .map(|i| digits[i..i + 100].iter().map(|digit| *digit as u16).sum())
        .rev()
        .collect();

    let mut first_digits = 0;
    // Honestly, this is probably not reliable but chances are an extra 5 digits in we're not going to have
    // any more carry-overs...
    for i in 0..15 {
        first_digits += digit_sums[i] as u64;
        // Shift all digits over to prepare adding the next
        first_digits *= 10;
    }
    first_digits.to_string()[0..10].to_string()
}

/// I'm never going to escape my PrimeNumbers, am I? Was easy enough to look up how to calculate
/// the nth triangular number, and that you can get the number of divisors via prime factorization,
/// which did make this relatively straight forward to solve and surprisingly quickly. Not sure if
/// there is any optimization to this one other than maybe starting with a more sensible n depending
/// on the input.
fn solver12(divisors_min: u32) -> u64 {
    let mut primes = PrimeNumbers::new();
    let mut n = 1; // If this was less general purpose, we could start with something higher
    loop {
        let mut factors: Vec<u64> = Vec::new();
        let mut factor_counts: HashMap<u64, u32> = HashMap::new();
        let triangular_number = nth_triangular_number(n);
        primes.get_prime_factors(triangular_number, &mut factors);

        for factor in factors {
            *factor_counts.entry(factor).or_default() += 1;
        }

        let divisors_count: u32 = factor_counts.values().map(|val| val + 1).product();
        if divisors_count > divisors_min {
            println!("The n={n} triangular number is the first with over 500 divisors:");
            return triangular_number;
        }
        n += 1;
    }
}

fn nth_triangular_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

// I attempted to use a const function to initialize the array but gave up so honestly
// I resorted to just parsing the input in JS so I could move on with the problem
const GRID: [[u32; 20]; 20] = [
    [
        8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8,
    ],
    [
        49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0,
    ],
    [
        81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65,
    ],
    [
        52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68, 56, 1, 32, 56, 71, 37, 2, 36, 91,
    ],
    [
        22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    ],
    [
        24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    ],
    [
        32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    ],
    [
        67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21,
    ],
    [
        24, 55, 58, 5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    ],
    [
        21, 36, 23, 9, 75, 0, 76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95,
    ],
    [
        78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92,
    ],
    [
        16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0, 17, 54, 24, 36, 29, 85, 57,
    ],
    [
        86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    ],
    [
        19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55, 40,
    ],
    [
        4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    ],
    [
        88, 36, 68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    ],
    [
        4, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36,
    ],
    [
        20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16,
    ],
    [
        20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 5, 54,
    ],
    [
        1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 1, 89, 19, 67, 48,
    ],
];

/// This reminds me of Advent of Code type problems. Starting from a cell, you can check in
/// 4 directions: NE, E SE, and S. And of course depending on the direction, you're limited
/// in how many rows and columns you need to check. Other than that, it's simply a matter of
/// iterating through every combo. If it's 0 we can just ignore that cell.
fn solver11() -> u32 {
    (0..GRID.len())
        .map(check_row)
        .max()
        .expect("Couldn't find max")
}

/// This seemed like a good opportunity to attempt multi-threading since each row can be checked
/// independently from a const grid.
fn solver11a() -> u32 {
    let handles: Vec<JoinHandle<u32>> = (0..GRID.len())
        .map(|row| thread::spawn(move || check_row(row)))
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .max()
        .expect("Couldn't find max")
}

fn check_row(row: usize) -> u32 {
    let mut max = 0u32;

    for (column, val) in GRID[row].iter().enumerate() {
        if *val == 0u32 {
            continue;
        }
        // NE
        if column < 17 && row > 3 {
            let product = (0..4).map(|diff| GRID[row - diff][column + diff]).product();
            max = max.max(product)
        }
        // E
        if column < 17 {
            let product = (0..4).map(|diff| GRID[row][column + diff]).product();
            max = max.max(product)
        }
        // SE
        if column < 17 && row < 17 {
            let product = (0..4).map(|diff| GRID[row + diff][column + diff]).product();
            max = max.max(product)
        }
        // S
        if row < 17 {
            let product = (0..4).map(|diff| GRID[row + diff][column]).product();
            max = max.max(product)
        }
    }
    println!("Found max for row {row} of {max}");
    max
}

/// Another case of Rust somewhat trivializing the problem, though this is likely not
/// terribly efficient. Not to mention PrimeNumbers does not feel very well designed or
/// idiomatic and so I'm only more prone to abusing it as I go.
fn solver10(n: u64) -> u64 {
    let mut primes = PrimeNumbers::new();
    primes.add_primes(n);
    primes.cache.iter().filter(|prime| **prime < n).sum()
}

/// Another relatively brute force method with some optimizations based on the limits
/// of what contendor numbers can be.
fn solver9() -> i32 {
    // Since x < y < z, we start at 1 and x can only ever be at most = 332 < 333 < 335
    for x in 1..332i32 {
        // Honestly, it's difficult to explain why the max is 500 - x / 2, but this
        // is what I inferred after some manual enumeration
        for y in x + 1..500 - x / 2 {
            let z = 1000 - x - y;
            let sum = f64::from(x.pow(2) + y.pow(2));
            if (sum.sqrt()) == z as f64 {
                println!("Found: {x} {y} {z}");
                return x * y * z;
            }
        }
    }
    panic!("Answer not found!")
}

/// Originally I wanted to be more clever by keeping track of a single mutable value where
/// I'd divide the starting digit and then multiply the next digit in a sort of queue but
/// 0s quickly shot down this approach without needing more complications to check for 0s
/// within a range and start fresh with a new product whenever we found a range without 0s.
/// So I went with a more naive brute force sliding range. Once again, Rust iterators to the
/// rescue. Apparently there isn't any PDF for this one describing any clever tricks so...I
/// guess I'm on my own if I want to optimize this.
fn solver8() -> u64 {
    let str_num = String::from(
        "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450",
    );
    let mut start = 0;
    let increment = 13;
    let mut max = 0u64;
    while start + increment < str_num.len() {
        let product = str_num[start..start + increment]
            .chars()
            .map(|char| char.to_digit(10).expect("Not a digit"))
            .map(u64::from)
            .product();
        if product > max {
            max = product
        }
        start += 1;
    }
    max
}

/// Alright, so I did end up reusing the PrimeNumbers object, and ultimately this was brute
/// force, but I guess still not too much for Rust to handle. I'm hoping there's some mathgic
/// to make this more efficient.
fn solver7() -> u64 {
    let mut primes = PrimeNumbers::new();
    primes.get_nth_prime(10_001)
}

/// Huh...OK that one was really straight forward. Though at least it helped me appreciate
/// Rust's iterators even more with how easy it was to implement
fn solver6() -> u64 {
    let square_of_sums = (1..=100).sum::<u64>().pow(2);
    let sum_of_squares = (1..=100).map(|num: u64| num.pow(2)).sum::<u64>();
    square_of_sums - sum_of_squares
}

/// OK, of course there were optimizations to make this run in O(1) even as the limit grows large
fn solver6a(limit: u64) -> u64 {
    let sum = limit * (limit + 1) / 2;
    let sum_sq = (2 * limit + 1) * (limit + 1) * limit / 6;
    sum.pow(2) - sum_sq
}

/// They gave us an easy one this time as far as finding the mathgic. Just need to use
/// prime factorization and use the highest exponent for each prime that occurs in any
/// of the integers from 1-20. This could technically be generalized using my PrimeNumbers
/// struct to get prime factors for any arbitrary range, calculating and storing the max
/// exponent, but...maybe another time.
fn solver5() -> u32 {
    2u32.pow(4) * 3u32.pow(2) * 5 * 7 * 11 * 13 * 17 * 19
}

fn is_palindrome(n: &impl ToString) -> bool {
    let n_str = n.to_string();
    let reverse: String = n_str.chars().rev().collect();
    n_str == reverse
}

/// I'm pleased with how this one turned out (ignoring the palindrome check...)
/// because it *seems* like more idiomatic Rust this time around. It could be made
/// a bit more efficient and once again I'm probably missing some mathgic but this
/// one felt less bad than the last problem.
fn solver4() -> u32 {
    // Two three-digit numbers would result in a six-digit product
    (100_000..999_999)
        .filter(is_palindrome)
        .rev()
        .find_map(|product: u32| {
            // The factors switch after the sqrt, so we only need to check up to it
            let max = product.isqrt();
            if let Some(factor) = (100..max).rev().find(|check_factor| {
                product % check_factor == 0 && (100..999).contains(&(product / check_factor))
            }) {
                println!("Factors: {factor} * {}", product / factor);
                Some(product)
            } else {
                None
            }
        })
        .unwrap()
}

struct PrimeNumbers {
    cache: Vec<u64>,
}

impl PrimeNumbers {
    // A better implementation might let you specify your own starting vector
    // but then I would need to at least validate it which in part eliminates
    // any benefit unless I blindly trust the input
    pub fn new() -> PrimeNumbers {
        // Seed the cache with known primes
        PrimeNumbers {
            cache: vec![2, 3, 5, 7, 11, 13],
        }
    }

    fn add_primes(&mut self, to: u64) {
        let mut next = self.cache.last().unwrap_or(&0) + 1;
        while next < to {
            self.cache_if_prime(next);
            next += 1;
        }
    }

    fn get_nth_prime(&mut self, n: usize) -> u64 {
        let mut check = self.cache.last().unwrap_or(&1).clone();
        while (self.cache.len()) < n {
            check += 1;
            self.cache_if_prime(check);
        }
        self.cache[n - 1]
    }

    fn cache_if_prime(&mut self, n: u64) {
        let max_check = n.isqrt();
        let upper_index = self
            .cache
            .iter()
            .position(|prime| *prime > max_check)
            .unwrap_or(self.cache.len() - 1);
        if self.cache[0..upper_index]
            .iter()
            .all(|prime| n % prime != 0)
        {
            self.cache.push(n);
        }
    }

    pub fn get_prime_factors(&mut self, n: u64, out: &mut Vec<u64>) {
        if self.cache.binary_search(&n).is_ok() {
            out.push(n);
            return;
        }
        let max = n.isqrt();
        self.add_primes(max);
        if let Some(factor) = self
            .cache
            .iter()
            .filter(|prime| **prime != 1 && **prime <= max)
            .find_map(|prime| if n % *prime == 0 { Some(prime) } else { None })
        {
            out.push(*factor);
            self.get_prime_factors(n / factor, out)
        } else {
            out.push(n)
        }
    }
}

/// This one was a bit messier than I'd like ironically because I was probably trying to be
/// too clever and there is a lot of leftover unnecessary work from previous iterations. I've
/// had my fill with this one so I'm just going to leave this inelegant solution in for now.
fn solver3() -> u64 {
    let mut primes = PrimeNumbers::new();

    let mut prime_factors = Vec::new();
    primes.get_prime_factors(600851475143, &mut prime_factors);

    prime_factors.sort();
    *prime_factors.last().expect("No prime factors found")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_works() {
        assert!(is_palindrome(&1));
        assert!(is_palindrome(&1001));
        assert!(is_palindrome(&51615));
        assert!(is_palindrome(&987789));
        assert!(!is_palindrome(&123));
        assert!(!is_palindrome(&1212));

        assert!(is_palindrome(&String::from("racecar")));
        assert!(!is_palindrome(&String::from("extracredit")));
    }
}
