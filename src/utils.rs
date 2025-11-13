use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::mem::swap;
use std::ops::{AddAssign, MulAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct BigInt {
    digits: Vec<u8>,
}

impl BigInt {
    pub fn new() -> BigInt {
        BigInt { digits: vec![0] }
    }

    pub fn digit_sum(&self) -> u32 {
        self.digits.iter().map(|digit| *digit as u32).sum()
    }

    pub fn digit_count(&self) -> usize {
        self.digits.len()
    }
}

impl From<u32> for BigInt {
    fn from(value: u32) -> Self {
        let mut digits = Vec::<u8>::new();
        let mut remainder = value;
        while remainder > 0 {
            digits.push((remainder % 10) as u8);
            remainder /= 10;
        }
        BigInt { digits }
    }
}

impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        let mut carry = 0;
        for i in 0..self.digits.len().max(rhs.digits.len()) {
            let sum = self.digits.get(i).unwrap_or(&0) + rhs.digits.get(i).unwrap_or(&0) + carry;
            carry = sum / 10;
            if i < self.digits.len() {
                self.digits[i] = sum % 10;
            } else {
                self.digits.push(sum % 10);
            }
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }
}

impl MulAssign<u32> for BigInt {
    fn mul_assign(&mut self, rhs: u32) {
        let mut carry = 0;
        for digit in &mut self.digits {
            let product = *digit as u32 * rhs + carry;
            *digit = (product % 10) as u8;
            carry = product / 10;
        }
        while carry > 0 {
            self.digits.push((carry % 10) as u8);
            carry /= 10
        }
    }
}

impl MulAssign<BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: BigInt) {
        let mut result = BigInt::new();
        for (i, &rhs_digit) in rhs.digits.iter().enumerate() {
            let mut temp = self.clone();
            temp *= rhs_digit as u32;
            for _ in 0..i {
                temp.digits.insert(0, 0);
            }
            result += temp;
        }
        *self = result;
    }
}

impl ToString for BigInt {
    fn to_string(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|digit| digit.to_string())
            .collect()
    }
}

pub fn try_digits_to_u64(digits: &[u8]) -> Result<u64, &'static str> {
    if digits.iter().all(|&n| n < 10) {
        Ok(digits_to_u64(digits))
    } else {
        Err("Values must be < 10")
    }
}

/// Turns a slice representing a series of digits into a u64.
///
/// Note: Will treat numbers > 9 as having a "carry over" into
/// the next digit. If you want to check that all digits are
/// < 10 use try_digits_to_u64
pub fn digits_to_u64(digits: &[u8]) -> u64 {
    digits.iter().fold(0, |acc, &d| acc * 10 + d as u64)
}

pub trait Divisors {
    fn divisors(&self) -> Vec<Self>
    where
        Self: Sized;
    fn aliquot_sum(&self) -> Self
    where
        Self: Sized + std::ops::Add;

    fn is_abundant(&self) -> bool;
}

impl Divisors for u32 {
    fn divisors(&self) -> Vec<u32> {
        let mut divisors = vec![];
        for n in 1..=self / 2 {
            if self % n == 0 {
                divisors.push(n);
            }
        }
        divisors
    }

    fn aliquot_sum(&self) -> Self
    where
        Self: Sized + std::ops::Add,
    {
        self.divisors().iter().sum()
    }

    fn is_abundant(&self) -> bool {
        self.divisors().iter().sum::<u32>() > *self
    }
}

pub trait NextLexicographicPermutation {
    fn next_lexicographic_permutation(&mut self) -> &Self;
}

impl NextLexicographicPermutation for Vec<u32> {
    fn next_lexicographic_permutation(&mut self) -> &Self {
        // Because we're using rev, need to manually return the desired index, not the iter position
        if let Some(pivot) = self.iter().enumerate().rev().find_map(|(i, item)| {
            if i != (self.len() - 1) && *item < self[i + 1] {
                Some(i)
            } else {
                None
            }
        }) {
            let swap_index = self
                .iter()
                .enumerate()
                .rev()
                // Likewise
                .find_map(|(i, item)| if *item > self[pivot] { Some(i) } else { None })
                .expect("Swap index not found");
            self.swap(pivot, swap_index);
            self[pivot + 1..].reverse();
        } else {
            self.reverse();
        };
        self
    }
}

