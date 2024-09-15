pub struct DivisionResult<N> {
    pub quotient: N,
    pub remainder: N,
}

pub trait BigUInt: Sized + std::cmp::Ord + std::fmt::Debug + From<usize> {
    fn subtract(&self, other: &Self) -> Option<Self>;
    fn divide(&self, other: &Self) -> DivisionResult<Self>;
    fn add(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn try_into_u8(&self) -> Option<u8>;
}

impl BigUInt for num_bigint::BigUint {
    fn subtract(&self, other: &Self) -> Option<Self> {
        num_traits::CheckedSub::checked_sub(self, other)
    }
    fn divide(&self, other: &Self) -> DivisionResult<Self> {
        let (quotient, remainder) = num_integer::Integer::div_rem(self, other);
        DivisionResult {
            quotient,
            remainder,
        }
    }
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    fn multiply(&self, other: &Self) -> Self {
        self * other
    }
    fn try_into_u8(&self) -> Option<u8> {
        u8::try_from(self).ok()
    }
}
