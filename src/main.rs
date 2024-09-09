mod number;
use num_bigint::BigUint;
use number::{ExclusiveUpperLimit, LimitedNumber, Number, Readable, Writeable};

fn main() {
    fn biguint(value: u32) -> BigUint {
        BigUint::new(vec![value])
    }
    fn limit(value: u32) -> ExclusiveUpperLimit {
        ExclusiveUpperLimit::new(biguint(value)).unwrap()
    }
    fn limited_number(value: u32, limit_: u32) -> LimitedNumber {
        LimitedNumber::new(biguint(value), limit(limit_)).unwrap()
    }
    let mut number = Number::new();
    number.write(&limited_number(15, 20));
    number.write(&limited_number(2, 3));
    number.write(&limited_number(1, 2));
    println!("{}", number.read(&limit(2)));
    println!("{}", number.read(&limit(3)));
    println!("{}", number.read(&limit(20)));
}
