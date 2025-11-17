pub trait ResultMonadError<T0, E0> {
    fn flat_map<T1, E1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> Result<T1, E1>,
        E1: Into<E0>;

    fn map<T1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1;

    #[inline]
    fn m_product<T1, E1, F>(self, f: F) -> Result<(T0, T1), E0>
    where
        Self: Sized,
        F: FnOnce(&T0) -> Result<T1, E1>,
        E1: Into<E0>,
    {
        self.flat_map(|t0| f(&t0).map(|t1| (t0, t1)))
    }

    #[inline]
    fn f_product<T1, F>(self, f: F) -> Result<(T0, T1), E0>
    where
        Self: Sized,
        F: FnOnce(&T0) -> T1,
    {
        self.map(|t0| {
            let t1 = f(&t0);
            (t0, t1)
        })
    }

    #[inline]
    fn product<T1, E1>(self, other: Result<T1, E1>) -> Result<(T0, T1), E0>
    where
        Self: Sized,
        E1: Into<E0>,
    {
        self.flat_map(|t0| other.map(|t1| (t0, t1)))
    }

    #[inline]
    fn m_as<T1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce() -> T1,
    {
        self.map(|_| f())
    }

    #[inline]
    fn void(self) -> Result<(), E0>
    where
        Self: Sized,
    {
        self.m_as(|| ())
    }
}

impl<T0, E0> ResultMonadError<T0, E0> for Result<T0, E0> {
    #[inline]
    fn flat_map<T1, E1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> Result<T1, E1>,
        E1: Into<E0>,
    {
        self.and_then(|t| f(t).map_err(|e| e.into()))
    }

    #[inline]
    fn map<T1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1,
    {
        self.map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::ResultMonadError;
    use crate::test_errors::*;

    #[test]
    fn flat_map() {
        // This won't work because type has to be know at first call to flat_map
        // Ok(4).flat_map(|x| Ok(x + 4));
        assert_eq!(
            Ok::<u8, Error1>(4).flat_map(|x| Ok::<u8, Error2>(x + 4)),
            Ok(8)
        );
        assert_eq!(
            Ok::<u8, Error1>(4).flat_map(|_| Err::<u8, Error2>(Error2::new("Test"))),
            Err(Error1::new("Converted Error2(Test)"))
        );

        assert_eq!(
            Err(Error1::new("Test1")).flat_map(|x: u8| Ok::<u8, Error2>(x + 4)),
            Err(Error1::new("Test1"))
        );
    }

    #[test]
    fn m_product() {
        assert_eq!(
            Ok::<u8, Error1>(3).m_product(|x| Ok::<u8, Error2>(x + 1)),
            Ok((3, 4))
        );
        assert_eq!(
            Ok(3).m_product(|_| Err::<u8, Error2>(Error2::new("Test"))),
            Err(Error1::new("Test"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).m_product(|x| Ok::<u8, Error2>(x + 1)),
            Err(Error1::new("Test"))
        );
    }

    #[test]
    fn f_product() {
        assert_eq!(Ok::<u8, Error1>(3).f_product(|x| x + 1), Ok((3, 4)));
        assert_eq!(
            Err(Error1::new("Test1")).f_product(|x| x + 1),
            Err(Error1::new("Test1"))
        );
    }

    #[test]
    fn product() {
        assert_eq!(Ok::<u8, Error1>(3).product(Ok::<u8, Error2>(4)), Ok((3, 4)));
        assert_eq!(
            Ok::<u8, Error1>(3).product(Err::<u8, Error2>(Error2::new("Test"))),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).product(Ok::<u8, Error2>(4)),
            Err(Error1::new("Test"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).product(Err::<u8, Error2>(Error2::new("Test"))),
            Err(Error1::new("Test"))
        );
    }
}
