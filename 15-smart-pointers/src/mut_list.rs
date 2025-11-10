use std::{cell::{Ref, RefCell}, fmt::Display, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct List<T> {
    pub node: Rc<Node<T>>,
}

impl<T> List<T> {
    pub fn nil() -> List<T> {
        List {
            node: Rc::new(Node::Nil),
        }
    }

    pub fn push(&self, head: T) -> List<T> {
        match *self.node {
            Node::Nil => List {
                node: Rc::new(Node::Cons {
                    head: Rc::new(RefCell::new(head)),
                    tail: List::nil(),
                }),
            },
            _ => List {
                node: Rc::new(Node::Cons {
                    head: Rc::new(RefCell::new(head)),
                    tail: List {
                        node: Rc::clone(&self.node),
                    },
                }),
            },
        }
    }

    pub fn push_cell(&self, head: Rc<RefCell<T>>) -> List<T> {
        match *self.node {
            Node::Nil => List {
                node: Rc::new(Node::Cons {
                    head,
                    tail: List::nil(),
                }),
            },
            _ => List {
                node: Rc::new(Node::Cons {
                    head,
                    tail: List {
                        node: Rc::clone(&self.node),
                    },
                }),
            },
        }
    }

    fn _strong_counts(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut ptr = &self.node;
        loop {
            result.push(Rc::strong_count(ptr));
            match ptr.as_cons() {
                None => break,
                Some((_, tail)) => ptr = &tail.node,
            }
        }
        result
    }
}
impl<T> Display for List<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ptr = &self.node;
        loop {
            match ptr.as_cons() {
                None => break f.write_str("Nil"),
                Some((head, tail)) => {
                    write!(f, "{} :: ", head.borrow())?;
                    ptr = &tail.node
                }
            }
        }
    }
}
impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self {
            node: Rc::clone(&self.node),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Node<T> {
    Nil,
    Cons { head: Rc<RefCell<T>>, tail: List<T> },
}

impl<T> Node<T> {
    pub fn is_nil(&self) -> bool {
        matches!(self, Node::Nil)
    }

    pub fn as_cons(&self) -> Option<(&RefCell<T>, &List<T>)> {
        match self {
            Node::Nil => None,
            Node::Cons { head, tail } => Some((head, tail)),
        }
    }
}

pub struct ListIterator<'a, T>(&'a List<T>);
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.node.as_cons().map(|(head, tail)| {
            self.0 = tail;
            head.borrow()
        })
    }
}
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = Ref<'a, T>;

    type IntoIter = ListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator(self)
    }
}

pub trait CanBeConned<T> {
    fn cons(self, tail: &List<T>) -> List<T>;

    fn nil(self) -> List<T>;
}
impl<T> CanBeConned<T> for Rc<RefCell<T>> {
    fn cons(self, tail: &List<T>) -> List<T> {
        tail.push_cell(self)
    }

    fn nil(self) -> List<T> {
        self.cons(&List::nil())
    }
}
impl<T> CanBeConned<T> for T {
    fn cons(self, tail: &List<T>) -> List<T> {
        tail.push(self)
    }

    fn nil(self) -> List<T> {
        self.cons(&List::nil())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Node::{Cons, Nil};

    #[test]
    fn push() {
        assert_eq!(
            List::<i32>::nil(),
            List {
                node: Rc::new(Node::<i32>::Nil)
            }
        );
        assert_eq!(
            List::nil().push(1).push(2).push(3),
            List {
                node: Rc::new(Cons {
                    head: Rc::new(RefCell::new(3)),
                    tail: List {
                        node: Rc::new(Cons {
                            head: Rc::new(RefCell::new(2)),
                            tail: List {
                                node: Rc::new(Cons {
                                    head: Rc::new(RefCell::new(1)),
                                    tail: List { node: Rc::new(Nil) }
                                })
                            }
                        })
                    }
                })
            }
        );
    }

    #[test]
    fn as_cons() {
        let input: List<i32> = 1.cons(&2.cons(&3.nil()));
        let mut result: Vec<i32> = Vec::new();
        let mut ptr = Rc::clone(&input.node);
        while let Some((head, tail)) = ptr.as_cons() {
            result.push(*head.borrow());
            ptr = Rc::clone(&tail.node);
        }
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn iterator() {
        let input: List<i32> = 1.cons(&2.cons(&3.nil()));
        assert_eq!(
            input.into_iter().map(|x|*x).collect::<Vec<i32>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn structural_stability() {
        let prefix = 2.cons(&1.nil());
        assert_eq!(prefix._strong_counts(), vec![1, 1, 1]);

        let branch1 = 3.cons(&prefix);
        assert_eq!(prefix._strong_counts(), vec![2, 1, 1]);
        assert_eq!(branch1._strong_counts(), vec![1, 2, 1, 1]);

        let branch2 = 4.cons(&prefix);
        assert_eq!(prefix._strong_counts(), vec![3, 1, 1]);
        assert_eq!(branch2._strong_counts(), vec![1, 3, 1, 1]);

        drop(branch1);
        assert_eq!(prefix._strong_counts(), vec![2, 1, 1]);
        assert_eq!(branch2._strong_counts(), vec![1, 2, 1, 1]);

        let branch3 = 5.cons(&branch2);
        assert_eq!(prefix._strong_counts(), vec![2, 1, 1]);
        assert_eq!(branch2._strong_counts(), vec![2, 2, 1, 1]);
        assert_eq!(branch3._strong_counts(), vec![1, 2, 2, 1, 1]);

        drop(branch2);
        assert_eq!(prefix._strong_counts(), vec![2, 1, 1]);
        assert_eq!(branch3._strong_counts(), vec![1, 1, 2, 1, 1]);

        drop(prefix);
        assert_eq!(branch3._strong_counts(), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn this_feels_dirty() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::clone(&value).nil();

        let b = 3.cons(&a);
        let c = 4.cons(&a);

        *value.borrow_mut() += 10;

        assert_eq!(
            a.into_iter().map(|x|*x).collect::<Vec<i32>>(),
            vec![15]
        );
        assert_eq!(
            b.into_iter().map(|x|*x).collect::<Vec<i32>>(),
            vec![3, 15]
        );
        assert_eq!(
            c.into_iter().map(|x|*x).collect::<Vec<i32>>(),
            vec![4, 15]
        );
    }
}
