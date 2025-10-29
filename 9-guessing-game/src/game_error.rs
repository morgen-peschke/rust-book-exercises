use super::guess::OutOfBounds;
use super::prompt::Stop;
use std::{fmt::Display, io, num::ParseIntError, process::ExitCode};

pub enum GameError {
    Quit,
    ReadFail(io::Error),
    ParseFail(ParseIntError),
    InvalidGuess(OutOfBounds),
}
impl GameError {
    pub fn exit_code(&self) -> Option<ExitCode> {
        match self {
            GameError::Quit => Some(ExitCode::SUCCESS),
            GameError::ReadFail(_) => Some(ExitCode::from(1)),
            GameError::ParseFail(_) => None,
            GameError::InvalidGuess(_) => None,
        }
    }
}
impl From<Stop> for GameError {
    fn from(value: Stop) -> Self {
        match value {
            Stop::Quit => Self::Quit,
            Stop::Failure(e) => Self::ReadFail(e),
        }
    }
}
impl From<ParseIntError> for GameError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseFail(value)
    }
}
impl From<OutOfBounds> for GameError {
    fn from(value: OutOfBounds) -> Self {
        Self::InvalidGuess(value)
    }
}
impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::Quit => f.write_str("Goodbye"),
            GameError::ReadFail(e) => {
                write!(f, "Failed to read line: {e}")
            }
            GameError::ParseFail(e) => {
                write!(f, "Not a number ({e}), try again.")
            }
            GameError::InvalidGuess(OutOfBounds::TooSmall { min }) => {
                write!(f, "Out of bounds (must be at least {min})")
            }
            GameError::InvalidGuess(OutOfBounds::TooBig { max }) => {
                write!(f, "Out of bounds (must be no more than {max})")
            }
        }
    }
}