// AI generated code start
pub struct Permutations<'a, T> {
    slice: &'a [T],
    n: usize,
    indices: Option<Vec<usize>>,
    used: Option<Vec<bool>>,
    stack: VecDeque<Vec<usize>>,
}

impl<'a, T> Permutations<'a, T> {
    pub fn new(slice: &'a [T], n: usize) -> Self {
        let mut stack = VecDeque::new();
        if n > 0 && n <= slice.len() {
            stack.push_back(Vec::new());
        }
        Permutations {
            slice,
            n,
            indices: None,
            used: None,
            stack,
        }
    }
}

impl<'a, T: Clone> Iterator for Permutations<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(prefix) = self.stack.pop_front() {
            if prefix.len() == self.n {
                return Some(prefix.iter().map(|&i| self.slice[i].clone()).collect());
            }
            for i in 0..self.slice.len() {
                if !prefix.contains(&i) {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(i);
                    self.stack.push_back(new_prefix);
                }
            }
        }
        None
    }
}

pub fn permutations<T>(slice: &[T], n: usize) -> Permutations<'_, T> {
    Permutations::new(slice, n)
}
// AI generated code end

#[derive(Debug)]
pub struct FibonacciSequence {
    previous: BigInt,
    current: BigInt,
}

impl FibonacciSequence {
    pub fn new() -> FibonacciSequence {
        FibonacciSequence {
            previous: BigInt::from(0),
            current: BigInt::from(1),
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_copy = self.current.clone();
        self.current += self.previous.to_owned();
        self.previous = curr_copy;
        Some(self.current.clone())
    }
}

pub struct Primes {
    cache: Vec<u64>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            cache: Vec::<u64>::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_prime = match self.cache.len() {
            0 => 2,
            1 => 3,
            _ => {
                let start = self.cache.last().expect("No cached primes") + 2;
                (start..)
                    .step_by(2)
                    .find(|n| {
                        let max_check = n.isqrt();
                        let upper_index = self
                            .cache
                            .iter()
                            .position(|prime| *prime > max_check)
                            .unwrap_or(self.cache.len() - 1);
                        self.cache[0..upper_index]
                            .iter()
                            .all(|prime| n % prime != 0)
                    })
                    .expect("Couldn't find next prime")
            }
        };

        self.cache.push(next_prime);
        Some(next_prime)
    }
}

pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

impl IsPrime for u32 {
    fn is_prime(&self) -> bool {
        // Very rudimentary check
        if *self < 3 {
            return false;
        }
        if self % 2 == 0 {
            return false;
        }
        for odd in (3..=self.isqrt()).step_by(2) {
            if self % odd == 0 {
                return false;
            }
        }
        true
    }
}

// TODO: Make this more efficient by caching a list of primes?
impl IsPrime for u64 {
    fn is_prime(&self) -> bool {
        // Very rudimentary check
        if *self < 3 {
            return false;
        }
        if self % 2 == 0 {
            return false;
        }
        for odd in (3..=self.isqrt()).step_by(2) {
            if self % odd == 0 {
                return false;
            }
        }
        true
    }
}

// From Wikipedia (is that better than AI? Either way, yes, the laziness is setting in)
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}

pub fn digits<T: ToString>(n: T) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|char| char.to_digit(10).expect("Not a digit"))
        .collect()
}

