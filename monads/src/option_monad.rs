//! Pulling this out to a lib to play with workspaces
pub trait OptionMonad<T0> {
    fn flat_map<T1, F>(self, f: F) -> Option<T1>
    where
        Self: Sized,
        F: FnOnce(T0) -> Option<T1>;

    fn map<T1, F>(self, f: F) -> Option<T1>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1;

    #[inline]
    fn m_product<T1, F>(self, f: F) -> Option<(T0, T1)>
    where
        Self: Sized,
        F: FnOnce(&T0) -> Option<T1>,
    {
        self.flat_map(|t0| f(&t0).map(|t1| (t0, t1)))
    }

    #[inline]
    fn f_product<T1, F>(self, f: F) -> Option<(T0, T1)>
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
    fn product<T1>(self, other: Option<T1>) -> Option<(T0, T1)>
    where
        Self: Sized,
    {
        self.flat_map(|t0| other.map(|t1| (t0, t1)))
    }

    #[inline]
    fn m_as<T1, F>(self, f: F) -> Option<T1>
    where
        Self: Sized,
        F: FnOnce() -> T1,
    {
        self.map(|_| f())
    }

    #[inline]
    fn void(self) -> Option<()>
    where
        Self: Sized,
    {
        self.m_as(|| ())
    }
}

impl<T0> OptionMonad<T0> for Option<T0> {
    #[inline]
    fn flat_map<T1, F>(self, f: F) -> Option<T1>
    where
        Self: Sized,
        F: FnOnce(T0) -> Option<T1>,
    {
        self.and_then(f)
    }

    #[inline]
    fn map<T1, F>(self, f: F) -> Option<T1>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1,
    {
        self.map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::OptionMonad;

    #[test]
    fn flat_map() {
        assert_eq!(Some(4).flat_map(|x| Some(x + 4)), Some(8));
        assert_eq!(Some(4).flat_map(|_| None::<u8>), None);
        assert_eq!(None.flat_map(|x: u8| Some(x + 4)), None);
    }

    #[test]
    fn m_product() {
        assert_eq!(Some(3).m_product(|x| Some(x + 1)), Some((3, 4)));
        assert_eq!(Some(3).m_product(|_| None::<u8>), None);
        assert_eq!(
            // This fails
            // None.m_product(|x: u8| Some(x + 4)),
            None::<u8>.m_product(|x| Some(x + 1)),
            None
        );
    }

    #[test]
    fn f_product() {
        assert_eq!(Some(3).f_product(|x| x + 1), Some((3, 4)));
        assert_eq!(
            // This fails
            // None.f_product(|x: u8| x + 4),
            None::<u8>.f_product(|x| x + 1),
            None
        );
    }

    #[test]
    fn product() {
        assert_eq!(Some(3).product(Some(4)), Some((3, 4)));
        assert_eq!(Some(3).product(None::<u8>), None);
        assert_eq!(None::<u8>.product(Some(4)), None);
    }
}
