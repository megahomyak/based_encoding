mod number;
use num_bigint::BigUint;

use number::{read, represent, write, Base, Digit};

/// Supposed to be between 0 and 17
#[derive(Debug)]
pub struct NumberA {
    value: BigUint,
}
impl NumberA {
    fn base() -> Base {
        Base {
            value: BigUint::from(18usize),
        }
    }
    fn encode(&self, number: &mut BigUint) {
        write(
            number,
            &Digit {
                value: self.value.clone(),
                base: Self::base(),
            },
        );
    }
    fn decode(number: &mut BigUint) -> Self {
        let value = read(number, &Self::base());
        Self { value }
    }
}

#[derive(Debug)]
struct MyString {
    content: String,
}
impl MyString {
    fn base() -> Base {
        Base {
            value: BigUint::from(256usize),
        }
    }
    fn encode(&self, number: &mut BigUint) {
        write(
            number,
            &Digit {
                base: Self::base(),
                value: BigUint::ZERO,
            },
        );
        for byte in self.content.as_bytes().iter().rev() {
            write(
                number,
                &Digit {
                    value: BigUint::from(*byte),
                    base: Self::base(),
                },
            );
        }
    }
    fn decode(number: &mut BigUint) -> Self {
        let mut bytes = Vec::new();
        loop {
            let byte = read(number, &Self::base());
            if byte == BigUint::ZERO {
                break;
            }
            bytes.push(u8::try_from(byte).unwrap());
        }
        Self {
            content: String::from_utf8(bytes).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Struct {
    number_a: NumberA,
    my_string: MyString,
}
impl Struct {
    #[allow(dead_code)]
    fn base() -> Base {
        NumberA::base()
    }
    fn encode(&self, number: &mut BigUint) {
        self.my_string.encode(number);
        self.number_a.encode(number);
    }
    fn decode(number: &mut BigUint) -> Self {
        let number_a = NumberA::decode(number);
        let my_string = MyString::decode(number);
        Self {
            number_a,
            my_string,
        }
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
        Base {
            value: BigUint::from(3usize),
        }
    }
    fn encode(&self, number: &mut BigUint) {
        match self {
            Self::A(struct_) => {
                struct_.encode(number);
                write(
                    number,
                    &Digit {
                        value: BigUint::from(0usize),
                        base: Self::base(),
                    },
                );
            }
            Self::B() => write(
                number,
                &Digit {
                    value: BigUint::from(1usize),
                    base: Self::base(),
                },
            ),
            Self::C(struct_) => {
                struct_.encode(number);
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
    fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let struct_ = Struct::decode(number);
            return Self::A(struct_);
        }
        if num == BigUint::from(1usize) {
            return Self::B();
        }
        if num == BigUint::from(2usize) {
            let struct_ = Struct::decode(number);
            return Self::C(struct_);
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct Sequence {
    pub items: Vec<SequenceItem>,
}
impl Sequence {
    fn base() -> Base {
        Base {
            value: SequenceItem::base().value + 1usize,
        }
    }
    fn encode(&self, number: &mut BigUint) {
        write(
            number,
            &Digit {
                base: Self::base(),
                value: BigUint::ZERO,
            },
        );
        for item in self.items.iter().rev() {
            item.encode(number);
            let item = read(number, &SequenceItem::base());
            write(
                number,
                &Digit {
                    value: item + 1usize,
                    base: Self::base(),
                },
            );
        }
    }
    fn decode(number: &mut BigUint) -> Self {
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
                    base: SequenceItem::base(),
                },
            );
            items.push(SequenceItem::decode(number));
        }
        Self { items }
    }
}

fn main() {
    let sequence = Sequence {
        items: vec![
            SequenceItem::A(Struct {
                number_a: NumberA {
                    value: BigUint::from(5usize),
                },
                my_string: MyString {
                    content: String::from("hello!"),
                },
            }),
            SequenceItem::C(Struct {
                number_a: NumberA {
                    value: BigUint::from(10usize),
                },
                my_string: MyString {
                    content: String::from("hi!"),
                },
            }),
            SequenceItem::A(Struct {
                number_a: NumberA {
                    value: BigUint::from(0usize),
                },
                my_string: MyString {
                    content: String::from("*laughter*"),
                },
            }),
            SequenceItem::B(),
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
