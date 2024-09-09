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
}

pub trait Readable {
    fn read(&mut self, base: &Base) -> BigUint;
}

impl Readable for BigUint {
    fn read(&mut self, base: &Base) -> BigUint {
        let output = &*self % &base.value;
        *self /= &base.value;
        output
    }
}

pub trait Writeable {
    fn write(&mut self, digit: &Digit);
}

impl Writeable for BigUint {
    fn write(&mut self, digit: &Digit) {
        *self = &*self * &digit.base.value + &digit.value;
    }
}
