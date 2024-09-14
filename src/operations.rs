use crate::bigint::BigUInt;

pub fn read(number: &mut BigUInt, base: &BigUInt) -> BigUInt {
    let division_result = number.divide(base);
    *number = division_result.quotient;
    division_result.remainder
}

pub fn write(number: &mut BigUInt, base: &BigUInt, value: &BigUInt) {
    #[cfg(debug_assertions)]
    if value >= base {
        panic!("Value too big for base: value {value:?} vs base {base:?}");
    }
    *number = number.multiply(base).add(value);
}
