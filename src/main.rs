#![allow(dead_code)]

fn main() {
    println!("{}", solver9());
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
