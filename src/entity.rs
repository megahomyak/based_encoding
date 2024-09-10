use crate::number::Base;
use num_bigint::BigUint;

pub trait Entity {
    fn base() -> Base;
    fn encode(&self, number: &mut BigUint);
    fn decode(number: &mut BigUint) -> Self;
}
