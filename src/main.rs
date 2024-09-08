mod based_number;
use based_number::{BasedDigit, ValidBase, BasedNumber};
use num_bigint::BigUint;

fn main() {
    fn biguint(value: u32) -> BigUint {
        BigUint::new(vec![value])
    }
    fn valid_base(value: u32) -> ValidBase {
        ValidBase::new(biguint(value)).unwrap()
    }
    fn based_digit(base: u32, value: u32) -> BasedDigit {
        BasedDigit::new(valid_base(base), biguint(value)).unwrap()
    }
    let mut based_number = BasedNumber::new();
    based_number.write(based_digit(20, 15));
    based_number.write(based_digit(3, 2));
    based_number.write(based_digit(2, 1));
    println!("{}", based_number.read(valid_base(2)));
    println!("{}", based_number.read(valid_base(3)));
    println!("{}", based_number.read(valid_base(20)));
}