pub fn is_pandigital(n: &u64) -> bool {
    let digits_vec = digits(n).into_iter().map(|digit| digit as u64);
    let n_len = digits_vec.len();
    let digits_set: HashSet<u64> = HashSet::from_iter(digits_vec);
    if n_len != digits_set.len() {
        return false;
    }
    let range_set = HashSet::from_iter(1..=n_len as u64);

    digits_set == range_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_lexicographic_permutation_works() {
        assert_eq!(
            *vec![1u32, 2, 3, 4].next_lexicographic_permutation(),
            vec![1, 2, 4, 3]
        )
    }

    #[test]
    fn big_ints_add_assign_works() {
        let mut big_int = BigInt::from(29);
        big_int += BigInt::from(13);
        assert_eq!(big_int.digits, vec![2, 4]);
        big_int += BigInt::from(100);
        assert_eq!(big_int.digits, vec![2, 4, 1]);
        big_int += BigInt::from(900);
        assert_eq!(big_int.digits, vec![2, 4, 0, 1]);
    }

    #[test]
    fn big_ints_mul_assign_big_int_works() {
        let mut big_int = BigInt::from(29);
        big_int *= BigInt::from(13);
        assert_eq!(big_int.digits, vec![7, 7, 3]);
        big_int *= BigInt::from(100);
        assert_eq!(big_int.digits, vec![0, 0, 7, 7, 3]);
        big_int *= BigInt::from(9);
        assert_eq!(big_int.digits, vec![0, 0, 3, 9, 3, 3]);
    }

    #[test]
    fn big_int_digit_count_works() {
        let big_int = BigInt::from(12345);
        assert_eq!(big_int.digit_count(), 5);
    }

    #[test]
    fn big_int_to_string_works() {
        let big_int = BigInt::from(12345);
        assert_eq!(big_int.to_string(), "12345");
    }

    #[test]
    fn fibonacci_sequence_iterates() {
        let mut fib = FibonacciSequence::new();
        assert_eq!(fib.next().unwrap(), BigInt::from(1));
        assert_eq!(fib.next().unwrap(), BigInt::from(2));
        assert_eq!(fib.next().unwrap(), BigInt::from(3));
        assert_eq!(fib.next().unwrap(), BigInt::from(5));
        assert_eq!(fib.next().unwrap(), BigInt::from(8));
        assert_eq!(fib.next().unwrap(), BigInt::from(13));
    }

    #[test]
    fn primes_iterates() {
        let primes = Primes::new();
        assert_eq!(
            primes.take(6).collect::<Vec<u64>>(),
            vec![2, 3, 5, 7, 11, 13]
        );
    }

    #[test]
    fn permutations_iterator_works() {
        let v = vec![1, 2, 3, 4];
        let perms: Vec<Vec<_>> = permutations(&v, 2).collect();
        let mut expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 1],
            vec![2, 3],
            vec![2, 4],
            vec![3, 1],
            vec![3, 2],
            vec![3, 4],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
        ];
        expected.sort();
        let mut perms_sorted = perms.clone();
        perms_sorted.sort();
        assert_eq!(perms_sorted, expected);
    }

    #[test]
    fn permutations_empty_and_n_too_large() {
        let v: Vec<u32> = vec![];
        let perms: Vec<Vec<_>> = permutations(&v, 2).collect();
        assert_eq!(perms, Vec::<Vec<u32>>::new());
        let v = vec![1, 2];
        let perms: Vec<Vec<_>> = permutations(&v, 3).collect();
        assert_eq!(perms, Vec::<Vec<u32>>::new());
    }

    #[test]
    fn permutations_n_equals_1() {
        let v = vec![1, 2, 3];
        let perms: Vec<Vec<_>> = permutations(&v, 1).collect();
        let mut expected = vec![vec![1], vec![2], vec![3]];
        expected.sort();
        let mut perms_sorted = perms.clone();
        perms_sorted.sort();
        assert_eq!(perms_sorted, expected);
    }

    #[test]
    fn digits_to_u64_works() {
        assert_eq!(digits_to_u64(&[1, 2, 3]), 123);
        assert_eq!(digits_to_u64(&[1, 12, 3]), 223);
    }

    #[test]
    fn is_pandigital_works() {
        assert!(is_pandigital(&1423));
        assert!(is_pandigital(&624531));
        assert!(!is_pandigital(&134));
    }

    #[test]
    fn is_prime_works() {
        assert!(!1u64.is_prime());
        assert!(!2u64.is_prime());
        assert!(3u64.is_prime());
        assert!(!4u64.is_prime());
        assert!(5u64.is_prime());
        assert!(17u64.is_prime());
        assert!(!63u64.is_prime());
    }
}
