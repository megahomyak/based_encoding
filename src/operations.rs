use crate::biguint::BigUInt;

pub fn read<N: BigUInt>(number: &mut N, base: &N) -> N {
    let division_result = number.divide(base);
    *number = division_result.quotient;
    division_result.remainder
}

pub fn write<N: BigUInt>(number: &mut N, base: &N, value: &N) {
    #[cfg(debug_assertions)]
    if value >= base {
        panic!("Value too big for base: value {value:?} vs base {base:?}");
    }
    *number = number.multiply(base).add(value);
}
