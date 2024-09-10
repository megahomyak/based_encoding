mod uint {
    use num_bigint::BigUint;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct Uint {
        value: BigUint,
    }
    impl Uint {
        pub fn new(value: BigUint) -> Self {
            Self { value }
        }
        pub fn increment(&self) -> Uint {
            Uint {
                value: &self.value + 1u8,
            }
        }
        pub fn decrement(&self) -> Option<Uint> {
            if self.is_zero() {
                None
            } else {
                Some(Uint {
                    value: &self.value - 1u8,
                })
            }
        }
        pub fn is_zero(&self) -> bool {
            self.value == BigUint::ZERO
        }
    }
    impl std::fmt::Debug for Uint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.value, f)
        }
    }
    impl std::ops::Rem for Uint {
        type Output = Self;

        fn rem(self, rhs: Self) -> Self::Output {
            Self {
                value: std::ops::Rem::rem(rhs),
            }
        }
    }
}
pub use uint::Uint;

#[derive(Debug)]
pub struct Base {
    value: Uint,
}
impl Base {
    pub fn new(value: Uint) -> Option<Self> {
        if value.is_zero() {
            None
        } else {
            Some(Self { value })
        }
    }

    pub fn increment(&self) -> Self {
        Base {
            value: self.value.increment(),
        }
    }
}

#[derive(Debug)]
pub struct Digit {
    base: Base,
    value: Uint,
}
impl Digit {
    pub fn new(value: Uint, base: Base) -> Option<Self> {
        if base.value > value {
            Some(Self { base, value })
        } else {
            None
        }
    }
}

pub fn read(number: &mut Uint, base: &Base) -> Uint {
    let output = &*number % &base.value;
    *number /= &base.value;
    output
}

pub fn write(number: &mut BigUint, digit: &Digit) {
    *number = &*number * &digit.base.value + &digit.value;
}
