use crate::based_number::{BasedDigit, Digit, ValidBase};

pub trait NumberInput {
    fn read(&mut self, base: &ValidBase) -> Digit;
}

pub trait Entity {
    type Input;

    fn encode(&self, input: Self::Input) -> impl Iterator<Item = BasedDigit>;
    fn decode(&self, number_input: impl NumberInput) -> Self::Input;
    fn base(&self) -> &ValidBase;
}

pub struct Struct<E, const N: usize> {
    pub rest: [E; N],
    pub last: E,
}

impl<E: Entity, const N: usize> Entity for Struct<E, N> {
    type Input = ([E; N], E);

    fn base(&self) -> &ValidBase {
        self.last.base()
    }
    fn encode(&self, input: Self::Input) -> impl Iterator<Item = BasedDigit> {
    }
    fn decode(&self, number_input: impl NumberInput) -> Self::Input {
        
    }
}
