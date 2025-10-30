use super::secret::{self, Secret};
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
pub enum OutOfBounds {
    TooSmall { min: i32 },
    TooBig { max: i32 },
}

pub enum IsSecret {
    Yes,
    Less,
    Greater,
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Result<Guess, OutOfBounds> {
        if value < secret::MIN {
            Result::Err(OutOfBounds::TooSmall { min: secret::MIN })
        } else if value > secret::MAX {
            Result::Err(OutOfBounds::TooBig { max: secret::MAX })
        } else {
            Result::Ok(Guess { value })
        }
    }

    pub fn is(&self, s: &Secret) -> IsSecret {
        match self.value.cmp(&s.value()) {
            Ordering::Equal => IsSecret::Yes,
            Ordering::Greater => IsSecret::Greater,
            Ordering::Less => IsSecret::Less,
        }
    }
}
impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        i32::fmt(&self.value, f)
    }
}
