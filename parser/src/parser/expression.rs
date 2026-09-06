use std::{
    fmt::{Debug, Display, Write},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::common;
use crate::lexer::token::Tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Group(common::Group),
    Negate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Divide,
    Multiply,
}
impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            BinaryOp::Plus => '+',
            BinaryOp::Minus => '-',
            BinaryOp::Divide => '/',
            BinaryOp::Multiply => '*',
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
    Number {
        value: u64,
    },
    Unary {
        op: UnaryOp,
        child: Box<Expression>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}
impl Expression {
    pub fn bracket_with(&self, group: common::Group) -> Expression {
        Expression::Unary {
            op: UnaryOp::Group(group),
            child: Box::new(self.clone()),
        }
    }

    pub(super) fn in_parens(&self) -> Expression {
        self.bracket_with(common::Group::Paren)
    }

    /// Insert explicit params
    ///
    /// This _will_ insert more params than needed, but it's easier to be dumb
    /// about this and follow it with a call to `normalize_nesting`
    pub(super) fn with_explicit_parens(self) -> Expression {
        match self {
            Expression::Number { .. } => self,
            Expression::Unary { op, child } => match op {
                UnaryOp::Group(..) => Expression::Unary {
                    op,
                    child: Box::new(child.with_explicit_parens()),
                },
                UnaryOp::Negate => Expression::Unary {
                    op,
                    child: Box::new(child.with_explicit_parens().in_parens()),
                },
            },
            Expression::Binary { op, lhs, rhs } => Expression::Binary {
                op,
                lhs: Box::new(lhs.with_explicit_parens().in_parens()),
                rhs: Box::new(rhs.with_explicit_parens().in_parens()),
            },
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            Expression::Number { .. } => true,
            Expression::Unary { .. } => true,
            Expression::Binary { .. } => false,
        }
    }

    /// Minimize nesting
    ///
    /// Removes duplicate nesting, as well as situations where we know the
    /// nesting isn't needed (like around a number)
    pub(super) fn minimize_nesting(self) -> Expression {
        match self {
            Expression::Number { .. } => self,
            Expression::Unary {
                op: op @ UnaryOp::Group(_),
                child,
            } => {
                let child = child.minimize_nesting();
                if child.is_terminal() {
                    child
                } else {
                    Expression::Unary {
                        op,
                        child: Box::new(child),
                    }
                }
            }
            Expression::Unary { op, child } => Expression::Unary {
                op,
                child: Box::new(child.minimize_nesting()),
            },
            Expression::Binary { op, lhs, rhs } => Expression::Binary {
                op,
                lhs: Box::new(lhs.minimize_nesting()),
                rhs: Box::new(rhs.minimize_nesting()),
            },
        }
    }

    /// Normalize which group is used, and when
    ///
    /// Flips the grouping character at each level, because `([{1}])` is easier
    /// to read than `(((1)))`
    fn normalize_group_choice(self, last_group: common::Group) -> Expression {
        match self {
            Expression::Number { .. } => self,
            Expression::Unary { op, child } => match op {
                UnaryOp::Group(_) => Expression::Unary {
                    op: UnaryOp::Group(last_group),
                    child: Box::new(child.normalize_group_choice(last_group.next())),
                },
                UnaryOp::Negate => Expression::Unary {
                    op,
                    child: Box::new(child.normalize_group_choice(last_group)),
                },
            },
            Expression::Binary { op, lhs, rhs } => Expression::Binary {
                op,
                lhs: Box::new(lhs.normalize_group_choice(last_group)),
                rhs: Box::new(rhs.normalize_group_choice(last_group)),
            },
        }
    }

    /// Normalize the expression
    ///
    /// Makes the grouping explicit, minimal, and easy to read.
    pub fn normalized(self) -> Expression {
        self.with_explicit_parens()
            .minimize_nesting()
            .normalize_group_choice(common::Group::Paren)
    }

