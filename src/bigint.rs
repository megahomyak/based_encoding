pub struct BigInt {
    pub contents: Vec<usize>,
}

impl std::ops::Sub<usize> for BigInt {
    type Output = Option<Self>;

    fn sub(self, other: usize) -> Self::Output {
        match self.contents.pop() {
            None => todo!(),
            Some(n) => {

            }
        }
    }
}

impl std::ops::Add<usize> for BigInt {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {

    }
}
