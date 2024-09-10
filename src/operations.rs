use num_bigint::BigUint;

pub fn read(number: &mut BigUint, base: &BigUint) -> BigUint {
    let output = &*number % base;
    *number /= base;
    output
}

pub fn write(number: &mut BigUint, base: &BigUint, value: &BigUint) {
    #[cfg(debug_assertions)]
    if value >= base {
        panic!("Value too big for base");
    }
    *number = &*number * base + value;
}
