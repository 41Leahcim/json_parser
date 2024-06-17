use core::{
    fmt::{self, Display, Formatter},
    num::ParseFloatError,
};

#[derive(Debug)]
#[non_exhaustive]
#[allow(clippy::error_impl_error)]
pub enum Error {
    UnexpectedEndOfText,
    FloatError(ParseFloatError),
    InvalidStartOfJsonValue(char),
    UnexpectedCharacterInArray(char),
    UnexpectedCharacterInObject(char),
    InvalidJsonValue,
    InvalidObjectKey,
    MissingKeyValueSeparator,
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
