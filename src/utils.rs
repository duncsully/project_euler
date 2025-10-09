use std::ops::{AddAssign, MulAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    fn big_int_digit_count_works() {
        let big_int = BigInt::from(12345);
        assert_eq!(big_int.digit_count(), 5);
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
}
