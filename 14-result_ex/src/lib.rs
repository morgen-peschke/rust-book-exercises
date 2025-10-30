//! Pulling this out to a lib to play with workspaces

pub trait ResultEx<T0, E0> {
    fn flat_map<T1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> Result<T1, E0>;

    fn map<T1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> T1;

    #[inline]
    fn m_product<T1, F>(self, f: F) -> Result<(T0, T1), E0>
    where
        Self: Sized,
        F: FnOnce(&T0) -> Result<T1, E0>,
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
    fn product<T1>(self, t1: T1) -> Result<(T0, T1), E0>
    where
        Self: Sized,
    {
        self.map(|t0| (t0, t1))
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

impl<T0, E0> ResultEx<T0, E0> for Result<T0, E0> {
    #[inline]
    fn flat_map<T1, F>(self, f: F) -> Result<T1, E0>
    where
        Self: Sized,
        F: FnOnce(T0) -> Result<T1, E0>,
    {
        self.and_then(f)
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
