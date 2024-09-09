mod number;
use num_bigint::BigUint;

use number::{Readable, Writeable};

fn main() {
    fn biguint(value: u32) -> BigUint {
        BigUint::new(vec![value])
    }
    let mut number = BigUint::ZERO.clone();
    number.write(&biguint(15), &biguint(20));
    number.write(&biguint(2), &biguint(3));
    number.write(&biguint(1), &biguint(2));
    println!("{}", number.read(&biguint(2)));
    println!("{}", number.read(&biguint(3)));
    println!("{}", number.read(&biguint(20)));
}
