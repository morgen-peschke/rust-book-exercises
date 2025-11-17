pub trait ResultMonad<T0, E> {
    fn flat_map<T1, F>(self, f: F) -> Result<T1, E>
    where
        Self: Sized,
        F: FnOnce(T0) -> Result<T1, E>;

    fn map<T1, F>(self, f: F) -> Result<T1, E>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1;

    #[inline]
    fn m_product<T1, F>(self, f: F) -> Result<(T0, T1), E>
    where
        Self: Sized,
        F: FnOnce(&T0) -> Result<T1, E>,
    {
        self.flat_map(|t0| f(&t0).map(|t1| (t0, t1)))
    }

    #[inline]
    fn f_product<T1, F>(self, f: F) -> Result<(T0, T1), E>
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
    fn product<T1>(self, other: Result<T1, E>) -> Result<(T0, T1), E>
    where
        Self: Sized,
    {
        self.flat_map(|t0| other.map(|t1| (t0, t1)))
    }

    #[inline]
    fn m_as<T1, F>(self, f: F) -> Result<T1, E>
    where
        Self: Sized,
        F: FnOnce() -> T1,
    {
        self.map(|_| f())
    }

    #[inline]
    fn void(self) -> Result<(), E>
    where
        Self: Sized,
    {
        self.m_as(|| ())
    }
}

impl<T0, E> ResultMonad<T0, E> for Result<T0, E> {
    #[inline]
    fn flat_map<T1, F>(self, f: F) -> Result<T1, E>
    where
        Self: Sized,
        F: FnOnce(T0) -> Result<T1, E>,
    {
        self.and_then(f)
    }

    #[inline]
    fn map<T1, F>(self, f: F) -> Result<T1, E>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1,
    {
        self.map(f)
    }
}

#[cfg(test)]
mod tests_vanilla {
    use super::ResultMonad;
    use crate::test_errors::*;

    #[test]
    fn flat_map() {
        // This works because the error channel is fixed, so the
        // compiler can work backwards to figure it out.
        assert_eq!(Ok(4).flat_map(|x| Ok::<u8, Error2>(x + 4)), Ok(8));
        assert_eq!(
            Ok(4)
                .flat_map(|x| Ok::<u8, Error2>(x + 4))
                // Since the first flat_map fixes the error channel,
                // this one needs an into()
                .flat_map(|x| Ok::<u8, Error1>(x + 4).map_err(Error1::into)),
            Ok(12)
        );
        assert_eq!(
            Ok(4).flat_map(|_| { Err::<u8, Error2>(Error2::new("Test")).map_err(|e| e.into()) }),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err(Error1::new("Test1"))
                .flat_map(|x: u8| { Ok::<u8, Error2>(x + 4).map_err(Error2::into) }),
            Err(Error1::new("Test1"))
        );
    }

    #[test]
    fn m_product() {
        fn plus_one(x: u8) -> Result<u8, Error2> {
            Ok(x + 1)
        }
        fn plus_fail(_: u8) -> Result<u8, Error2> {
            Err(Error2::new("Test"))
        }
        assert_eq!(
            Ok::<u8, Error1>(3).m_product(|x| plus_one(*x).map_err(Error2::into)),
            Ok((3, 4))
        );
        assert_eq!(
            Ok(3).m_product(|x| plus_fail(*x).map_err(Error1::from)),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test"))
                .m_product(|x| plus_one(*x).map_err(Error1::from)),
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
        fn four() -> Result<u8, Error2> {
            Ok(4)
        }
        fn bad() -> Result<u8, Error2> {
            Err(Error2::new("Test"))
        }
        assert_eq!(
            Ok::<u8, Error1>(3).product(four().map_err(Error1::from)),
            Ok((3, 4))
        );
        assert_eq!(
            Ok::<u8, Error1>(3).product(bad().map_err(Error1::from)),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).product(four().map_err(Error1::from)),
            Err(Error1::new("Test"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).product(bad().map_err(Error1::from)),
            Err(Error1::new("Test"))
        );
    }
}

#[cfg(test)]
mod tests_hybrid {
    use super::ResultMonad;
    use crate::test_errors::*;

    #[test]
    fn flat_map() {
        fn plus_four_1(x: u8) -> Result<u8, Error1> {
            Ok(x + 4)
        }
        fn plus_four_2(x: u8) -> Result<u8, Error2> {
            Ok(x + 4)
        }
        fn fail_1(_: u8) -> Result<u8, Error1> {
            Err(Error1::new("Test"))
        }
        fn fail_2(_: u8) -> Result<u8, Error2> {
            Err(Error2::new("Test"))
        }
        assert_eq!(Ok::<u8, Error1>(4).flat_map(|x| Ok(x + 4)), Ok(8));
        assert_eq!(
            Ok(4)
                .flat_map(plus_four_1)
                .flat_map(|x| Ok(plus_four_2(x)?)),
            Ok(12)
        );
        assert_eq!(
            Ok(4).flat_map(fail_1).flat_map(|x| Ok(fail_2(x)?)),
            Err(Error1::new("Test"))
        );
        assert_eq!(
            Ok::<u8, Error1>(4)
                .flat_map(|x| Ok(fail_2(x)?))
                .flat_map(fail_1),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err(Error1::new("Test1")).flat_map(|x: u8| Ok(plus_four_2(x)?)),
            Err(Error1::new("Test1"))
        );
    }

    #[test]
    fn m_product() {
        fn plus_one(x: u8) -> Result<u8, Error2> {
            Ok(x + 1)
        }
        fn plus_fail(_: u8) -> Result<u8, Error2> {
            Err(Error2::new("Test"))
        }
        assert_eq!(
            Ok::<u8, Error1>(3).m_product(|x| Ok(plus_one(*x)?)),
            Ok((3, 4))
        );
        assert_eq!(
            Ok(3).m_product(|x| Ok(plus_fail(*x)?)),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).m_product(|x| Ok(plus_one(*x)?)),
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
        fn four() -> Result<u8, Error2> {
            Ok(4)
        }
        fn bad() -> Result<u8, Error2> {
            Err(Error2::new("Test"))
        }
        assert_eq!(
            Ok::<u8, Error1>(3).product((|| { Ok(four()?) })()),
            Ok((3, 4))
        );
        assert_eq!(
            Ok::<u8, Error1>(3).product((|| { Ok(bad()?) })()),
            Err(Error1::new("Converted Error2(Test)"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).product((|| { Ok(four()?) })()),
            Err(Error1::new("Test"))
        );
        assert_eq!(
            Err::<u8, Error1>(Error1::new("Test")).product((|| { Ok(bad()?) })()),
            Err(Error1::new("Test"))
        );
    }
}
