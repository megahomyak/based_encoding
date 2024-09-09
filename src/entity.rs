use crate::based_number::{BasedDigit, BasedNumber, Digit, ValidBase};

pub trait NumberInput {
    fn read(&mut self, base: &ValidBase) -> Digit;
}

impl NumberInput for BasedNumber {
    fn read(&mut self, base: &ValidBase) -> Digit {
        let a = num_bigint::BigUint::new(vec![]);
        let a = a.iter_u32_digits().next().unwrap();
        let a: u8 = a.try_into().unwrap();
        BasedNumber::read(self, base)
    }
}

pub trait Entity {
    fn encode(self) -> impl Iterator<Item = BasedDigit>;
    fn decode(number_input: impl NumberInput) -> Self;
    fn base(&self) -> &ValidBase;
}
