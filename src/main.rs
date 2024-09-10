mod entities;
mod number;
use num_bigint::BigUint;

use entities::{Enum, MyString, NumberA, Sequence, Struct};
use number::{represent, Base};

fn main() {
    let sequence = Sequence {
        items: vec![
            Enum::A(Struct {
                number_a: NumberA {
                    value: BigUint::from(5usize),
                },
                my_string: MyString {
                    contents: String::from("hello!"),
                },
            }),
            Enum::C(Struct {
                number_a: NumberA {
                    value: BigUint::from(10usize),
                },
                my_string: MyString {
                    contents: String::from("hi!"),
                },
            }),
            Enum::A(Struct {
                number_a: NumberA {
                    value: BigUint::from(0usize),
                },
                my_string: MyString {
                    contents: String::from("*laughter*"),
                },
            }),
            Enum::B(),
        ],
    };
    let mut number = BigUint::ZERO;
    sequence.encode(&mut number);
    let sequence = Sequence::decode(&mut number.clone());
    println!("{:?}", sequence);
    println!(
        "{:?}",
        represent(
            number.clone(),
            Base {
                value: BigUint::from(256usize)
            }
        )
    );
    println!(
        "{:?}",
        represent(
            number.clone(),
            Base {
                value: BigUint::from(2usize)
            }
        )
    );
}
