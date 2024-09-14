#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigUInt {
    digits: Vec<usize>,
}

pub struct DivisionResult {
    pub quotient: BigUInt,
    pub remainder: BigUInt,
}

impl BigUInt {
    pub fn zero() -> Self {
        Self { digits: Vec::new() }
    }

    pub fn subtract(&self, other: &Self) -> Option<Self> {
        let mut self_digits = self.digits.iter();
        let mut other_digits = other.digits.iter();
        let mut result_digits = Vec::new();
        let mut is_overflowed = false;
        loop {
            let self_digit = self_digits.next()?;
            let other_digit = other_digits.next();
            let (mut result_digit, mut new_is_overflowed) =
                self_digit.overflowing_sub(*other_digit.unwrap_or(&0));
            if is_overflowed {
                let (new_result_digit, new_new_is_overflowed) = result_digit.overflowing_sub(1);
                new_is_overflowed = new_is_overflowed || new_new_is_overflowed;
                result_digit = new_result_digit;
            }
            is_overflowed = new_is_overflowed;
            result_digits.push(result_digit);
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut self_digits = self.digits.iter();
        let mut other_digits = other.digits.iter();
        let mut result_digits = Vec::new();
        let mut is_overflowed = false;
        loop {
            let self_digit = self_digits.next();
            let other_digit = other_digits.next();
            if self_digit.is_none() && other_digit.is_none() && !is_overflowed {
                break;
            }
            let (mut result_digit, new_is_overflowed) = self_digit
                .unwrap_or(&0)
                .overflowing_add(*other_digit.unwrap_or(&0));
            if is_overflowed {
                result_digit = result_digit.checked_add(1).unwrap();
            }
            result_digits.push(result_digit);
            is_overflowed = new_is_overflowed;
        }
        Self {
            digits: result_digits,
        }
    }

    pub fn multiply(&self, factor: &Self) -> Self {
        let result = BigUInt::zero();
        let mut factor = factor.clone();
        loop {
            factor = match factor.subtract(&BigUInt::from(1)) {
                None => break,
                Some(factor) => {
                    result.add(self);
                    factor
                }
            }
        }
        result
    }

    pub fn divide(&self, other: &Self) -> DivisionResult {
        let mut remainder = self.clone();
        let quotient = BigUInt::zero();
        loop {
            remainder = match remainder.subtract(other) {
                None => break,
                Some(result) => {
                    quotient.add(&BigUInt::from(1));
                    result
                }
            };
        }
        DivisionResult {
            quotient,
            remainder,
        }
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match Ord::cmp(&self.digits.len(), &other.digits.len()) {
            Ordering::Equal => (),
            ordering => return ordering,
        };
        for index in (0..self.digits.len()).rev() {
            match unsafe {
                Ord::cmp(
                    self.digits.get_unchecked(index),
                    other.digits.get_unchecked(index),
                )
            } {
                Ordering::Equal => (),
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl From<usize> for BigUInt {
    fn from(value: usize) -> Self {
        let mut digits = Vec::new();
        if value != 0 {
            digits.push(value);
        }
        Self { digits }
    }
}

impl TryFrom<&BigUInt> for u8 {
    type Error = ();

    fn try_from(value: &BigUInt) -> Result<u8, Self::Error> {
        if value.digits.len() > 1 {
            return Err(());
        }
        let digit = value.digits.first().unwrap_or(&0);
        (*digit).try_into().map_err(|_| ())
    }
}

/// WARNING: These tests are *lazily written*, so they don't cover MANY cases they better should
/// cover. Please, do not assume these are enough; the real test is running based_encoding on some
/// data with this big integer and seeing if it will work properly
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(BigUInt::from(5).add(&BigUInt::from(15)).digits, vec![20]);
        assert_eq!(
            BigUInt::from(5).add(&BigUInt::from(usize::MAX)).digits,
            vec![4, 1],
        );
        assert_eq!(
            BigUInt::from(usize::MAX)
                .add(&BigUInt::from(usize::MAX))
                .digits,
            vec![usize::MAX - 1, 1],
        );
        assert_eq!(
            BigUInt::from(usize::MAX)
                .add(&BigUInt::from(usize::MAX))
                .add(&BigUInt::from(usize::MAX))
                .digits,
            vec![usize::MAX - 2, 2],
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            BigUInt::from(1).subtract(&BigUInt::from(1)).unwrap().digits,
            vec![],
        );
        assert_eq!(
            BigUInt::from(3).subtract(&BigUInt::from(1)).unwrap().digits,
            vec![2],
        );
        assert_eq!(BigUInt::from(1).subtract(&BigUInt::from(2)), None);
        assert_eq!(
            BigUInt {
                digits: vec![usize::MAX - 1, 1]
            }
            .subtract(&BigUInt::from(usize::MAX))
            .unwrap()
            .digits,
            vec![usize::MAX],
        );
    }
}
