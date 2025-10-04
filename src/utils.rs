use std::ops::MulAssign;
pub struct BigInt {
    digits: Vec<u8>,
}

impl BigInt {
    fn new() -> BigInt {
        BigInt { digits: vec![0] }
    }

    pub fn digit_sum(&self) -> u32 {
        self.digits.iter().map(|digit| *digit as u32).sum()
    }
}

impl From<u8> for BigInt {
    fn from(value: u8) -> Self {
        let mut digits = Vec::<u8>::new();
        let mut remainder = value;
        while remainder > 0 {
            digits.push(remainder % 10);
            remainder /= 10;
        }
        BigInt { digits }
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
