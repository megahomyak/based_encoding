pub(self) use num_bigint::BigUint;

// NEVER fucking make this public
pub(self) type BigEndian<T> = T;
pub type Digit = BigUint;

pub(self) fn one() -> BigUint {
    BigUint::new(vec![1])
}

#[derive(PartialEq, Eq, Clone)]
pub struct ValidBase {
    pub(self) value: BigUint,
}

impl ValidBase {
    pub fn new(value: BigUint) -> Option<Self> {
        if value > BigUint::ZERO {
            Some(Self { value })
        } else {
            None
        }
    }

    pub(self) fn pow(&self, base: usize) -> Self {
        let mut value = one();
        for _ in 0..base {
            value *= &self.value;
        }
        Self { value }
    }
}

#[derive(Clone)]
pub struct BasedDigit {
    pub(self) base: ValidBase,
    pub(self) value: BigUint,
}

impl BasedDigit {
    pub fn new(base: ValidBase, value: BigUint) -> Option<Self> {
        if base.value > value {
            Some(Self { base, value })
        } else {
            None
        }
    }
}

pub struct BasedNumber {
    pub(self) base: ValidBase,
    pub(self) digits: BigEndian<Vec<BigUint>>,
}

impl BasedNumber {
    pub fn new() -> Self {
        Self {
            base: ValidBase { value: one() },
            digits: vec![],
        }
    }

    pub fn convert(&mut self, new_base: &ValidBase) {
        if new_base == &self.base {
            return;
        }
        let mut self_as_number = BigUint::ZERO;

        for (index, digit) in self.digits.iter().rev().enumerate() {
            self_as_number += self.base.pow(index).value * digit;
        }

        let mut reversed_digits = Vec::new();
        while self_as_number != BigUint::ZERO {
            reversed_digits.push(&self_as_number % &new_base.value);
            self_as_number /= &new_base.value;
        }

        self.base = new_base.clone();
        reversed_digits.reverse();
        self.digits = reversed_digits;
    }

    pub fn write(&mut self, based_digit: BasedDigit) {
        if self.digits.is_empty() && based_digit.value == BigUint::ZERO {
            return;
        }
        self.convert(&based_digit.base);
        self.digits.push(based_digit.value);
    }

    pub fn read(&mut self, base: &ValidBase) -> Digit {
        self.convert(base);
        self.digits.pop().unwrap_or(BigUint::ZERO)
    }
}
