use num_bigint::BigUint;

pub struct ExclusiveUpperLimit {
    value: BigUint,
}

impl ExclusiveUpperLimit {
    pub fn new(value: BigUint) -> Option<Self> {
        if value > BigUint::ZERO {
            Some(Self { value })
        } else {
            None
        }
    }
}

pub struct LimitedNumber {
    value: BigUint,
    exclusive_upper_limit: ExclusiveUpperLimit,
}

impl LimitedNumber {
    pub fn new(value: BigUint, exclusive_upper_limit: ExclusiveUpperLimit) -> Option<Self> {
        if value < exclusive_upper_limit.value {
            Some(Self { value, exclusive_upper_limit })
        } else {
            None
        }
    }
}

pub struct Number {
    pub value: BigUint,
}

impl Number {
    pub fn new() -> Self {
        Self { value: BigUint::ZERO.clone() }
    }

    pub fn read(&mut self, upper_limit: &ExclusiveUpperLimit) -> BigUint {
        let output = &self.value % &upper_limit.value;
        self.value /= &upper_limit.value;
        output
    }

    pub fn write(&mut self, limited_number: &LimitedNumber) {
        self.value = &self.value * &limited_number.exclusive_upper_limit.value + &limited_number.value;
    }
}
