#![allow(dead_code)]

fn main() {
    println!("{}", solver6a(100));
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

/// They gave us an easy one this time as far as finding the mathgic. Just need to use
/// prime factorization and use the highest exponent for each prime that occurs in any
/// of the integers from 1-20. This could technically be generalized using my PrimeNumbers
/// struct to get prime factors for any arbitrary range, calculating and storing the max
/// exponent, but...maybe another time.
fn solver5() -> u32 {
    2u32.pow(4) * 3u32.pow(2) * 5 * 7 * 11 * 13 * 17 * 19
}

/// Huh...OK that one was really straight forward. Though at least it helped me appreciate
/// Rust's iterators even more with how easy it was to implement
fn solver6() -> u64 {
    let square_of_sums = (1..=100).sum::<u64>().pow(2);
    let sum_of_squares = (1..=100).map(|num: u64| num.pow(2)).sum::<u64>();
    square_of_sums - sum_of_squares
}

// OK, of course there were optimizations to make this run in O(1) even as the limit grows large
fn solver6a(limit: u64) -> u64 {
    let sum = limit * (limit + 1) / 2;
    let sum_sq = (2 * limit + 1) * (limit + 1) * limit / 6;
    sum.pow(2) - sum_sq
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
