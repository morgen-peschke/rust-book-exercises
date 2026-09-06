use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Group {
    Paren,
    Bracket,
    Brace,
}
impl Group {
    pub fn open(&self) -> &str {
        match self {
            Group::Paren => "(",
            Group::Bracket => "[",
            Group::Brace => "{",
        }
    }
    pub fn close(&self) -> &str {
        match self {
            Group::Paren => ")",
            Group::Bracket => "]",
            Group::Brace => "}",
        }
    }
    pub fn next(&self) -> Group {
        match self {
            Group::Paren => Group::Bracket,
            Group::Bracket => Group::Brace,
            Group::Brace => Group::Paren,
        }
    }
}
impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.open(), self.close())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use proptest::prelude::*;
    pub(crate) fn group_strategy() -> impl Strategy<Value = Group> {
        prop_oneof![Just(Group::Paren), Just(Group::Brace), Just(Group::Bracket),]
    }
}
