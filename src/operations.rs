use num_bigint::BigUint;
use num_integer::Integer;

pub fn read(number: &mut BigUint, base: &BigUint) -> BigUint {
    let (quotient, remainder) = number.div_rem(base);
    *number = quotient;
    remainder
}

pub fn write(number: &mut BigUint, base: &BigUint, value: &BigUint) {
    #[cfg(debug_assertions)]
    if value >= base {
        panic!("Value too big for base: value {value:?} vs base {base:?}");
    }
    *number = &*number * base + value;
}
