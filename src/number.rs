use num_bigint::BigUint;

pub trait Readable {
    fn read(&mut self, limit: &BigUint) -> BigUint;
}

impl Readable for BigUint {
    fn read(&mut self, limit: &BigUint) -> BigUint {
        let output = &*self % limit;
        *self /= limit;
        output
    }
}

pub trait Writeable {
    fn write(&mut self, value: &BigUint, limit: &BigUint);
}

impl Writeable for BigUint {
    fn write(&mut self, value: &BigUint, limit: &BigUint) {
        *self = &*self * limit + value;
    }
}
