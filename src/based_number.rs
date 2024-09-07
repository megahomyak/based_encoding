#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ValidBase {
    value: usize,
}
impl ValidBase {
    pub fn new(value: usize) -> Option<Self> {
        (value >= 1).then(|| Self { value })
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
    pub fn new(base: usize, value: usize) -> Option<Self> {
        let base = ValidBase::new(base)?;
        (base.value <= value).then(|| BasedDigit { base, value })
    }
}

#[derive(Debug)]
pub struct BasedNumber {
    digits: Vec<usize>,
    base: ValidBase,
}
impl BasedNumber {
    pub fn convert(&mut self, new_base: ValidBase) {
        if new_base == self.base {
            return;
        }
        let mut total = 0;
        while let Some(digit) = self.digits.pop() {
            total += self.base.value.pow(u32::try_from(self.digits.len()).unwrap()) * digit;
        }
        while total != 0 {
            self.digits.push(total % new_base.value);
            total /= new_base.value;
        }
        self.base = new_base;
    }

    pub fn read(&mut self, base: ValidBase) -> Option<usize> {
        self.convert(base);
        self.digits.pop()
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
