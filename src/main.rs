mod based_number {
    /// A number that is smaller or equal to `BASE`
    pub struct BasedDigit<const BASE: usize> {
        value: usize,
    }

    impl<const BASE: usize> BasedDigit<BASE> {
        pub fn new(value: usize) -> Option<Self> {
            if value > BASE {
                None
            } else {
                Some(Self { value })
            }
        }

        pub fn value(&self) -> usize {
            return self.value;
        }
    }

    pub struct BasedNumber {
        digits: Vec<usize>,
    }

    impl BasedNumber {
        pub fn new() -> Self {
            Self { digits: Vec::new() }
        }

        pub fn push(&mut self, digit: BasedDigit<BASE>) {
            self.digits.push(digit.value);
        }

        pub fn digits<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
            self.digits.iter().by_ref.rev()
        }

        pub fn convert<const NEW_BASE: usize>(&self) -> BasedNumber<NEW_BASE> {
            let mut self_as_number: usize = 0;
            for (index, digit) in self.digits.iter().enumerate() {
                self_as_number += BASE.pow(index as u32) * digit;
            }
            let mut digits = Vec::new();
            while self_as_number != 0 {
                digits.push(self_as_number % NEW_BASE);
                self_as_number /= NEW_BASE;
            }
            BasedNumber { digits }
        }
    }
}
use based_number::{BasedNumber, BasedDigit};

pub struct BinaryString {
    pub content: Vec<bool>,
}

fn encode(digits: Vec<BasedDigit>) -> BinaryString {

}

fn main() {
    println!("Hello, world!");
}