    /// Turn the expression back into tokens, just for fun.
    ///
    /// It's usually best to call `normalized` on it first.
    pub fn unparse(&self) -> Tokens {
        use crate::lexer::token::Token;

        match self {
            Expression::Number { value } => Tokens(vec![Token::Number {
                value: *value,
                start: 0,
            }]),
            Expression::Unary { op, child } => match op {
                UnaryOp::Group(group) => {
                    Tokens::one(Token::OpenGroup {
                        group: *group,
                        start: 0,
                    }) + child.unparse()
                        + Tokens::one(Token::CloseGroup {
                            group: *group,
                            start: 0,
                        })
                }
                UnaryOp::Negate => Tokens::one(Token::Minus { start: 0 }) + child.unparse(),
            },
            Expression::Binary { op, lhs, rhs } => {
                let op_token = match op {
                    BinaryOp::Plus => Token::Plus { start: 0 },
                    BinaryOp::Minus => Token::Minus { start: 0 },
                    BinaryOp::Divide => Token::Divide { start: 0 },
                    BinaryOp::Multiply => Token::Multiply { start: 0 },
                };
                lhs.unparse() + Tokens::one(op_token) + rhs.unparse()
            }
        }
    }
}
impl Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        Expression::Unary {
            op: UnaryOp::Negate,
            child: Box::new(self),
        }
    }
}
impl Neg for &Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        -(self.clone())
    }
}
impl Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Binary {
            op: BinaryOp::Plus,
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}
impl Add for &Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}
impl Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::Binary {
            op: BinaryOp::Minus,
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}
impl Sub for &Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}
impl Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::Binary {
            op: BinaryOp::Multiply,
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}
impl Mul for &Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}
impl Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Binary {
            op: BinaryOp::Divide,
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}
impl Div for &Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        self.clone() / rhs.clone()
    }
}

impl From<u64> for Expression {
    fn from(value: u64) -> Self {
        Expression::Number { value }
    }
}
/// Pretty printing is the default display
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number { value } => write!(f, "{value}"),
            Expression::Unary { op, child } => match op {
                UnaryOp::Group(group) => {
                    write!(f, "{}{}{}", group.open(), child, group.close())
                }
                UnaryOp::Negate => match **child {
                    Expression::Number { value } => write!(f, "-{value}"),
                    _ => write!(f, "-({})", child),
                },
            },
            Expression::Binary { op, lhs, rhs } => {
                write!(f, "{} {} {}", lhs, op, rhs)
            }
        }
    }
}
/// Debug display is a lispy representation that makes it easier to see the
/// structure, which is invaluable for tests.
impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Expression::Number { value } => write!(f, "{value}"),
            Expression::Unary { op, child } => match op {
                UnaryOp::Group(group) => {
                    write!(f, "(grp '{}{}' {:?})", group.open(), group.close(), child)
                }
                UnaryOp::Negate => {
                    write!(f, "(neg {child:?})")
                }
            },
            Expression::Binary { op, lhs, rhs } => {
                write!(f, "({op} {lhs:?} {rhs:?})")
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::common::group::test::group_strategy;
    use proptest::prelude::*;

    fn unary_op_strategy() -> impl Strategy<Value = UnaryOp> {
        prop_oneof![
            group_strategy().prop_map(UnaryOp::Group),
            Just(UnaryOp::Negate)
        ]
    }

    fn binary_op_strategy() -> impl Strategy<Value = BinaryOp> {
        prop_oneof![
            Just(BinaryOp::Plus),
            Just(BinaryOp::Minus),
            Just(BinaryOp::Divide),
            Just(BinaryOp::Multiply),
        ]
    }

    pub(crate) fn expression_strategy() -> impl Strategy<Value = Expression> {
        let term = any::<u64>().prop_map(|value| Expression::Number { value });
        term.prop_recursive(8, 256, 3, |recurse| {
            prop_oneof![
                (unary_op_strategy(), recurse.clone()).prop_map(|(op, child)| Expression::Unary {
                    op,
                    child: Box::new(child)
                }),
                (recurse.clone(), binary_op_strategy(), recurse.clone()).prop_map(
                    |(lhs, op, rhs)| Expression::Binary {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs)
                    }
                ),
            ]
        })
    }
}
