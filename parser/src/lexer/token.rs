use std::{fmt::Display, ops::Add};

use crate::common::Group;

#[derive(Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<Token>);
impl Tokens {
    pub fn one(token: Token) -> Tokens {
        Tokens(vec![token])
    }

    #[cfg(test)]
    pub(crate) fn with_zero_starts(self) -> Tokens {
        Tokens(self.0.iter().map(|t| t.with_zero_start()).collect())
    }
}
impl Add for Tokens {
    type Output = Tokens;

    fn add(self, rhs: Self) -> Self::Output {
        Tokens(self.0.into_iter().chain(rhs.0).collect())
    }
}
impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut itr = self.0.iter();
        if let Some(t) = itr.next() {
            write!(f, "{t}")?
        }
        for t in itr {
            write!(f, ",{t}")?
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token {
    OpenGroup { group: Group, start: usize },
    CloseGroup { group: Group, start: usize },
    Plus { start: usize },
    Minus { start: usize },
    Divide { start: usize },
    Multiply { start: usize },
    Number { value: u64, start: usize },
}
impl Token {
    pub fn start(&self) -> &usize {
        match self {
            Token::OpenGroup { group: _, start } => start,
            Token::CloseGroup { group: _, start } => start,
            Token::Plus { start } => start,
            Token::Minus { start } => start,
            Token::Divide { start } => start,
            Token::Multiply { start } => start,
            Token::Number { value: _, start } => start,
        }
    }

    pub fn equivalent_to(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::OpenGroup { group: l_group, .. }, Token::OpenGroup { group: r_group, .. }) => {
                l_group == r_group
            }
            (
                Token::CloseGroup { group: l_group, .. },
                Token::CloseGroup { group: r_group, .. },
            ) => l_group == r_group,
            (Token::Plus { .. }, Token::Plus { .. }) => true,
            (Token::Minus { .. }, Token::Minus { .. }) => true,
            (Token::Divide { .. }, Token::Divide { .. }) => true,
            (Token::Multiply { .. }, Token::Multiply { .. }) => true,
            (Token::Number { value: l_value, .. }, Token::Number { value: r_value, .. }) => {
                l_value == r_value
            }
            _ => false,
        }
    }

    #[cfg(test)]
    pub fn with_zero_start(self) -> Token {
        match self {
            Token::OpenGroup { group, .. } => Token::OpenGroup { group, start: 0 },
            Token::CloseGroup { group, .. } => Token::CloseGroup { group, start: 0 },
            Token::Plus { .. } => Token::Plus { start: 0 },
            Token::Minus { .. } => Token::Minus { start: 0 },
            Token::Divide { .. } => Token::Divide { start: 0 },
            Token::Multiply { .. } => Token::Multiply { start: 0 },
            Token::Number { value, .. } => Token::Number { value, start: 0 },
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::OpenGroup { group, .. } => group.open().to_string(),
                Token::CloseGroup { group, .. } => group.close().to_string(),
                Token::Plus { .. } => "+".to_string(),
                Token::Minus { .. } => "-".to_string(),
                Token::Divide { .. } => "/".to_string(),
                Token::Multiply { .. } => "*".to_string(),
                Token::Number { value, .. } => format!("{value}"),
            }
        )
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::common::group::test::group_strategy;
    use proptest::prelude::*;

    pub(crate) fn token_strategy() -> impl Strategy<Value = Token> {
        prop_oneof![
            (group_strategy(), any::<usize>())
                .prop_map(|(group, start)| Token::OpenGroup { group, start }),
            (group_strategy(), any::<usize>())
                .prop_map(|(group, start)| Token::CloseGroup { group, start }),
            any::<usize>().prop_map(|start| Token::Plus { start }),
            any::<usize>().prop_map(|start| Token::Minus { start }),
            any::<usize>().prop_map(|start| Token::Divide { start }),
            any::<usize>().prop_map(|start| Token::Multiply { start }),
            (any::<u64>(), any::<usize>())
                .prop_map(|(value, start)| Token::Number { value, start }),
        ]
    }

    pub(crate) fn tokens_strategy() -> impl Strategy<Value = Tokens> {
        prop::collection::vec(token_strategy(), 0..=100).prop_map(Tokens)
    }
}
