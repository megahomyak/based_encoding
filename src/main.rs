mod number;
use num_bigint::BigUint;

use number::{Base, Digit, Readable, Writeable};

fn main() {
    fn biguint(value: u32) -> BigUint {
        BigUint::new(vec![value])
    }
    fn base(value: u32) -> Base {
        Base::new(biguint(value)).unwrap()
    }
    fn digit(value: u32, base: Base) -> Digit {
        Digit::new(biguint(value), base).unwrap()
    }
    let mut number = BigUint::ZERO.clone();
    number.write(&digit(15, base(20)));
    number.write(&digit(2, base(3)));
    number.write(&digit(1, base(2)));
    println!("{}", number.read(&base(2)));
    println!("{}", number.read(&base(3)));
    println!("{}", number.read(&base(20)));
}
