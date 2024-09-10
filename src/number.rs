use num_bigint::BigUint;

pub struct Base {
    value: BigUint,
}
impl Base {
    pub fn new(value: BigUint) -> Option<Self> {
        if value > BigUint::ZERO {
            Some(Self { value })
        } else {
            None
        }
    }

    pub fn value(&self) -> &BigUint {
        &self.value
    }
}

pub struct Digit {
    base: Base,
    value: BigUint,
}
impl Digit {
    pub fn new(value: BigUint, base: Base) -> Option<Self> {
        if base.value > value {
            Some(Self { base, value })
        } else {
            None
        }
    }

    pub fn value(&self) -> &BigUint {
        &self.value
    }

    pub fn base(&self) -> &Base {
        &self.base
    }
}

pub fn read(number: &mut BigUint, base: &Base) -> BigUint {
    let output = &*number % &base.value;
    *number /= &base.value;
    output
}

pub fn write(number: &mut BigUint, digit: &Digit) {
    *number = &*number * &digit.base.value + &digit.value;
}
