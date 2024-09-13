#[derive(Clone, PartialEq, Eq)]
pub struct BigInt {
    // Actually these aren't even regular digits, the new one appears when the last one reaches its
    // maximum value, and then that other digit isn't reset to "0". It's done to keep the
    // implementation simple, but it takes much more space in memory than it could have taken with regular digits
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

    pub fn decrement(mut self) -> Option<Self> {
        let last = self.digits.pop()?;
        if last != 0 {
            self.digits.push(last - 1);
        }
        Some(self)
    }

    pub fn increment(&mut self) {
        match self.digits.last_mut() {
            None => self.digits.push(0),
            Some(digit) => match digit.checked_add(1) {
                Some(new_value) => *digit = new_value,
                None => self.digits.push(0),
            },
        }
    }

    pub fn subtract(mut self, mut other: Self) -> Option<Self> {
        while let Some(new_other) = other.decrement() {
            other = new_other;
            self = self.decrement()?;
        }
        Some(self)
    }

    pub fn add(&mut self, mut other: Self) {
        while let Some(new_other) = other.decrement() {
            other = new_other;
            self.increment();
        }
    }

    pub fn multiply(&mut self, mut factor: Self) {
        let mut result = Self::zero();
        while let Some(new_other) = factor.decrement() {
            factor = new_other;
            result.add(Clone::clone(self));
        }
        *self = result;
    }

    pub fn divide(mut self, other: Self) -> DivisionResult {
        let mut amount = Self::zero();
        let remainder = loop {
            match self.clone().subtract(other.clone()) {
                None => break self,
                Some(new_self) => {
                    amount.increment();
                    self = new_self;
                }
            }
        };
        DivisionResult { amount, remainder }
    }
}

impl From<usize> for BigInt {
    fn from(mut value: usize) -> Self {
        let mut result = Self::zero();
        while value != 0 {
            result.increment();
            value -= 1;
        }
        result
    }
}
