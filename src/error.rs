use std::{fmt::Display, num::ParseFloatError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    UnexpectedEndOfText,
    FloatError(#[from] ParseFloatError),
    InvalidStartOfJsonValue(char),
    UnexpectedCharacterInArray(char),
    UnexpectedCharacterInObject(char),
    InvalidJsonValue,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
