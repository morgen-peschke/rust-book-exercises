use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum List<T> {
    Nil,
    ConsRef(Rc<Cons<T>>),
}

#[derive(Debug, PartialEq)]
pub struct Cons<T> {
    head: T,
    tail: Rc<List<T>>,
}

impl<T> List<T> {
    pub fn nil() -> List<T> {
        List::Nil
    }
    pub fn push(self, head: T) -> List<T> {
        List::ConsRef(match self {
            List::Nil => Rc::new(Cons {
                head,
                tail: Rc::new(self),
            }),
            List::ConsRef(cons) => Rc::new(Cons {
                head,
                tail: Rc::new(List::ConsRef(Rc::clone(&cons))),
            }),
        })
    }
    pub fn pop(&self) -> Option<Rc<Cons<T>>> {
        match &self {
            List::Nil => None,
            List::ConsRef(cons) => Some(Rc::clone(cons)),
        }
    }
}

pub struct ListIterator<'a, T>(&'a List<T>);
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            List::Nil => None,
            List::ConsRef(cons) => {
                self.0 = &cons.tail;
                Some(&cons.head)
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
    use List::{ConsRef, Nil};

    #[test]
    fn push() {
        assert_eq!(List::<i32>::nil(), Nil);
        assert_eq!(
            List::nil().push(1).push(2).push(3),
            ConsRef(Rc::new(Cons {
                head: 3,
                tail: Rc::new(ConsRef(Rc::new(Cons {
                    head: 2,
                    tail: Rc::new(ConsRef(Rc::new(Cons {
                        head: 1,
                        tail: Rc::new(Nil)
                    })))
                })))
            }))
        );
    }

    #[test]
    fn pop() {
        let input: List<i32> = 1.cons(2.cons(3.nil()));
        let mut result: Vec<i32> = Vec::new();
        let mut ptr = Rc::new(input);
        while let Some(cons) = ptr.pop() {
            result.push(cons.head);
            ptr = Rc::clone(&cons.tail);
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
