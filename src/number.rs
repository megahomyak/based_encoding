use num_bigint::BigUint;

#[derive(Debug)]
pub struct Base {
    pub value: BigUint,
}

fn validate_base(base: &Base) {
    if base.value == BigUint::ZERO {
        panic!("Base cannot be zero");
    }
}

#[derive(Debug)]
pub struct Digit {
    pub base: Base,
    pub value: BigUint,
}

fn validate_digit(digit: &Digit) {
    validate_base(&digit.base);
    if digit.value >= digit.base.value {
        panic!("Value too big for base");
    }
}

pub fn read(number: &mut BigUint, base: &Base) -> BigUint {
    #[cfg(debug_assertions)]
    validate_base(base);
    let output = &*number % &base.value;
    *number /= &base.value;
    output
}

pub fn write(number: &mut BigUint, digit: &Digit) {
    #[cfg(debug_assertions)]
    validate_digit(digit);
    *number = &*number * &digit.base.value + &digit.value;
}

pub fn represent(mut number: BigUint, base: Base) -> Vec<BigUint> {
    #[cfg(debug_assertions)]
    validate_base(&base);
    let mut digits = Vec::new();
    while number != BigUint::ZERO {
        digits.push(read(&mut number, &base));
    }
    digits
}
