use std::ops::Deref;

use semigroup::Semigroup;

pub struct Resource<T, E, Init, Close>
where
    Init: Fn() -> Result<T, E>,
    Close: Fn(&T) -> Result<(), E>,
{
    init: Init,
    close: Close,
}

struct Managed<'a, T, E, Close>
where
    Close: Fn(&'a T) -> Result<(), E>,
    T: 'a,
{
    value: &'a T,
    close: &'a Close,
    result: &'a mut Result<(), E>,
}

impl<'a, T, E, Close> Deref for Managed<'a, T, E, Close>
where
    Close: Fn(&'a T) -> Result<(), E>,
    T: 'a,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}
impl<'a, T, E, Close> Drop for Managed<'a, T, E, Close>
where
    Close: Fn(&'a T) -> Result<(), E>,
    T: 'a,
{
    fn drop(&mut self) {
        *self.result = (self.close)(self.value);
    }
}

impl<T, E, Init, Close> Resource<T, E, Init, Close>
where
    Init: Fn() -> Result<T, E>,
    Close: Fn(&T) -> Result<(), E>,
    E: Semigroup,
{
    pub fn new(init: Init, close: Close) -> Resource<T, E, Init, Close> {
        Resource { init, close }
    }
}

impl<'a, T, E, Init, Close> Resource<T, E, Init, Close>
where
    Init: Fn() -> Result<T, E>,
    Close: Fn(&T) -> Result<(), E>,
    E: Semigroup,
{
    pub fn use_value<F, T2>(&'a self, f: F) -> Result<T2, E>
    where
        F: FnOnce(&T) -> Result<T2, E>,
    {
        match (self.init)() {
            Ok(value) => {
                let mut close_result: Result<(), E> = Ok(());
                let managed: Managed<'_, T, E, Close> = Managed {
                    value: &value,
                    close: &self.close,
                    result: &mut close_result,
                };
                let f_result = f(&managed);
                drop(managed);
                match (f_result, close_result) {
                    (Ok(v), Ok(())) => Ok(v),
                    (Ok(_), Err(e)) => Err(e),
                    (Err(e), Ok(())) => Err(e),
                    (Err(e0), Err(e1)) => Err(e0.combine(e1)),
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn lazy() {
        let _ = Resource::new(
            || -> Result<(), String> {
                panic!("Init should not have run");
            },
            |_| -> Result<(), String> {
                panic!("Close should not have run");
            },
        );
    }

    #[test]
    fn returns_value() {
        let resource = Resource::new(|| Ok::<i32, String>(5), |_: &i32| Ok(()));
        assert_eq!(resource.use_value(|i| Ok(i + 1)), Ok(6));
        assert_eq!(resource.use_value(|i| Ok(i + 2)), Ok(7));
    }

    #[test]
    fn init_failures_short_circuit() {
        let resource = Resource::new(|| Err("Failure to launch".to_owned()), |_: &i32| Ok(()));
        assert_eq!(
            resource.use_value(|_| -> Result<i32, String> { panic!() }),
            Err("Failure to launch".to_owned())
        );
    }

    #[test]
    fn returns_use_and_close_errors() {
        let resource = Resource::new(|| Ok(()), |_| Err("|close ran as expected".to_owned()));
        assert_eq!(
            resource.use_value(|_| Ok(5)),
            Err("|close ran as expected".to_owned())
        );
        assert_eq!(
            resource.use_value(|_| -> Result<(), String> { Err("use error returned".to_owned()) }),
            Err("use error returned|close ran as expected".to_owned())
        );
    }

    struct Tester {
        value: i32,
        log: Cell<Vec<String>>,
    }
    impl Tester {
        fn record(&self, l: &str) {
            let mut log = self.log.take();
            log.push(l.to_owned());
            self.log.set(log)
        }
    }
    #[test]
    fn order_check() {
        fn init() -> Result<Tester, String> {
            Ok(Tester {
                value: 0,
                log: Cell::new(vec!["Opened(0)".to_owned()]),
            })
        }
        fn close(tester: &Tester) -> Result<(), String> {
            tester.record(&format!("Closed({})", &tester.value));
            Err(tester.log.take().join(" -- "))
        }
        let resource = Resource::new(init, close);
        assert_eq!(
            resource.use_value(|t| {
                t.record("using value");
                Ok(())
            }),
            Err("Opened(0) -- using value -- Closed(0)".to_owned())
        )
    }
}
