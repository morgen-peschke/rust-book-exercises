pub trait Semigroup {
    fn combine(self, rhs: Self) -> Self;
}

impl Semigroup for String {
    fn combine(self, rhs: Self) -> Self {
        self + &rhs
    }
}

impl<L: Semigroup, R: Semigroup> Semigroup for (L, R) {
    fn combine(self, rhs: Self) -> Self {
        (self.0.combine(rhs.0), self.1.combine(rhs.1))
    }
}

impl<T: Semigroup, E: Semigroup> Semigroup for Result<T, E> {
    fn combine(self, rhs: Result<T, E>) -> Result<T, E> {
        match self {
            Ok(l) => match rhs {
                Ok(r) => Ok(l.combine(r)),
                Err(e) => Err(e),
            },
            Err(l) => match rhs {
                Ok(_) => Err(l),
                Err(r) => Err(l.combine(r)),
            },
        }
    }
}
