#[derive(Clone, PartialEq, Eq)]
pub struct BigInt {
    digits: Vec<usize>,
}

pub struct DivisionResult {
    amount: BigInt,
    remainder: BigInt,
}

impl BigInt {
    pub fn zero() -> Self {
        Self { digits: Vec::new() }
    }

    pub fn subtract(mut self, mut other: Self) -> Option<Self> {
        todo!()
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut self_digits = self.digits.iter();
        let mut other_digits = other.digits.iter();
        let mut result_digits = Vec::new();
        let mut is_overflowed = false;
        loop {
            let self_digit = match self_digits.next() {
                Some(n) => *n,
                None => 0,
            };
            let other_digit = match other_digits.next() {
                Some(n) => *n,
                None => 0,
            };
            if self_digit == 0 && other_digit == 0 && !is_overflowed {
                break;
            }
            let (mut result_digit, new_is_overflowed) = self_digit.overflowing_add(other_digit);
            if is_overflowed {
                result_digit = result_digit.checked_add(1).unwrap();
            }
            result_digits.push(result_digit);
            is_overflowed = new_is_overflowed;
        }
        Self { digits: result_digits }
    }

    pub fn multiply(&mut self, mut factor: Self) {
        todo!()
    }

    pub fn divide(mut self, other: Self) -> DivisionResult {
        todo!()
    }
}

impl From<usize> for BigInt {
    fn from(value: usize) -> Self {
        Self {
            digits: vec![value],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(BigInt::from(5).add(&BigInt::from(15)).digits, vec![20]);
        assert_eq!(
            BigInt::from(5).add(&BigInt::from(usize::MAX)).digits,
            vec![4, 1]
        );
        assert_eq!(
            BigInt::from(usize::MAX).add(&BigInt::from(usize::MAX)).digits,
            vec![usize::MAX - 1, 1]
        );
        assert_eq!(
            BigInt::from(usize::MAX).add(&BigInt::from(usize::MAX)).add(&BigInt::from(usize::MAX)).digits,
            vec![usize::MAX - 2, 2]
        );
    }
}
