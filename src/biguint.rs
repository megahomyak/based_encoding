pub struct DivisionResult {
    pub quotient: BigUInt,
    pub remainder: BigUInt,
}

#[derive(Clone)]
pub struct BigUInt {
    value: num_bigint::BigUint,
}

impl BigUInt {
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        num_traits::CheckedSub::checked_sub(&self.value, &other.value).map(|value| Self { value })
    }
    pub fn divide(&self, other: &Self) -> DivisionResult {
        let (quotient, remainder) = num_integer::Integer::div_rem(&self.value, &other.value);
        DivisionResult {
            quotient: Self { value: quotient },
            remainder: Self { value: remainder },
        }
    }
}

impl std::ops::Mul for &BigUInt {
    type Output = BigUInt;

    fn mul(self, rhs: Self) -> Self::Output {
        BigUInt {
            value: &self.value * &rhs.value,
        }
    }
}

impl std::ops::Add for &BigUInt {
    type Output = BigUInt;

    fn add(self, rhs: Self) -> Self::Output {
        BigUInt {
            value: &self.value + &rhs.value,
        }
    }
}

impl std::cmp::PartialEq for &BigUInt {
    fn eq(&self, other: &Self) -> bool {
        std::cmp::PartialEq::eq(&self.value, &other.value)
    }
}
impl std::cmp::Eq for &BigUInt {}

impl std::cmp::PartialOrd for &BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        std::cmp::PartialOrd::partial_cmp(&self.value, &other.value)
    }
}
impl std::cmp::Ord for &BigUInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ord::cmp(&self.value, &other.value)
    }
}

impl TryFrom<&BigUInt> for u8 {
    type Error = ();

    fn try_from(biguint: &BigUInt) -> Result<Self, Self::Error> {
        u8::try_from(&biguint.value).map_err(|_| ())
    }
}

impl std::fmt::Debug for BigUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.value, f)
    }
}

impl From<usize> for BigUInt {
    fn from(value: usize) -> Self {
        Self {
            value: value.into(),
        }
    }
}
