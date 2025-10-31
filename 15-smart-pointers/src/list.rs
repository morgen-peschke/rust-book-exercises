#[derive(Debug, PartialEq)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
impl<T> List<T> {
    pub fn nil() -> List<T> {
        List::Nil
    }
    pub fn push(self, head: T) -> List<T> {
        List::Cons(head, Box::new(self))
    }
    pub fn pop(self) -> Option<(T, List<T>)> {
        match self {
            List::Nil => None,
            List::Cons(head, list) => Some((head, *list)),
        }
    }
}

pub struct ListIterator<'a, T>(&'a List<T>);
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            List::Nil => None,
            List::Cons(t, list) => {
                self.0 = list;
                Some(t)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;

    type IntoIter = ListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator(self)
    }
}

pub trait CanBeConned<T> {
    fn cons(self, tail: List<T>) -> List<T>;

    fn nil(self) -> List<T>;
}
impl<T> CanBeConned<T> for T {
    fn cons(self, tail: List<T>) -> List<T> {
        tail.push(self)
    }

    fn nil(self) -> List<T> {
        self.cons(List::Nil)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use List::{Cons, Nil};

    #[test]
    fn push() {
        assert_eq!(List::<i32>::nil(), Nil);
        assert_eq!(
            List::nil().push(1).push(2).push(3),
            Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil))))))
        );
    }

    #[test]
    fn pop() {
        let mut input: List<i32> = 1.cons(2.cons(3.nil()));
        let mut result: Vec<i32> = Vec::new();
        while let Some((head, tail)) = input.pop() {
            result.push(head);
            input = tail;
        }
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn iterator() {
        let input: List<i32> = 1.cons(2.cons(3.nil()));
        assert_eq!(
            input.into_iter().cloned().collect::<Vec<i32>>(),
            vec![1, 2, 3]
        );
    }
}
