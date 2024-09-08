use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ValidBase {
    value: usize,
}
impl ValidBase {
    pub fn new(value: usize) -> Option<Self> {
        (value > 0).then(|| Self { value })
    }
    pub fn value(&self) -> usize {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BasedDigit {
    base: ValidBase,
    value: usize,
}
impl BasedDigit {
    pub fn new(base: ValidBase, value: usize) -> Option<Self> {
        (base.value > value).then(|| BasedDigit { base, value })
    }
}

#[derive(Debug)]
pub struct BasedNumber {
    digits: Vec<usize>,
    base: ValidBase,
}
impl BasedNumber {
    pub fn new() -> Self {
        Self {
            digits: Vec::new(),
            base: ValidBase { value: 2 },
        }
    }

    pub fn convert(&mut self, new_base: ValidBase) {
        fn pow(mut target: BigUint, mut power: usize) -> BigUint {
            let initial_base = target.clone();
            while power != 0 {
                target *= &initial_base;
                power -= 1;
            }
            target
        }
        if new_base == self.base {
            return;
        }
        let mut total = BigUint::ZERO;
        {
            let mut index = 0;
            while let Some(digit) = self.digits.pop() {
                total += pow(BigUint::from(self.base.value), index) * digit;
                index += 1;
            }
        }
        while total != BigUint::ZERO {
            self.digits
                .push((&total % self.base.value).try_into().unwrap());
            total /=  self.base.value;
        }
        self.digits.reverse();
        self.base = new_base;
    }

    pub fn read(&mut self, base: ValidBase) -> usize {
        self.convert(base);
        self.digits.pop().unwrap_or(0)
    }

    pub fn write(&mut self, based_digit: BasedDigit) {
        self.convert(based_digit.base);
        self.digits.push(based_digit.value);
    }

    pub fn base(&self) -> ValidBase {
        self.base
    }

    pub fn digits(&self) -> &[usize] {
        &self.digits
    }
}
