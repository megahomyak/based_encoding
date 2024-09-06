pub struct BigInt {}

impl std::ops::Sub<usize> for BigInt {
    type Output = Option<Self>;

    fn sub(self, rhs: usize) -> Self::Output {

    }
}

impl std::ops::Add<usize> for BigInt {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {

    }
}
