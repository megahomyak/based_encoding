use crate::number::{read, write, Base, Digit};
use num_bigint::BigUint;
#[derive(Debug)]
pub struct NumberA {
    pub value: BigUint,
}
impl NumberA {
    #[allow(dead_code)]
    pub fn base() -> Base {
        Base {
            value: BigUint::from(18usize),
        }
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(
            number,
            &Digit {
                value: self.value.clone(),
                base: Self::base(),
            },
        );
    }
    pub fn decode(number: &mut BigUint) -> Self {
        Self {
            value: read(number, &Self::base()),
        }
    }
}
#[derive(Debug)]
pub struct MyString {
    pub contents: String,
}
impl MyString {
    #[allow(dead_code)]
    pub fn base() -> Base {
        Base {
            value: BigUint::from(256usize),
        }
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(
            number,
            &Digit {
                value: BigUint::ZERO,
                base: Self::base(),
            },
        );
        for byte in self.contents.as_bytes().iter().rev() {
            write(
                number,
                &Digit {
                    value: BigUint::from(*byte),
                    base: Self::base(),
                },
            );
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut bytes = Vec::new();
        loop {
            let byte = read(number, &Self::base());
            if byte == BigUint::ZERO {
                break;
            }
            bytes.push(u8::try_from(byte).unwrap());
        }
        Self {
            contents: String::from_utf8(bytes).unwrap(),
        }
    }
}
#[derive(Debug)]
pub struct Struct {
    pub number_a: NumberA,
    pub my_string: MyString,
}
impl Struct {
    #[allow(dead_code)]
    pub fn base() -> Base {
        NumberA::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.my_string.encode(number);
        self.number_a.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let number_a = NumberA::decode(number);
        let my_string = MyString::decode(number);
        Self {
            number_a,
            my_string,
        }
    }
}
#[derive(Debug)]
pub enum Enum {
    A(Struct),
    B(),
    C(Struct),
}
impl Enum {
    #[allow(dead_code)]
    pub fn base() -> Base {
        Base {
            value: BigUint::from(3usize),
        }
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::A(associated_value) => {
                associated_value.encode(number);
                write(
                    number,
                    &Digit {
                        value: BigUint::from(0usize),
                        base: Self::base(),
                    },
                );
            }
            Self::B() => {
                write(
                    number,
                    &Digit {
                        value: BigUint::from(1usize),
                        base: Self::base(),
                    },
                );
            }
            Self::C(associated_value) => {
                associated_value.encode(number);
                write(
                    number,
                    &Digit {
                        value: BigUint::from(2usize),
                        base: Self::base(),
                    },
                );
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let associated_value = Struct::decode(number);
            return Self::A(associated_value);
        }
        if num == BigUint::from(1usize) {
            return Self::B();
        }
        if num == BigUint::from(2usize) {
            let associated_value = Struct::decode(number);
            return Self::C(associated_value);
        }
        unreachable!()
    }
}
#[derive(Debug)]
pub struct Sequence {
    pub items: Vec<Enum>,
}
impl Sequence {
    #[allow(dead_code)]
    pub fn base() -> Base {
        Base {
            value: Enum::base().value + 1usize,
        }
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(
            number,
            &Digit {
                value: BigUint::ZERO,
                base: Self::base(),
            },
        );
        for item in self.items.iter().rev() {
            item.encode(number);
            let item = read(number, &Enum::base());
            write(
                number,
                &Digit {
                    value: item + 1usize,
                    base: Self::base(),
                },
            );
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut items = Vec::new();
        loop {
            let num = read(number, &Self::base());
            if num == BigUint::ZERO {
                break;
            }
            write(
                number,
                &Digit {
                    value: num - 1usize,
                    base: Enum::base(),
                },
            );
            items.push(Enum::decode(number));
        }
        Self { items }
    }
}
