mod number;
use num_bigint::BigUint;

use number::{read, write, Base, Digit};

mod shortcuts {
    use super::*;

    pub fn biguint(value: u32) -> BigUint {
        BigUint::new(vec![value])
    }
    pub fn base(value: u32) -> Base {
        Base::new(biguint(value)).unwrap()
    }
    pub fn digit(value: u32, base: Base) -> Digit {
        Digit::new(biguint(value), base).unwrap()
    }
}
use shortcuts as s;

/// Supposed to be between 0 and 17
mod number_a {
    use super::*;

    pub struct NumberA {
        digit: Digit,
    }
    impl NumberA {
        pub fn new(value: u32) -> Option<Self> {
            Digit::new(s::biguint(value), Self::base()).map(|digit| Self { digit })
        }
        pub fn base() -> Base {
            s::base(18)
        }
        pub fn encode(&self, number: &mut BigUint) {
            write(number, &self.digit);
        }
        pub fn decode(number: &mut BigUint) -> Self {
            let num = read(number, &Self::base());
            Self {
                digit: Digit::new(num, Self::base()).unwrap(),
            }
        }
    }
}
use number_a::NumberA;

/// Supposed to be between 0 and 543
mod number_b {
    use super::*;

    pub struct NumberB {
        digit: Digit,
    }
    impl NumberB {
        pub fn new(value: u32) -> Option<Self> {
            Digit::new(s::biguint(value), Self::base()).map(|digit| Self { digit })
        }
        pub fn base() -> Base {
            s::base(544)
        }
        pub fn encode(&self, number: &mut BigUint) {
            write(number, &self.digit);
        }
        pub fn decode(number: &mut BigUint) -> Self {
            let num = read(number, &Self::base());
            Self {
                digit: Digit::new(num, Self::base()).unwrap(),
            }
        }
    }
}
use number_b::NumberB;

struct Struct {
    number_a: NumberA,
    number_b: NumberB,
}

impl Struct {
    fn base() -> Base {
        NumberA::base()
    }
    fn encode(&self, number: &mut BigUint) {
        self.number_b.encode(number);
        self.number_a.encode(number);
    }
    fn decode(number: &mut BigUint) -> Self {
        let number_a = NumberA::decode(number);
        let number_b = NumberB::decode(number);
        Self { number_a, number_b }
    }
}

#[derive(Debug)]
enum SequenceItem {
    A(Struct),
    B(),
    C(Struct),
}

impl SequenceItem {
    fn base() -> Base {
        s::base(3)
    }
    fn encode(&self, number: &mut BigUint) {
        match self {
            Self::A(struct_) => {
                struct_.encode(number);
                write(number, &s::digit(0, Self::base()));
            }
            Self::B() => write(number, &s::digit(1, Self::base())),
            Self::C(struct_) => {
                struct_.encode(number);
                write(number, &s::digit(2, Self::base()));
            }
        }
    }
    fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == s::biguint(0) {
            let struct_ = Struct::decode(number);
            Self::A(struct_)
        } else if num == s::biguint(1) {
            Self::B()
        } else if num == s::biguint(2) {
            let struct_ = Struct::decode(number);
            Self::C(struct_)
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct Sequence {
    pub items: Vec<SequenceItem>,
}
impl Sequence {
    fn base() -> Base {
        Base::new(SequenceItem::base().value() + 1u8).unwrap()
    }
    fn encode(&self, number: &mut BigUint) {
        write(number, &s::digit(0, Self::base()));
        for item in &self.items {
            item.encode(number);
            let item = read(number, &SequenceItem::base());
            write(number, &Digit::new(item + 1u8, Self::base()).unwrap());
        }
    }
    fn decode(number: &mut BigUint) -> Self {
        let mut items = Vec::new();
        loop {
            let num = read(number, &Self::base());
            if num == s::biguint(0) {
                break;
            }
            write(
                number,
                &Digit::new(num - 1u8, SequenceItem::base()).unwrap(),
            );
            items.push(SequenceItem::decode(number));
        }
        items.reverse();
        Self { items }
    }
}

fn main() {
    let sequence = Sequence {
        items: vec![
            SequenceItem::A(Struct {
                number_a: NumberA { value: 5 },
            }),
            SequenceItem::C(),
            SequenceItem::A(),
            SequenceItem::B(),
        ],
    };
    let mut number = BigUint::ZERO.clone();
    sequence.encode(&mut number);
    println!("{:?}", Sequence::decode(&mut number));
}
